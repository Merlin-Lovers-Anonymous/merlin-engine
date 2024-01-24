// /src/checks.rs

enum CheckType {
    PathExists,
    ChecksumCompare,

}
enum Algorithm {
    MD5,
    SHA256,
    SHA512,
}

struct Vuln {
    pass_message: String,
    check_logic: String,
    hint: String,
    point_value: f32,


}

struct Check {
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
        use ring::digest::{Context, Digest, SHA256};
        use data_encoding::HEXUPPER;

        pub fn path_exists(file_path: &str) -> bool {
            return Path::new(&file_path).exists();
        }
        
        // pub fn checksum_compare(file_path: &str, algorithm: Algorithm, comparison: &str) -> bool {
        //     return match algorithm {
        //         Algorithm::MD5 => true,
        //         Algorithm::SHA256 => true,
        //         Algorithm::SHA512 => true,
        //     }
        // }
        pub fn calculate_sha256(file_path: &str) -> io::Result<String> {
            let mut file = File::open(file_path)?;
            let mut buffer = [0; 1024];
            let mut context = Context::new(&SHA256);

            loop {
                let count = file.read(&mut buffer)?;
                if (count == 0) {
                    break;
                }
                context.update(&buffer[..count]);
            }
            Ok(HEXUPPER.encode(context.finish().as_ref()))
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
        let hash = checks::file_checks::calculate_sha256("./tests/hash.txt").unwrap();
        println!("{:?}", hash);

        assert_eq!(hash, "F09DC8EA24B801E2E980E06F92110EF577A08F35A32EEC8613624FFD211BF394");
    }
}