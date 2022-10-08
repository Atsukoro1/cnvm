use std::path::PathBuf;
use super::Error;

/// Uninstall specific version of node.js, this function will also use
/// switch function to switch to the latest stable version of node.js that 
/// it can find.
/// 
/// # Arguments
/// 
/// * `nodeversion` - Version of node.js to uninstall
/// * `npmversion` - Version of npm to uninstall
/// * `configpath` - Path to the json config file
pub async fn execute(args: (Option<String>, Option<String>, PathBuf, PathBuf)) -> Result<(), Error> {
    println!("Installing");

    Ok(())
}