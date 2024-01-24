// /src/main.rs

/*

 __ __            _  _
|  \  \ ___  _ _ | |<_>._ _
|     |/ ._>| '_>| || || ' |
|_|_|_|\___.|_|  |_||_||_|_|

Copyright (c) 2023 Wilhelm, Donkey

*/

use std::env;
use terminal_link::Link;

mod checks;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 0 {
        return;
    }

    match &args[0] {

        _help => {
            println!("Please see Merlin Engine documentation for an explanation of command-line parameters.");
            println!("Click {} to visit the wiki.", Link::new("here", "https://github.com/Merlin-Lovers-Anonymous/merlin-engine/wiki"))
        },

    }
}