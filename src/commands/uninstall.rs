use crate::{commands::{BOX, CROSS}, filesystem::node::{self, get_latest_version, remove_symlink}};
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

    match get_latest_version(&args.3) {
        Ok(v) => {
            remove_symlink(&args.2).expect("remove symlink failed");
            node::symlink_node(&args.3, v.clone(), &args.2).expect("symlink node failed");

            println!(
                "{} {} NodeJS uninstalled, switched to version {}.",
                style("[2/2]").bold().dim(),
                CROSS,
                v.clone()
            );
        },
        Err(..) => {
            println!(
                "{} {} NodeJS uninstalled.",
                style("[2/2]").bold().dim(),
                CROSS
            );
        }
    }

    Ok(())
}