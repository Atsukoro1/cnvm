pub mod install;
pub mod uninstall;
pub mod switch;

#[repr(u8)]
pub enum Error {
    /// Can't create a new Reqwest client for some reason
    ClientInstantiateError = 0x00,
}