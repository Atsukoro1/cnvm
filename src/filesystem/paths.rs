use std::env::consts::ARCH;
use std::path::PathBuf;
use console::style;

/// Determine path to remote resource name based on the operating system and
/// CPU architecture
/// 
/// # Arguments:
/// 
/// * `version` - Version of node.js to download
/// 
/// # Returns:
/// 
/// * `String` - Path to the remote resource
#[cfg(target_os = "windows")]
pub fn node_path(version: String) -> String {
    return match ARCH {
        "x86" => format!("node-{}-win-x86", version),
        "x86_64" => format!("node-{}-win-x64", version),
        _ => panic!("Invalid CPU architecture")
    }
}

#[cfg(target_os = "linux")]
pub fn node_path(version: String) -> String {
    return format!(
        "node-{}-linux-{}",
        version,
        ARCH.to_string().replace("x86_64", "x64")
    );
}

/// Determine path to remote resource extension based on the operating system
/// and CPU architecture
/// 
/// # Returns
/// 
/// * `String` - Path to the remote resource
#[cfg(target_os = "windows")]
pub fn node_ext() -> String {
    return ".zip".to_string()
}

#[cfg(target_os = "linux")]
pub fn node_ext() -> String {
    return ".tar.gz".to_string()
}

/// Check if specific directory is in path
/// 
/// # Arguments:
/// 
/// * `node_path` - Path to the directory where symlinked node directory is located
/// * `cnvm_path` - Path to the cnvm directory
pub fn check_path(node_path: &PathBuf, cnvm_path: &PathBuf) {
    let path = std::env::var("PATH").unwrap();

    if !path.contains(node_path.to_str().unwrap()) || !path.contains(cnvm_path.to_str().unwrap()) {
        println!("Remind no path for now");
    }
}