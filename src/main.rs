use std::env;
use std::process;

use bambu_launcher::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("ERROR: Invalid syntax. {}", err);
        process::exit(1);
    });
    if let Err(e) = bambu_launcher::run(config) {
        eprintln!("ERROR: {}", e);
        process::exit(1);
    };
}

