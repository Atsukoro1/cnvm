use std::path::PathBuf;
use super::Error;

/// Will switch to a specific version of node.js
/// 
/// # Arguments
/// 
/// * `nodeversion` - Version of node.js to switch to
/// * `npmversion` - Version of npm to switch to
/// * `configpath` - Path to the json config file
pub async fn execute(args: (Option<String>, Option<String>, PathBuf, PathBuf)) -> Result<(), Error> {
    println!("Installing");

    Ok(())
}