use std::path::{Path, PathBuf};
use crc::{Crc, CRC_32_JAMCRC, Digest};
use egui::TextBuffer;
use crate::ffxiv::parser::ffxiv_data::metadata::category::Category;
use crate::ffxiv::parser::ffxiv_data::metadata::platform::Platform;
use crate::ffxiv::parser::ffxiv_data::metadata::repository::Repository;

#[derive(Debug, Clone)]
pub struct IndexPath {
    pub full_path: String,
    pub file_extension: String,
    pub file_stem: String,
    pub index1_hash: u64,
    pub index2_hash: u32,
    pub data_repo: Repository,
    pub data_category: Category,
    pub platform: Platform,
}

impl IndexPath {
    pub fn from_str(path: &str) -> Result<IndexPath, String> {
        let path = PathBuf::from(path);
        IndexPath::new(path)
    }
    pub fn new(path: PathBuf) -> Result<IndexPath, String> {
        let full_path = path.as_os_str().to_str().ok_or("Failed to convert path to str.")?.to_lowercase();
        let path = PathBuf::from(&full_path);
        let data_name = path.file_name().ok_or("Failed to get file name.")?.to_str().ok_or("Failed to get file name as str.")?;
        let data_stem = path.file_stem().ok_or("Failed to get file name.")?.to_str().ok_or("Failed to get file name as str.")?;
        let data_extension = path.extension().ok_or("Failed to get file extension.")?.to_str().ok_or("Failed to get file extension as str.")?;
        let components: Vec<Option<&str>> = path.components().map(|c| c.as_os_str().to_str()).collect();

        let data_category = Category::from_str(components.get(0).ok_or("Failed to get category name.")?.ok_or("Failed to get category name as str.")?)?;
        let data_repo = Repository::from_str(components.get(1).ok_or("Failed to get repository name.")?.ok_or("Failed to get repository name as str.")?);
        let data_folder = path.parent().ok_or("Failed to get parent dir.")?.to_str().ok_or("Failed to convert parent dir to str.")?;

        let data_folder = IndexPath::hash(data_folder);
        let data_category_hash = IndexPath::hash(data_category.name.as_str());
        let data_name_hash = IndexPath::hash(data_name);
        let index1_hash = IndexPath::double_hash(data_folder, data_name_hash);
        let index2_hash = IndexPath::hash(&full_path);

        let platform = Platform::from_u32(0)?;

        Ok(
            IndexPath {
                full_path: String::from(full_path),
                file_extension: String::from(data_extension),
                file_stem: String::from(data_stem),
                index1_hash,
                index2_hash,
                data_repo,
                data_category,
                platform,
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