use minigrep::{run, Config};
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("{err}");
        process::exit(1);
    });

    match run(config) {
        Err(e) => {
            println!("{e}");
            process::exit(1);
        }
        Ok(()) => (),
    }
}
