use clap::{
    Parser,
};

#[allow(non_camel_case_types)]
#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Action {
    install,
    uninstall,
    switch
}

#[derive(Parser, Debug, Clone)]
#[clap(author="Atsukoro1", about="Cross-platform node.js version manager")]
pub struct Args {
    /// Specifies the type of action to perform.
    #[clap(
        value_enum,
        required=true,
    )]
    pub action: Action,

    /// Version flag of Node.js that will only be required if the action 
    /// is install, uninstall or switch
    #[clap(
        short, 
        long, 
        required_ifs(&[
            ("action", "install"), 
            ("action", "switch"),
            ("action", "uninstall")
        ])
    )]
    pub version: Option<String>,
    
    /// Specifies the path to the node.js installation directory.
    /// If not specified, the default path will be used.
    /// 
    /// The path is actually not the folder but symlink to the folder, this is
    /// done to prevent moving large folders when switching versions.
    /// 
    /// The default path is:
    ///     - Windows: %USERPROFILE%\.nodejs
    ///     - Linux: $HOME/.nodejs
    ///     - MacOS: $HOME/.nodejs
    #[clap(
        short, 
        long, 
        required=false, 
        parse(from_os_str),
    )]
    pub nodepath: Option<std::path::PathBuf>,


    /// Specifies the path to the json config file
    /// 
    /// The default path is:
    ///     - Windows: %USERPROFILE%\.cnvm\config.json
    ///     - Linux: $HOME/.cnvm/config.json
    ///     - MacOS: $HOME/.cnvm/config.json
    /// 
    /// The config file is used to store all these things
    /// - The path to the node.js folder symlink
    #[clap(
        short, 
        long, 
        required=false, 
        parse(from_os_str)
    )]
    pub configpath: Option<std::path::PathBuf>,
}

impl Args {
    /// This function adds the default values to the arguments
    /// 
    /// It is currently not possible to do this with clap because of the
    /// &str/Strign problem when passing closure into default_value
    /// 
    /// TODO: Try to solve this in future
    pub fn parse_patched() -> Self {
        let mut args = Self::parse();

        if args.nodepath.is_none() {
            args.nodepath = Some(std::path::PathBuf::from(
                dirs::home_dir().unwrap().join(".nodejs")
            ));
        }

        if args.configpath.is_none() {
            args.configpath = Some(std::path::PathBuf::from(
                dirs::home_dir().unwrap().join(".cnvm/config.json")
            ));
        }

        args
    }
}