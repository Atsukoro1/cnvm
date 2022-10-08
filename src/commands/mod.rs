use serde::{Serialize, Deserialize};

pub mod install;
pub mod uninstall;
pub mod switch;

#[repr(u8)]
#[derive(Debug)]
pub enum Error {
    /// Can't create a new Reqwest client for some reason
    ClientInstantiateError = 0x00,

    /// User specified invalid version of node.js
    InvalidVersion = 0x02,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NodeVersion {
    pub version: String,
    pub npm: Option<String>,
    pub security: bool,
    pub date: String,
    pub files: Vec<String>
}