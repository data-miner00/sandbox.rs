mod config;
mod search;

use config::Config;
use search::search;
use std::env;
use std::error::Error;
use std::fs;

use crate::search::search_iter;

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::from_args(env::args())?;

    println!(
        "Searching for {}, in file {}",
        config.query, config.file_path
    );

    let contents = fs::read_to_string(config.file_path)?;

    for hit in search(&config.query, &contents, config.ignore_case) {
        println!("{}", hit);
    }

    for hit_iter in search_iter(&config.query, &contents, config.ignore_case) {
        println!("{}", hit_iter);
    }

    Ok(())
}
