use std::env::consts::ARCH;
use std::process::Command;
use super::Error;

/// These four functions are used to determine the names and
/// extesions for the source node version
#[cfg(target_os = "windows")]
pub fn node_path(version: String) -> String {
    return match ARCH {
        "x86" => format!("node-{}-win-x86", version),
        "x86_64" => format!("node-{}-win-x64", version),
        _ => panic!("Invalid CPU architecture")
    }
}

#[cfg(target_os = "linux")]
pub fn node_path(version: String) -> String {
    return format!(
        "node-{}-linux-{}",
        version,
        ARCH.to_string().replace("x86_64", "x64")
    );
}

#[cfg(target_os = "windows")]
pub fn node_ext() -> String {
    return ".zip".to_string()
}

#[cfg(target_os = "linux")]
pub fn node_ext() -> String {
    return ".tar.gz".to_string()
}

/// Create path to the symlinked directory containing symlinked 
/// node files, this is done with command and is unique for each OS
/// 
/// This function is not used on it's own, but is used in the
/// check_path function
/// 
/// # Returns:
/// 
/// * `Result<(), Error>` - Can return error containing the error message
///                         if adding path fails
#[cfg(target_os = "windows")]
fn export_path() -> Result<(), Error> {
    Command::new("cmd")
        .args(&["/C", "setx", "PATH", "%PATH%;%USERPROFILE%\\.cnvm\\nodeary"])
        .spawn()
        .map_err(|err| {
            Error::PathError(
                Some(err.to_string())
            )
        })?;

    Ok(())
}

#[cfg(target_os = "linux")]
fn export_path() -> Result<(), Error> {
    Command::new("bash")
        .args(&["-c", "echo 'export PATH=$PATH:$HOME/.cnvm/node/binary' >> ~/.bashrc"])
        .spawn()
        .map_err(|err| {
            Error::PathError(
                Some(err.to_string())
            )
        })?;

    Ok(())
}

/// Check if specific directory is in path and if not, add it
///
/// # Returns:
/// 
/// * `Result<(), Error>` - Can return error containing the error message
#[cfg(target_os = "windows")]
pub fn check_path() -> Result<(), Error> {
    let path = std::env::var("PATH").unwrap();
    let cnvm_path = std::env::var("USERPROFILE").unwrap() + "\\.cnvm\\nodeary";

    if !path.contains(&cnvm_path) {
        export_path()?;
    }

    Ok(())
}

#[cfg(target_os = "linux")]
pub fn check_path() -> Result<(), Error> {
    let path = std::env::var("PATH").unwrap();
    let cnvm_path = std::env::var("HOME").unwrap() + "/.cnvm/node\\binary";

    if !path.contains(&cnvm_path) {
        export_path()?;
    }

    Ok(())
}