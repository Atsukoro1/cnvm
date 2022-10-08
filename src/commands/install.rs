use std::path::PathBuf;
use console::style;
use super::Error;

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

/// Fetch from resource and save the Node executable to path
pub async fn install_node(args: (Option<String>, Option<String>, PathBuf)) -> Vec<u8> {
    let bytes = fetch_bytes(format!("https://nodejs.org/dist/v{}/win-x64/node.exe", 
        args.0.unwrap()
    ).as_str()).await;

    bytes
}

/// Fetch from resource and save the NPM executable to path
pub async fn install_npm(args: (Option<String>, Option<String>, PathBuf)) -> Vec<u8> {
    let url = if args.1.is_some() {
        format!("https://nodejs.org/dist/npm/npm-{}.zip", 
            args.1.unwrap()
        ).to_owned()
    } else {
        "https://nodejs.org/dist/npm/npm-1.4.9.zip".to_owned()
    };

    let bytes = fetch_bytes(&url).await;

    bytes
}

/// Will install specific version of node.js to the specified path
/// 
/// # Arguments
/// 
/// * `nodeversion` - Version of node.js to install
/// * `npmversion` - Version of npm to install
/// * `configpath` - Path to the json config file
pub async fn execute(args: (Option<String>, Option<String>, PathBuf)) -> Result<(), Error> {
    println!(
        "{} {} Installing Node.js version {}...",
        style("[1/4]").bold().green(),
        "⚙️",
        args.0.as_ref().unwrap()
    );
    install_node(args.to_owned()).await;

    // Installing npm
    println!(
        "{} {} Installing {} NPM version...",
        style("[2/4]").bold().green(),
        "📦",
        if args.1.is_none() {
            "latest"
        } else {
            args.1.as_ref().unwrap()
        }
    );
    install_npm(args.to_owned()).await;

    Ok(())
}