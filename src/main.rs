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

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        exit(0);
    }

    match args[1].as_str() {

        "--test" => {
            println!("Testing check functions...");

            println!("Done");
        },

        "--help" => {
            println!("Please see Merlin Engine documentation for an explanation of command-line parameters.");
            println!("Click {} to visit the wiki.", Link::new("here", "https://github.com/Merlin-Lovers-Anonymous/merlin-engine/wiki"))
        },

        _other => {
            println!("{} is not a recognized command-line parameter.", args[1]);
        }

    }
}