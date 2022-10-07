extern crate clap;
extern crate clap_derive;

mod args;

fn main() {
    args::Args::parse_patched();
    println!("Hello, world!");
}
