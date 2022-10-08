use std::path::PathBuf;
use super::Error;

/// Will switch to a specific version of node.js
/// 
/// # Arguments
/// 
/// * `version` - Version of node.js to switch to
/// * `nodepath` - Path to the node.js installation directory
/// * `configpath` - Path to the json config file
pub async fn execute(args: (Option<String>, PathBuf, PathBuf)) -> Result<(), Error> {
    println!("Installing");

    Ok(())
}