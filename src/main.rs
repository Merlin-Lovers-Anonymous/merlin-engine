// /src/main.rs

/*

 __ __            _  _
|  \  \ ___  _ _ | |<_>._ _
|     |/ ._>| '_>| || || ' |
|_|_|_|\___.|_|  |_||_||_|_|

Copyright (c) 2023 Wilhelm, Donkey

*/

use std::env;
use std::process::exit;
use terminal_link::Link;

mod checks;
mod config;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        exit(0);
    }

    match args[1].as_str() {

        _ => unsafe {
            println!("Testing config load...");

            config::load_config("config.toml");

            println!("Loaded id: {}", config::LOADED_CONFIG.config.id);

            println!("Done");
            exit(0);
        },

        "--help" => {
            println!("Please see Merlin Engine documentation for an explanation of command-line parameters.");
            println!("Click {} to visit the wiki.", Link::new("here", "https://github.com/Merlin-Lovers-Anonymous/merlin-engine/wiki"));
            exit(0);
        },

        _other => {
            println!("'{}' is not a recognized command-line parameter.", args[1]);
            exit(0);
        }

    }
}