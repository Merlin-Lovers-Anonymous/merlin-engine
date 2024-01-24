// /src/config.rs

mod config {
    use crate::checks::Vuln;


    struct ConfigFile {
        id: String,
        title: String,
        os: String,
        user: String,

        vulns: [Vuln; 200],
    }

}