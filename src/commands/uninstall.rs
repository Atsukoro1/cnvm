use std::path::PathBuf;
use super::Error;

/// Uninstall specific version of node.js, this function will also use
/// switch function to switch to the latest stable version of node.js that 
/// it can find.
/// 
/// # Arguments
/// 
/// * `version` - Version of node.js to uninstall
/// * `nodepath` - Path to the node.js installation directory
/// * `configpath` - Path to the json config file
pub async fn execute(args: (Option<String>, PathBuf, PathBuf)) -> Result<(), Error> {
    println!("Installing");

    Ok(())
}