use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("File path and query required.");
        }
        let file_path = args[1].clone();
        let query = args[2].clone();
        Ok(Config { file_path, query })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string(config.file_path)?;
    print!("{text}");
    Ok(())
}
