use std::path::PathBuf;
use super::Error;

/// Will install specific version of node.js to the specified path
/// 
/// # Arguments
/// 
/// * `version` - Version of node.js to install
/// * `nodepath` - Path to the node.js installation directory
/// * `configpath` - Path to the json config file
pub async fn execute(args: (Option<String>, PathBuf, PathBuf)) -> Result<(), Error> {
    println!("Installing");

    let mut res = reqwest::get(
        format!("https://nodejs.org/dist/v14.15.4/v{}/win-x64/node.exe", 
            args.0.unwrap()
        ).as_str()
    ).await.unwrap();

    while let Some(chunk) = res.chunk().await.unwrap() {
        println!("Chunk: {:?}", chunk);
        println!("");
        println!("");
        println!("");
        println!("");
        println!("");
        println!("");
    }

    Ok(())
}