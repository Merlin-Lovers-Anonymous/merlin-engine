// /src/config.rs

mod config {
    use std::env;
    use std::path::Path;
    use crate::checks::Vuln;

    let mut LoadedConfig: &'static ConfigFile; 

    struct Data {
        config: Config,
    }

    struct ConfigFile {
        id: String,
        title: String,
        os: String,
        user: String,

        vulns: Vec<Vuln>,
    }

    pub fn load_config() {
        let configPath: &Path = env::current_exe()?.parent().expect("Could not find working directory.");
        
        let configData: Data = toml::
    }

}