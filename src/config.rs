// /src/config.rs

mod config {
    use crate::checks::Vuln;

    let mut LoadedConfig: &'static ConfigFile; 
    let configPath: &str = env::current_exe()?.parent();

    struct ConfigFile {
        id: String,
        title: String,
        os: String,
        user: String,

        vulns: vec<Vuln>,
    }

    pub fn load_config() {
        
    }

}