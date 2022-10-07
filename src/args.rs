use clap::{
    Parser,
};

#[allow(non_camel_case_types)]
#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Action {
    install,
    uninstall,
    switch,
    help,
    list,
}

#[derive(Parser, Debug, Clone)]
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
    #[clap(long, required=false, parse(from_os_str))]
    nmppath: Option<std::path::PathBuf>,

    /// Specifies the path to the npx installation directory.
    /// If not specified, the default path will be used.
    /// 
    /// The default path is same as nodepath.
    /// 
    /// If the npx path is not specified, npx will be installed in the same
    /// directory as npm.
    #[clap(long, required=false, parse(from_os_str))]
    npxpath: Option<std::path::PathBuf>,
}

impl Args {
    /// Assign path based on used operating system
    fn assign_path(&mut self) {
        if self.nodepath.is_none() {
            self.nodepath = Some(std::path::PathBuf::from(
                dirs::home_dir().unwrap().join(".nodejs")
            ));
        }
    }

    /// This function is used to parse the arguments and do the following operations:
    /// 1. Check if npm path is specified, if not, set it to the same as nodepath.
    /// 2. Check if npx path is specified, if not, set it to the same as nmppath or nodepath.
    /// 3. Assign default values to the paths if they are not specified.
    /// 
    /// Then return the parsed and patched arguments
    pub fn parse_patched() -> Self {
        let mut args = Self::parse();

        args.assign_path();

        if args.nmppath.is_none() {
            args.nmppath = args.nodepath.clone();
        }
        
        if args.npxpath.is_none() {
            args.npxpath = args.nmppath.clone();
        }

        args
    }
}