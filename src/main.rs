use std::path::PathBuf;

use cbl::CBL;
use clap::Parser;
use error::CLIResult;
use specs::SpecVersion;

mod cbl;
mod common;
mod error;
mod specs;

use specs::CURRENT_SPEC_VERSION;

// TODO: https://github.com/Keats/validator would be useful

// TODO: would be convenient to pipe XML into program during CI, but not a big deal to
// specify the file path

/// A CLI for converting legacy CBL files to JSON
#[derive(Parser)]
#[command(name = "cblrs")]
#[command(author, version, about, long_about = None)]
pub struct CLI {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Parser)]
enum Command {
    CurrentSpec,
    Convert {
        /// The path to the CBL file to convert
        #[clap(long)]
        file: PathBuf,
        /// The version of the CBL spec to convert to, defaults to whatever
        /// the current version is
        #[clap(value_enum, long, default_value = "1.0-draft")]
        spec_version: SpecVersion,
        /// The path to write the converted CBL file to, defaults to stdout
        output: Option<PathBuf>,
    },
}

fn main() -> CLIResult<()> {
    let CLI { command } = CLI::parse();

    match command {
        Command::CurrentSpec => {
            println!("{CURRENT_SPEC_VERSION}");
        }
        Command::Convert {
            file, spec_version, ..
        } => {
            if !file.exists() {
                eprintln!("File not found: {:?}", file);
                std::process::exit(1);
            }

            let cbl = CBL::from_file(&file)?;

            println!("{:?}", cbl);

            println!("Spec version to convert to: {:}", spec_version.to_string());
        }
    }

    Ok(())
}
