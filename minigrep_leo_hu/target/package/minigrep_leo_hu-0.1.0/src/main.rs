extern crate minigrep_leo_hu;
use std::env;
use std::process;
use minigrep_leo_hu::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    // println!("Searching for {}", config.query);
    // println!("In file {}\n", config.filename);
    if let Err(e) = minigrep_leo_hu::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
