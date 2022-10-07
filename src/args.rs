use clap::{
    Parser,
};

#[allow(non_camel_case_types)]
#[derive(clap::ValueEnum, Clone)]
pub enum Action {
    install,
    uninstall,
    switch,
    help,
    list,
}

#[derive(Parser)]
#[clap(author="Atsukoro1", about="Cross-platform node.js version manager")]
pub struct Args {
    /// Specifies the type of action to perform.
    #[clap(value_enum)]
    action: Action,

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
    version: Option<String>,
    
    /// Specifies the path to the node.js installation directory.
    /// If not specified, the default path will be used.
    /// 
    /// The path is actually not the folder but symlink to the folder, this is
    /// done to prevent moving large folders when switching versions.
    /// 
    /// The default path is:
    ///    Windows: %USERPROFILE%\.nodejs
    ///    Linux: $HOME/.nodejs
    ///    MacOS: $HOME/.nodejs
    #[clap(short, long, required=false, parse(from_os_str))]
    nodepath: Option<std::path::PathBuf>,

    /// Specifies the path to the npm installation directory.
    /// If not specified, the default path will be used.
    /// 
    /// The default path is same as nodepath.
    #[clap(short, long, required=false, parse(from_os_str))]
    nmppath: Option<std::path::PathBuf>,

    /// Specifies the path to the npx installation directory.
    /// If not specified, the default path will be used.
    /// 
    /// The default path is same as nodepath.
    /// 
    /// If the npx path is not specified, npx will be installed in the same
    /// directory as npm.
    #[clap(short, long, required=false, parse(from_os_str))]
    npxpath: Option<std::path::PathBuf>,
}

impl Args {
    /// This is only done to assign npm path to npx path,
    /// if npx path is not specified.
    pub fn parse_patched() -> Self {
        let mut args = Self::parse();

        if args.nmppath.is_some() && args.npxpath.is_none() {
            args.npxpath = args.nmppath.clone();
        }

        args
    }
}