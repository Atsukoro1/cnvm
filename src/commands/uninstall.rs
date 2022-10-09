use crate::{commands::BOX, filesystem::node};
use std::path::PathBuf;
use console::style;
use super::Error;

/// Uninstall specific version of NodeJS, this function will remove the version
/// from cnvm directory and also the symlink to the path. 
/// 
/// The used version of node.js will be switched to the lastest stable version that
/// it can find on our system.
/// 
/// # Arguments
/// 
/// * `nodeversion` - Version of node.js to install
/// * `npmversion` - Version of npm to install
/// * `nodepath` - Path where node should be symlinked
/// * `cnvmpath` - Path to the cnvm folder
pub async fn execute(args: (Option<String>, Option<String>, PathBuf, PathBuf)) -> Result<(), Error> {
    let node_version = match &args.0 {
        Some(v) => {
            if v.starts_with("v") {
                v.clone()
            } else {
                format!("v{}", v)
            }
        },
        None => return Err(Error::InvalidVersion(None)),
    };

    println!(
        "{} {} Uninstalling version {} of Node...",
        style("[1/2]").bold().dim(),
        BOX,
        &node_version
    );

    node::remove_node(&node_version, &args.3).expect("remove node version failed");

    Ok(())
}