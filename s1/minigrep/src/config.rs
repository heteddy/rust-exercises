// use std::convert::From;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub is_lower: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        
        
        args.next();

        let query = match args.next() {
            Some(s) => s,
            None => return Err("no query parameter"),
        };
        let file_path = match args.next() {
            Some(s) => s,
            None => return Err("no file_path "),
        };
        let is_lower = env::var("is_lowercase").is_ok();
        println!("is_lower={:?}",is_lower);
        Ok(Config {
            query,
            file_path,
            is_lower,
        })
    }
}

// impl From<Config> for Config {}
