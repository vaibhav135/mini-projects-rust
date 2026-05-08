use std::{error::Error, process};

mod config;
mod file;
mod grep;

use config::GrepConfig;
use file::read_file;
use grep::grep;

fn main() {
    if let Err(error) = run() {
        println!("application error: {}", error);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let config = GrepConfig::build()?;
    let content = read_file(&config.filepath)?;
    let grepped_lines = grep(config.searchstring, &content);

    println!("Lines Match: \n{grepped_lines}");

    Ok(())
}
