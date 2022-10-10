use quick_error::quick_error;

pub mod cnvm;
pub mod node;
pub mod paths;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        /// An invalid version of NodeJS was provided
        PermissionError(err: Option<String>) {
            display(
                "You do not have permission to create file or directory here! {}",
                err.as_ref().unwrap_or(&"".to_string())
            )
        }

        NoNodeVersionInstalled(err: Option<String>) {
            display(
                "No node version installed! {}",
                err.as_ref().unwrap_or(&"".to_string())
            )
        }

        TarUnpackFailed(err: Option<String>) {
            display(
                "Failed to unpack tar file! {}", 
                err.as_ref().unwrap_or(&"".to_string())
            )
        }

        UnsupportedOS(err: Option<String>) {
            display(
                "Your operating system is not supported! {}",
                err.as_ref().unwrap_or(&"".to_string())
            )
        }

        PathError(err: Option<String>) {
            display(
                "Can't set path! {}",
                err.as_ref().unwrap_or(&"".to_string())
            )
        }
    }
}