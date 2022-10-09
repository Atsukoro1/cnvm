use serde::{Serialize, Deserialize};
use quick_error::quick_error;
use console::Emoji;

pub mod install;
pub mod uninstall;
pub mod switch;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        /// An invalid version of NodeJS was provided
        InvalidVersion(err: Option<String>) {
            display(
                "An invalid version of NodeJS was provided! {}", 
                err.as_ref().unwrap_or(&"".to_string())
            )
        }

        ConfigFileError(err: Option<String>) {
            display(
                "An error occured while trying to read or write to the config file! {}", 
                err.as_ref().unwrap_or(&"".to_string())
            )
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NodeVersion {
    pub version: String,
    pub npm: Option<String>,
    pub security: bool,
    pub date: String,
    pub files: Vec<String>
}

const DOWNLOAD: Emoji = Emoji("📥 ", " ");
const MAGNIFYING_GLASS: Emoji<'_, '_> = Emoji("🔍 ", " ");
const BOX: Emoji<'_, '_> = Emoji("📦 ", " ");
const CHECK: Emoji = Emoji("✅ ", " ");
pub const CROSS: Emoji = Emoji("❌ ", " ");