use std::path::PathBuf;
use super::paths::node_path;
use super::Error;

/// This function will download the compressed node.js zip file
/// and extract all of it's content to the specified path
/// 
/// # Arguments:
/// 
/// * `data` - Data of the node version we want to install
/// * `cnvmpath` - Path to the cnvm folder
/// * `node_bytes` - Bytes of the node executable
pub fn create_node(
    path: &PathBuf, 
    node_bytes: &Vec<u8>
) -> Result<(), Error> {
    // Create a zip file from the node bytes
    let mut node_zip = zip::ZipArchive::new(
        std::io::Cursor::new(node_bytes)
    ).unwrap();

    // Create the directory for the node version
    std::fs::create_dir_all(&path).map_err(|err| {
        Error::PermissionError(Some(err.to_string()))
    })?;

    // Extract the node executable from the zip file
    node_zip.extract(path).unwrap();

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
pub fn symlink_node(cnvmpath: &PathBuf, version: String, nodepath: &PathBuf) -> Result<(), Error> {
    let path = cnvmpath.join(node_path(version));

    std::os::windows::fs::symlink_dir(path, nodepath).map_err(|err| {
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
pub fn remove_symlink(nodepath: &PathBuf) -> Result<(), Error> {
    if nodepath.is_symlink() {
        std::fs::remove_dir_all(nodepath).map_err(|err| {
            Error::PermissionError(Some(err.to_string()))
        })?;
    }

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