use crc::{Crc, CRC_32_JAMCRC, Digest};
use std::path::Path;
use std::ptr::hash;

#[derive(Debug)]
pub struct GamePath {
    pub category: String,
    pub full_path: String,
    pub repository: String,
    pub index_hash: u64,
    pub index_hash2: u32,
    pub category_hash: u32,
    pub full_file_name_hash: u32
}

impl GamePath {
    pub fn new(full_path: &str) -> GamePath {
        let path = Path::new(full_path);
        let full_path = path.as_os_str().to_str().unwrap();
        let full_file_name = path.file_name().unwrap().to_str().unwrap();
        let file_name = path.file_stem().unwrap().to_str().unwrap();
        let file_extension = path.extension().unwrap().to_str().unwrap();
        let components: Vec<&str> = path.components().map(|c| c.as_os_str().to_str().unwrap()).collect();
        let category = components[0];
        let mut repo = components[1];

        let category_hash = GamePath::hash(category);
        let full_file_name_hash = GamePath::hash(full_file_name);
        let index_hash = GamePath::double_hash(category_hash, full_file_name_hash);
        let index_hash2 = GamePath::hash(full_path);

        let repo_bytes = repo.as_bytes();
        if repo_bytes[0] != b'e' || repo_bytes[1] != b'x' || !(repo_bytes[2] as char).is_alphanumeric() {
            repo = "ffxiv";
        }

        GamePath {
            category: String::from(category),
            full_path: String::from(full_path),
            repository: String::from(repo),
            index_hash,
            index_hash2,
            category_hash,
            full_file_name_hash
        }
    }

    fn hash(string: &str) -> u32 {
        let crc = Crc::<u32>::new(&CRC_32_JAMCRC);
        let mut digest = crc.digest();
        digest.update(string.as_bytes());
        digest.finalize()
    }

    fn double_hash(category_hash: u32, full_file_name_hash: u32) -> u64 {
        return ((category_hash as u64) << 32) | (full_file_name_hash as u64)
    }
}



// pub fn crc32() {
//
// }