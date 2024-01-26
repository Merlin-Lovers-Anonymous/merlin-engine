// /src/checks.rs

enum CheckType {
    PathExists,
    HashEquals,

}
enum Algorithm {
    MD5,
    SHA256,
    SHA512,
}

pub struct Vuln {
    pass_message: String,
    check_logic: String,
    hint: String,
    point_value: f32,

    checks: Vec<Check>,
}

pub struct Check {
    check_type: CheckType,

    file_path: String,
    command: String,
    user: String,
    group: String,
    name: String,


}

mod checks {

    pub(crate) mod file_checks {
        use std::fs::File;
        use std::io;
        use std::io::Read;
        use std::path::Path;
        use ring::digest::{Context, SHA256, SHA512}; // MD5
        use data_encoding::HEXUPPER;
        use crate::checks::Algorithm;

        // Check if a path exists
        pub fn path_exists(file_path: &str) -> bool {
            return Path::new(&file_path).exists();
        }
        
        // Check if a file matches the specified hash
        pub fn hash_equals(file_path: &str, algorithm: Algorithm, comparison: &str) -> bool {
            let hash = calculate_hash(&file_path, algorithm).unwrap();
            return hash.eq(comparison);
        }

        // Helper function to calculate file hash
        fn calculate_hash(file_path: &str, algorithm: Algorithm) -> io::Result<String> {
            let mut file = File::open(file_path)?;
            let mut buffer = [0; 1024];
            match algorithm {
                Algorithm::MD5 => {
                    //Context::new(&MD5);
                },
                Algorithm::SHA256 => {
                    Context::new(&SHA256);
                },
                Algorithm::SHA512 => {
                    Context::new(&SHA512);
                },
            }


            loop {
                let count = file.read(&mut buffer)?;
                if count == 0 {
                    break;
                }
                Context::update(&mut buffer[..count]);
            }

            Ok(HEXUPPER.encode(Context::finish().as_ref()))
        }

    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_path_exists_pass() {
        let file_path = "exists.txt";
        File::create(&file_path).expect("Error creating file");
        assert!(checks::file_checks::path_exists(&file_path));
        std::fs::remove_file(&file_path).expect("Error removing file");
    }

    #[test]
    fn test_path_exists_fail() {
        assert!(!checks::file_checks::path_exists("not_exists.txt"));
    }

    #[test]
    fn test_sha256_hash_pass() {
        assert!(checks::file_checks::hash_equals("./tests/hash.txt", Algorithm::SHA256, "F09DC8EA24B801E2E980E06F92110EF577A08F35A32EEC8613624FFD211BF394").unwrap());
    }
}