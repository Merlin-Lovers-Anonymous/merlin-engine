// /src/config.rs

    use std::{env, fs};
    use std::io::Read;
    use std::path::Path;
    use std::process::exit;
    use serde::Deserialize;
    use crate::checks::Vuln;

    pub(crate) static mut LOADED_CONFIG: Data = Data {

        // Static variables must be initialized ☹️

        config: ConfigFile {

            id: "".to_string(),
            title: "".to_string(),
            os: "".to_string(),
            user: "".to_string(),
            vulns: vec![],

        }

    };

    #[derive(Deserialize)]
    struct Data {
        pub(crate) config: ConfigFile,
    }

    #[derive(Deserialize)]
    struct ConfigFile {

        // Structure for Config File

        pub(crate) id: String,
        title: String,
        os: String,
        user: String,

        vulns: Vec<Vuln>,
    }

    pub unsafe fn load_config(file_name: &str) { // Must be unsafe to allow editing of our static variable ☠️

        let config_path: &Path = env::current_exe()?.parent() // Working dir of the EXE
            .expect("Could not find working directory.") // Catch if dir does not exist
            .push(Path::new(&file_name)); // Config file name... Should be config.toml (Maybe allow changing in future?)

        let file_as_string: String = match fs::read_to_string(config_path) { // Files are scary. Must be safe.

            // If result is acceptable, set var to result
            Ok(a) => a,

            Err(_) => {
                // Print error message
                eprintln!("Could not read file '{:?}'. Does your config exist?", config_path);
                // Exit with error code 1
                exit(1);
            }
        };

        LOADED_CONFIG = match toml::from_str(&*file_as_string) {

            // If result is acceptable, set LOADED_CONFIG to our newly loaded config
            Ok(cnf) => cnf,

            Err(_) => {
                // Print error message
                eprintln!("Unable to load data from '{:?}'", config_path);
                // Exit with error code 1
                exit(1);
            },

        };
    }