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

    file_path: str,
    command: str,
    user: str,
    group: str,
    name: str,


}

mod checks {

    mod file_checks {
        use std::fs::File;
        use std::path::Path;
        use chksum_md5 as md5;
        use crate::checks::Algorithm;

        pub fn path_exists(file_path: &str) -> bool {
            return Path::new(&file_path).exists();
        }

        pub fn checksum_compare(file_path: &str, algorithm: Algorithm, comparison: &str) -> bool {
            return match algorithm {
                Algorithm::MD5 => md5::chksum(File::open(&file_path)) == comparison,
                _ => false,
            }
        }

    }

}