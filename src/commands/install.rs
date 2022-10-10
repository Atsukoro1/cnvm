use crate::{filesystem::{
    cnvm,
    node
}, 
    commands::switch::execute as switch
};
use std::path::PathBuf;
use crate::filesystem::paths::{
    node_path, 
    node_ext
};
use console::style;
use super::{
    Error,
    NodeVersion,
    MAGNIFYING_GLASS,
    CHECK,
    BOX,
    DOWNLOAD
};

/// Will fetch bytes from remote resource and return a vector of them
pub async fn fetch_bytes(url: &str) -> Vec<u8> {
    let mut final_res = Vec::new();

    let mut res = reqwest::get(url).await.unwrap();

    let progress_bar = indicatif::ProgressBar::new(
        res.content_length().unwrap()
    );

    while let Some(chunk) = res.chunk().await.unwrap() {
        final_res.extend_from_slice(&chunk);
        progress_bar.inc(chunk.len() as u64);
    }

    final_res
}

/// Fetch all node.js versions and return the version user choosed or 
/// the latest stable one
pub async fn fetch_node_version(
    version: &Option<String>,
) -> Result<NodeVersion, Error> {
    // Fetch versions and serialize them into NodeVersion
    let versions: Vec<NodeVersion> = reqwest::get("https://nodejs.org/dist/index.json")
        .await
        .unwrap()
        .json::<Vec<NodeVersion>>()
        .await
        .unwrap();

    // If version is not specified, get the latest stable version
    let found = if version.is_none() {
        let latest_stable = versions.iter().find(|v| v.security == false);

        latest_stable
    } else {
        // If version is specified, get the version that matches the user input
        let version = versions.iter()
            .find(
                |v| v.version == ("v".to_owned() + version.as_ref().unwrap())
            );

        version
    };

    if found.is_none() {
        return Err(Error::InvalidVersion(None));
    };

    Ok(found.unwrap().clone())
}

/// Fetch from resource and save the Node executable to path
pub async fn install_node(args: (&Option<String>, &Option<String>, &PathBuf, &PathBuf)) -> Vec<u8> {
    let bytes = fetch_bytes(format!("https://nodejs.org/dist/{}/{}{}", 
        args.0.as_ref().unwrap(), 
        node_path(args.0.as_ref().unwrap().clone()),
        node_ext()
    ).as_str()).await;

    bytes
}

/// Will install specific version of node.js to the specified path
/// 
/// # Arguments
/// 
/// * `nodeversion` - Version of node.js to install
/// * `npmversion` - Version of npm to install
/// * `nodepath` - Path where node should be symlinked
/// * `cnvmpath` - Path to the cnvm folder
pub async fn execute(args: (Option<String>, Option<String>, PathBuf, PathBuf)) -> Result<(), Error> {
    println!(
        "{} {} Looking up information about the node version..", 
        style("[1/3]").bold().dim(),
        MAGNIFYING_GLASS
    );
    let node_version = fetch_node_version(&args.0).await?;

    println!(
        "{} {} Downloading node executable...", 
        style("[2/3]").bold().dim(),
        DOWNLOAD
    );
    let node_bytes = install_node((
        &Some(node_version.version.clone()), 
        &args.1, 
        &args.2, 
        &args.3
    )).await;

    let cfg_creation = cnvm::config_file_init(&args.3);
    if cfg_creation.is_err() {
        return Err(Error::ConfigFileError(None));
    };

    println!(
        "{} {} Unzipping data...", 
        style("[3/3]").bold().dim(),
        BOX
    );
    node::create_node(&args.3, &node_bytes)
        .expect("Bruh!");

    println!(
        "{} {} {}",
        style("[4/4]").bold().dim(),
        CHECK,
        style("NodeJS succefully installed!").bold().green(),
    );

    switch((Some(node_version.version), args.1, args.2, args.3))
        .await
        .unwrap();

    Ok(())
}