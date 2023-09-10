use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn from_args(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].to_owned();
        let file_path = args[2].to_owned();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Self {
            query,
            file_path,
            ignore_case,
        })
    }
}
