use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Args {
    #[clap(subcommand)]
    subcmd: DirConCommand,
}

#[derive(Subcommand, Debug)]
enum DirConCommand {
    NewDir{
        path: Option<PathBuf>
    }
}

fn main() {
    println!("Hello, world!");
}
