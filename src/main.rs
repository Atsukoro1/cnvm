extern crate clap;
extern crate clap_derive;

mod args;

fn main() {
    let arguments = args::Args::parse_patched();
    println!("{:?}", arguments);
}
