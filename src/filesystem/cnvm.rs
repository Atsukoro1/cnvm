use std::path::PathBuf;
use crate::commands::NodeVersion;

use super::Error;

/// This function will create config file if it does not exist
/// 
/// # Arguments:
/// 
/// * `configpath` - Path to the json config file
/// 
/// # Returns:
/// 
/// * `Result<(), Error>` - Result of the function
pub fn config_file_init(path: &PathBuf) -> Result<(), Error> {
    if !path.exists() {
        std::fs::create_dir_all(&path).map_err(|err| {
            Error::PermissionError(Some(err.to_string()))
        })?;
    };

    Ok(())
}

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