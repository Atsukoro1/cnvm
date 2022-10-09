use std::path::PathBuf;
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