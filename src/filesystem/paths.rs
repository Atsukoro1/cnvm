/// This file is used to get the right path to desired resource 
/// based on your operating system and CPU architecture

#[cfg(target_os = "windows")]
pub fn node_path(version: String) -> String {
    use std::env::consts::ARCH;

    return match ARCH {
        "x86" => format!("node-{}-x86", version),
        "x86_64" => format!("node-{}-x64", version),
        _ => panic!("Invalid CPU architecture")
    }
}

#[cfg(target_os = "linux")]
pub fn node_path(version: String) -> String {
    use std::env::consts::ARCH;

    return format!(
        "node-{}-linux-{}",
        version,
        ARCH.to_string().replace("x86_64", "x64")
    );
}

#[cfg(target_os = "windows")]
pub fn node_ext() -> String {
    return ".zip".to_string()
}

#[cfg(target_os = "linux")]
pub fn node_ext() -> String {
    return ".tar.gz".to_string()
}