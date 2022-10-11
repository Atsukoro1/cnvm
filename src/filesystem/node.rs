use std::path::PathBuf;
use super::paths::node_path;
use super::Error;
use std::io::Cursor;
use flate2::read::GzDecoder;

/// This function will download the compressed node.js zip file
/// and extract all of it's content to the specified path
/// 
/// # Arguments:
/// 
/// * `data` - Data of the node version we want to install
/// * `cnvmpath` - Path to the cnvm folder
/// * `node_bytes` - Bytes of the node executable
#[cfg(target_os = "windows")]
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

/// For UN*X systems, everything is encoded in tar.gz instead of zip
/// 
/// This function will download the compressed node.js zip file using
/// gunzip so we can extract all of it's content and extract all the 
/// data from tar file to specified path
/// 
/// # Arguments:
/// 
/// * `path` - Path to the desired node.js directory
/// * `node_bytes` - Bytes of the compressed tarball containing all files
///                  for NodeJS to work correctly
#[cfg(target_os = "linux")]
pub fn create_node(
    path: &PathBuf,
    node_bytes: &Vec<u8>
) -> Result<(), Error> {
    // Decompress the gzip file
    let node_gzip: GzDecoder<Cursor<&Vec<u8>>> = flate2::read::GzDecoder::new(
        std::io::Cursor::new(node_bytes)
    );

    // Create the directory for the node version
    std::fs::create_dir_all(&path).map_err(|err| {
        Error::PermissionError(Some(err.to_string()))
    })?;

    // Unpack tarball and return error if it fails
    tar::Archive::new(node_gzip)
        .unpack(&path)
        .map_err(|e| {
            return Error::TarUnpackFailed(
                Some(e.to_string())
            );
        })
        .unwrap();

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

#[cfg(target_os = "linux")]
pub fn symlink_node(cnvmpath: &PathBuf, version: String, nodepath: &PathBuf) -> Result<(), Error> {
    let path = cnvmpath.join(node_path(version));

    std::os::unix::fs::symlink(path, nodepath).map_err(|err| {
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
pub fn remove_node(version: &str, cnvmpath: &PathBuf) -> Result<(), Error> {
    let path = cnvmpath.join(node_path(version.to_owned()));
    
    std::fs::remove_dir_all(path).map_err(|err| {
        Error::PermissionError(Some(err.to_string()))
    })?;

    Ok(())
}

/// Get the lastest version of node.js installed in the cnvm directory
/// 
/// # Arguments:
/// 
/// * `cnvmpath` - Path to the cnvm folder
/// 
/// # Returns:
/// 
/// * `Result<String, Error>` - Result of the function
/// 
/// # Example:
/// 
/// ```rust
/// use cnvm::filesystem::node::get_lastest_node;
/// use std::path::PathBuf;
/// 
/// let cnvmpath = PathBuf::from("C:\\Users\\user\\cnvm");
/// let lastest_node = get_lastest_node(&cnvmpath).unwrap();
/// 
/// assert_eq!(lastest_node, "v14.15.4");
/// ```
pub fn get_latest_version(cnvmpath: &PathBuf) -> Result<String, Error> {
    let mut versions = std::fs::read_dir(cnvmpath).map_err(|err| {
        Error::PermissionError(Some(err.to_string()))
    })?;

    let mut latest_version: Option<String> = None;

    while let Some(version) = versions.next() {
        let version = version.unwrap().path();
        let version = version.file_name().unwrap().to_str().unwrap();

        if version.starts_with("node-v") {
            latest_version = Some(version.to_string());
        }
    }

    latest_version.ok_or(Error::NoNodeVersionInstalled(None))
}