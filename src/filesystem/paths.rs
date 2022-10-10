use super::Error;

/// These four functions are used to determine the names and
/// extesions for the source node version
#[cfg(target_os = "windows")]
pub fn node_path(version: String) -> String {
    return match ARCH {
        "x86" => format!("node-{}-x86", version),
        "x86_64" => format!("node-{}-x64", version),
        _ => panic!("Invalid CPU architecture")
    }
}

#[cfg(target_os = "linux")]
pub fn node_path(version: String) -> String {
    use std::env::consts::ARCH;

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
pub fn export_path() -> Result<(), Error> {
    use std::process::Command;
    use std::env::consts::OS;

    match OS {
        "windows" => {
            Command::new("cmd")
                .args(&["/C", "setx", "PATH", "%PATH%;%USERPROFILE%\\.cnvm\\node"])
                .spawn()
                .map_err(|err| {
                    Error::PathError(
                        Some(err.to_string())
                    )
                })?;
        }
        
        "linux" => {
            Command::new("bash")
                .args(&["-c", "echo 'export PATH=$PATH:$HOME/.cnvm/node' >> ~/.bashrc"])
                .spawn()
                .map_err(|err| {
                    Error::PathError(
                        Some(err.to_string())
                    )
                })?;
        }

        _ => panic!("Unsupported OS")
    }

    Ok(())
}