use minify::Config;
use std::{env, process};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|_err| {
        eprintln!("Problem parsing arguments");
        process::exit(1);
    });

    if let Err(e) = minify::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}