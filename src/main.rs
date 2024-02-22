use std::path::PathBuf;

use clap::Parser;

mod cbl;

// TODO: would be convenient to pipe XML into program during CI, but not a big deal to
// specify the file path

/// A CLI for converting legacy CBL files to JSON
#[derive(Parser)]
#[command(name = "cbl-convert")]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// The path to the CBL file to convert
    #[clap(long)]
    file: PathBuf,
}

fn main() {
    let Cli { file } = Cli::parse();

    if !file.exists() {
        eprintln!("File not found: {:?}", file);
        std::process::exit(1);
    }

    todo!("Implement the rest of the program lol")
}
