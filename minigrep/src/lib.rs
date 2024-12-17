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
    let lines = search(&config.query, &text);
    if lines.len() < 1 {
        println!("[...] Not found.");
    } else {
        for line in lines {
            println!("{line}");
        }
    }
    Ok(())
}

fn search<'a>(query: &str, text: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in text.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "safe";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}