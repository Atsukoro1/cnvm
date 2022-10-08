use quick_error::quick_error;

pub mod cnvm;

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
    }
}