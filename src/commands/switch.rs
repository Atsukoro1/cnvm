use console::style;
use crate::{
    filesystem::node::{
        remove_symlink, 
        self
    }, 
    commands::{
        BOX, 
        CHECK
    }
};
use std::path::PathBuf;
use super::Error;

/// Will switch to a specific version of node.js
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
        "{} {} Switching to version {} of Node...",
        style("[1/2]").bold().dim(),
        BOX,
        &node_version
    );

    remove_symlink(&args.2).expect("remove symlink failed");
    node::symlink_node(
        &args.3,
        node_version, 
        &args.2
    ).expect("linked failing");

    println!(
        "{} {} Done", 
        style("[2/2]").bold().dim(), 
        CHECK
    );

    Ok(())
}