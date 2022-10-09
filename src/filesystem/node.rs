use std::path::PathBuf;
use crate::commands::NodeVersion;
use super::Error;

/// This function will create the directory for specific version of node.js
/// and put all executable files into it.
/// 
/// # Arguments:
/// 
/// * `data` - Data of the node version we want to install
/// * `cnvmpath` - Path to the cnvm folder
/// * `node_bytes` - Bytes of the node executable
/// * `npm_bytes` - Bytes of the npm executable 
pub fn create_node(
    data: NodeVersion,
    path: &PathBuf, 
    node_bytes: &Vec<u8>,
    npm_bytes: &Vec<u8>
) -> Result<(), Error> {
    let node_path = path.join(data.version);
    
    std::fs::create_dir_all(&node_path).map_err(|err| {
        Error::PermissionError(Some(err.to_string()))
    })?;
    
    std::fs::write(&node_path.join("node.exe"), node_bytes).map_err(|err| {
        Error::PermissionError(Some(err.to_string()))
    })?;

    std::fs::write(&node_path.join("npm"), npm_bytes).map_err(|err| {
        Error::PermissionError(Some(err.to_string()))
    })?;

    Ok(())
}

/// Symlink the directory from cnvm directory to the path 
/// where the used version of node.js will be located
///
/// # Arguments:
/// 
/// * `cnvmpath` - Path to the cnvm folder
/// * `nodepath` - Path to the node folder
/// 
/// # Returns:
/// 
/// * `Result<(), Error>` - Result of the function
#[cfg(target_os = "windows")]
pub fn symlink_node(cnvmpath: &PathBuf, nodepath: &PathBuf) -> Result<(), Error> {
    std::os::windows::fs::symlink_dir(cnvmpath, nodepath).map_err(|err| {
        Error::PermissionError(Some(err.to_string()))
    })?;

    Ok(())
}

/// Remove the symlinked directory from the path where the used 
/// version of node.js is located
/// 
/// # Arguments:
/// 
/// * `cnvmpath` - Path to the cnvm folder
/// * `nodepath` - Path to the node folder
/// 
/// # Returns:
/// 
/// * `Result<(), Error>` - Result of the function
pub fn remove_symlink(cnvmpath: &PathBuf, nodepath: &PathBuf) -> Result<(), Error> {
    std::fs::remove_dir_all(nodepath).map_err(|err| {
        Error::PermissionError(Some(err.to_string()))
    })?;

    Ok(())
}

/// Remove the node version from the cnvm directory
/// 
/// # Arguments:
/// 
/// * `nodeversion` - Version of node.js to remove
/// * `cnvmpath` - Path to the cnvm folder
/// 
/// # Returns:
/// 
/// * `Result<(), Error>` - Result of the function
pub fn remove_node(nodeversion: &str, cnvmpath: &PathBuf) -> Result<(), Error> {
    let node_path = cnvmpath.join(nodeversion);
    
    std::fs::remove_dir_all(&node_path).map_err(|err| {
        Error::PermissionError(Some(err.to_string()))
    })?;

    Ok(())
}