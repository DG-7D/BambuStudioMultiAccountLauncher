use std::env;
use std::process;

use bambu_launcher::Config;

fn main() {
    println!("Bambu Studio Multi-Account Launcher v{}", env!("CARGO_PKG_VERSION"));
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("ERROR: Invalid syntax. {}", err);
        process::exit(1);
    });
    loop {
        match bambu_launcher::run(&config) {
            Ok(true) => {
                continue;      
            }
            Ok(false) => {
                process::exit(0);
            }
            Err(e) => {
                eprintln!("ERROR: {}", e);
                process::exit(1);
            }
        }
    }
}

