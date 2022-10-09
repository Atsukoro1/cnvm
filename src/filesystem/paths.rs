#[cfg(target_os = "windows")]
pub fn node_path(version: String) -> String {
    return format!(
        "node-{}-win-x64",
        version
    )
}