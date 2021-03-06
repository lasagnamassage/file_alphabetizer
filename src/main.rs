use std::{path::PathBuf};
use clap::Parser;
use file_alphabetizer;

#[derive(Parser)]
pub struct CLI {
    #[clap(parse(from_os_str))]
    pub filepath: std::path::PathBuf
}

#[warn(unused_must_use)]
fn main() {
    let args: CLI = CLI::parse();
    let p = PathBuf::from(args.filepath);
    file_alphabetizer::alphabetize(&p).unwrap();
}