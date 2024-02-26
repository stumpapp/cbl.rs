use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::time::Duration;
use std::time::Instant;

use cbl::CBL;
use clap::Parser;
use error::CLIResult;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use specs::SpecVersion;

mod cbl;
mod common;
mod error;
mod specs;

use specs::CURRENT_SPEC_VERSION;

use crate::specs::convert;

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

            let progress_bar = default_progress_spinner();
            progress_bar.set_message("Loading CBL file");

            let cbl = CBL::from_file(&file)?;

            // println!("{:?}", cbl);
            // println!("Spec version to convert to: {:}", spec_version.to_string());

            progress_bar.set_message(format!(
                "Converting CBL to {} spec",
                spec_version.to_string()
            ));

            let start = Instant::now();
            let json_string = convert(cbl, spec_version);
            let duration = start.elapsed();

            progress_bar.finish_with_message(format!("Conversion complete in {:?}", duration));

            File::create("output.json")?.write_all(json_string.as_bytes())?;
        }
    }

    Ok(())
}

pub(crate) fn default_progress_spinner() -> ProgressBar {
    let progress = ProgressBar::new_spinner();
    progress.enable_steady_tick(Duration::from_millis(120));
    progress.set_style(
        ProgressStyle::with_template("{spinner} {msg}")
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    );
    progress
}
