use std::{env, process};
use grep::Config;

fn main() {
    let args :Vec<String>= env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("error: {}", err);
        process::exit(1);
    });

    if let Err(e) = grep::read_file(config) {
        eprintln!("application error: {}", e);
        process::exit(1);
    }
}

