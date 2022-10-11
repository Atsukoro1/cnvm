use serde::{Serialize, Deserialize};
use quick_error::quick_error;

pub mod install;
pub mod uninstall;
pub mod switch;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        /// An invalid version of NodeJS was provided
        InvalidVersion(err: Option<String>) {
            display(
                "Invalid version of Node.js provided!",
            )
        }

        ConfigDirError(err: Option<String>) {
            display(
                "{}", 
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