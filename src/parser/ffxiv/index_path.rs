use std::path::{Path, PathBuf};
use crc::{Crc, CRC_32_JAMCRC, Digest};
use crate::parser::ffxiv::{Category, Platform, Repository};

#[derive(Debug)]
pub struct IndexPath {
    pub index1_hash: u64,
    pub index2_hash: u32,
    pub data_repo: Repository,
    pub data_category: Category,
    pub platform: Platform,
}

impl IndexPath {
    pub fn new(full_path: &str) -> Result<IndexPath, String> {
        let path = Path::new(full_path);
        let data_name = path.file_name().ok_or("Failed to get file name.")?.to_str().ok_or("Failed to get file name as str.")?;
        let data_path = path.as_os_str().to_str().ok_or("Failed to get full path as str.")?;
        let data_stem = path.file_stem().ok_or("Failed to get file name.")?.to_str().ok_or("Failed to get file name as str.")?;
        let data_extension = path.extension().ok_or("Failed to get file extension.")?.to_str().ok_or("Failed to get file extension as str.")?;

        let components: Vec<Option<&str>> = path.components().map(|c| c.as_os_str().to_str()).collect();
        let data_category = Category::from_str(components.get(0).ok_or("Failed to get category name.")?.ok_or("Failed to get category name as str.")?)?;
        let data_repo = Repository::from_str(components.get(1).ok_or("Failed to get repository name.")?.ok_or("Failed to get repository name as str.")?);

        let data_category_hash = IndexPath::hash(data_category.name.as_str());
        let data_name_hash = IndexPath::hash(data_name);
        let index1_hash = IndexPath::double_hash(data_category_hash, data_name_hash);
        let index2_hash = IndexPath::hash(full_path);

        let platform = Platform::from_number(0)?;

        Ok(
            IndexPath {
                index1_hash,
                index2_hash,
                data_repo,
                data_category,
                platform
            }
        )
    }


    fn hash(string: &str) -> u32 {
        let crc = Crc::<u32>::new(&CRC_32_JAMCRC);
        let mut digest = crc.digest();
        digest.update(string.as_bytes());
        digest.finalize()
    }

    fn double_hash(category_hash: u32, file_name_hash: u32) -> u64 {
        return ((category_hash as u64) << 32) | (file_name_hash as u64);
    }
}