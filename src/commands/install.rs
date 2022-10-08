use std::path::PathBuf;
use console::style;
use super::Error;

/// This function will fetch all bytes from the remote source pointing
/// to node.js executable and save it into file
pub async fn install_node(args: (Option<String>, PathBuf, PathBuf)) -> Vec<u8> {
    let mut final_res = Vec::new();

    let mut res = reqwest::get(
        format!("https://nodejs.org/dist/v{}/win-x64/node.exe", 
            args.0.unwrap()
        ).as_str()
    ).await.unwrap();

    let progress_bar = indicatif::ProgressBar::new(
        res.content_length().unwrap()
    );

    while let Some(chunk) = res.chunk().await.unwrap() {
        final_res.extend_from_slice(&chunk);
        progress_bar.inc(chunk.len() as u64);
    }

    final_res
}

/// Will install specific version of node.js to the specified path
/// 
/// # Arguments
/// 
/// * `version` - Version of node.js to install
/// * `nodepath` - Path to the node.js installation directory
/// * `configpath` - Path to the json config file
pub async fn execute(args: (Option<String>, PathBuf, PathBuf)) -> Result<(), Error> {
    println!(
        "{} {} Installing Node.js version {}...",
        style("[1/4]").bold().green(),
        "🚚",
        args.0.as_ref().unwrap()
    );
    install_node(args).await;

    Ok(())
}