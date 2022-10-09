/// This file is used to get the right path to desired resource 
/// based on your operating system and CPU architecture

#[cfg(target_os = "windows")]
pub fn node_path(version: String) -> String {
    use std::env::consts::ARCH;

    if ARCH == "x86_64" {
        format!(
            "node-{}-win-x64", 
            version
        )
    } else {
        format!(
            "node-{}-win-x86", 
            version
        )
    }
}