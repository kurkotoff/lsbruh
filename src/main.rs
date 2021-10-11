
use std::env::args;
use std::fs::read;
use std::process;

use lsbruh::Config;


fn main() {
    let config = Config::new(args()).unwrap_or_else(|err| {
        eprint!("Error reading command line arguments: {}", err);
        process::exit(1);
    });
    
    if let Err(e) = lsbruh::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}