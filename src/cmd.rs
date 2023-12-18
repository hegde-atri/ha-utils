use std::{
    io::{Error, ErrorKind},
    path::{Path, PathBuf},
    process::{Command, Output},
};

use which::which;

/// Returns output to give command
///
/// It takes in the command/binary to execute, optional flags and the path
/// to execute it in.
///
/// It will only execute if the provided `cmd` is found on host and `cwd` exists
/// as a directory.
///
/// # Arguments
///
/// * `cmd` - The binary to execute.
/// * `cwd` - The directory to execute in.
///
/// # Examples
/// ```rust
/// exec("hyprctl workspaces", Path::new("/home/user/repo"));
/// ```
/// You can use it with `get_pwd()` to avoid passing a path.
///
/// ```rust
/// exec("hyprctl workspaces", None);
/// ```
///
pub fn exec(cmd: &str, cwd: &Path) -> Result<Output, Error> {
    // Check if binary for command exists
    match which(cmd.split(' ').next().unwrap()) {
        Ok(_) => (),
        Err(_) => {
            return Err(Error::new(
                ErrorKind::NotFound,
                format!("Could not find specified command: {}", cmd),
            ))
        }
    }
    // Check if path exists
    if !cwd.exists() {
        return Err(Error::new(ErrorKind::Other, "Specified path is invalid!"));
    }
    // path is not a directory
    if !cwd.is_dir() {
        return Err(Error::new(
            ErrorKind::Other,
            "Specified path is not a directory",
        ));
    }
    // Now execute the command
    if cfg!(target_os = "windows") {
        return Command::new("cmd")
            .current_dir(&cwd.as_os_str())
            .args(["/C", cmd])
            .output();
    } else {
        return Command::new("sh")
            .current_dir(&cwd.as_os_str())
            .arg("-c")
            .arg(cmd)
            .output();
    };
}

/// Returns a Pathbuf of current working dir or the dir if provided.
///
/// It's to be used with `exec()`
///
/// ## Example
///
/// ```rust
/// get_pwd(None);
/// ```
pub fn get_pwd(dir: Option<&Path>) -> PathBuf {
    let pwd = match std::env::current_dir() {
        Ok(v) => PathBuf::from(v),
        Err(err) => panic!("Couldn't find current dir: {}", err),
    };

    return match dir {
        Some(v) => v.to_path_buf(),
        None => pwd,
    };
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::exec;

    #[test]
    fn test_exec_valid() {
        let cmd = "echo";
        let cwd = Path::new("/tmp");
        let result = exec(cmd, cwd);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.status.success());
    }

    #[test]
    #[should_panic]
    fn test_exec_cmd_invalid() {
        let cmd = "invalid_command";
        let cwd = Path::new("/tmp");
        exec(cmd, cwd).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_exec_cwd_invalid() {
        let cmd = "ls";
        let cwd = Path::new("/dir/that/does/not/exist");
        exec(cmd, cwd).unwrap();
    }
}
