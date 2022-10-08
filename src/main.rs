extern crate clap;
extern crate clap_derive;

mod args;
mod commands;
mod filesystem;

#[allow(unused)]
#[tokio::main]
async fn main() {
    let arguments: args::Args = args::Args::parse_patched();
    let para = (
        arguments.version,
        arguments.npmversion,
        arguments.path.unwrap(), 
        arguments.cnvm_path.unwrap()
    );

    // Run a command from command folder based on action argument
    match arguments.action {
        args::Action::install => commands::install::execute(para).await,
        args::Action::uninstall => commands::uninstall::execute(para).await,
        args::Action::switch => commands::switch::execute(para).await,
    };
}
