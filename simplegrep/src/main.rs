use std::{env, error::Error, process};

mod config;
mod custom_err;
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
    let args = env::args().collect();
    let config = GrepConfig::build(args)?;
    let content = read_file(&config.filepath)?;
    let grepped_lines = grep(config.searchstring, &content);

    println!("Lines Match: \n{grepped_lines}");

    Ok(())
}
