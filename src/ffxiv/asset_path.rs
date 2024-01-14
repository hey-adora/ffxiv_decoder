use std::path::{PathBuf};
use crc::{Crc, CRC_32_JAMCRC};
use crate::ffxiv::asset_file_category::AssetFileCategory;
use crate::ffxiv::asset_file_platform::AssetFilePlatform;
use crate::ffxiv::asset_file_repository::AssetFileRepository;

#[derive(Debug, Clone)]
pub struct AssetPath {
    pub full_path: String,
    pub file_extension: String,
    pub file_stem: String,
    pub index1_hash: u64,
    pub index2_hash: u32,
    pub data_repo: AssetFileRepository,
    pub data_category: AssetFileCategory,
    pub platform: AssetFilePlatform,
}

impl AssetPath {
    pub fn from_str(path: &str) -> Result<AssetPath, String> {
        let path = PathBuf::from(path);
        AssetPath::new(path)
    }
    pub fn new(path: PathBuf) -> Result<AssetPath, String> {
        let full_path = path.as_os_str().to_str().ok_or("Failed to convert path to str.")?.to_lowercase();
        let path = PathBuf::from(&full_path);
        let data_name = path.file_name().ok_or("Failed to get file name.")?.to_str().ok_or("Failed to get file name as str.")?;
        let data_stem = path.file_stem().ok_or("Failed to get file name.")?.to_str().ok_or("Failed to get file name as str.")?;
        let data_extension = path.extension().ok_or("Failed to get file extension.")?.to_str().ok_or("Failed to get file extension as str.")?;
        let components: Vec<Option<&str>> = path.components().map(|c| c.as_os_str().to_str()).collect();

        let data_category = AssetFileCategory::from_str(components.get(0).ok_or("Failed to get category name.")?.ok_or("Failed to get category name as str.")?)?;
        let data_repo = AssetFileRepository::from_str(components.get(1).ok_or("Failed to get repository name.")?.ok_or("Failed to get repository name as str.")?);
        let data_folder = path.parent().ok_or("Failed to get parent dir.")?.to_str().ok_or("Failed to convert parent dir to str.")?;

        let data_folder = AssetPath::hash(data_folder);
        //let data_category_hash = AssetPath::hash(data_category.name.as_str());
        let data_name_hash = AssetPath::hash(data_name);
        let index1_hash = AssetPath::double_hash(data_folder, data_name_hash);
        let index2_hash = AssetPath::hash(&full_path);

        let platform = AssetFilePlatform::from_u32(0)?;

        Ok(
            AssetPath {
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