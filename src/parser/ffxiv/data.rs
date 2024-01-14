use std::fmt::Error;
use crc::{Crc, CRC_32_JAMCRC, Digest};
use std::path::{Path, PathBuf};
use std::ptr::hash;

#[derive(Debug)]
pub struct Data {
    pub repository: String,
    pub category: String,
    pub data_path: String,
    pub data_name_hash: u32,
    pub data_category_hash: u32,
    pub index_hash: u64,
    pub index_hash2: u32,
    // pub file_offset: u32,
    // pub file_path: PathBuf

}

impl Data {
    pub fn new(full_path: &str) -> Result<Data, String> {

        let path = Path::new(full_path);
        let data_path = path.as_os_str().to_str().ok_or("Failed to get full path as str.")?;
        let data_name = path.file_name().ok_or("Failed to get full file name.")?.to_str().ok_or("Failed to get full file name as str.")?;
        let data_stem = path.file_stem().ok_or("Failed to get file name.")?.to_str().ok_or("Failed to get file name as str.")?;
        let data_extension = path.extension().ok_or("Failed to get file extension.")?.to_str().ok_or("Failed to get file extension as str.")?;
        let components: Vec<Option<&str>> = path.components().map(|c| c.as_os_str().to_str()).collect();
        let category = components.get(0).ok_or("Failed to get category name.")?.ok_or("Failed to get category name as str.")?;
        let mut repo = components.get(1).ok_or("Failed to get repository name.")?.ok_or("Failed to get repository name as str.")?;

        let data_category_hash = Data::hash(category);
        let data_name_hash = Data::hash(data_name);
        let index_hash = Data::double_hash(data_category_hash, data_name_hash);
        let index_hash2 = Data::hash(full_path);

        let repo_bytes = repo.as_bytes();
        if repo_bytes[0] != b'e' || repo_bytes[1] != b'x' || !(repo_bytes[2] as char).is_alphanumeric() {
            repo = "ffxiv";
        }

        Ok(
            Data {
                repository: String::from(repo),
                category: String::from(category),
                data_path: String::from(full_path),
                data_category_hash,
                index_hash,
                index_hash2,
                data_name_hash,
            }
        )
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