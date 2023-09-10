mod config;
mod search;

use config::Config;
use search::search;
use std::env;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let config = Config::from_args(&args)?;

    println!(
        "Searching for {}, in file {}",
        config.query, config.file_path
    );

    let contents = fs::read_to_string(config.file_path)?;

    for hit in search(&config.query, &contents, config.ignore_case) {
        println!("{}", hit);
    }

    Ok(())
}
