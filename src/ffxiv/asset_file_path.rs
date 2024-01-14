use std::path::PathBuf;
use regex::bytes::Regex;
use crate::ffxiv::asset_file_category::AssetFileCategory;
use crate::ffxiv::asset_file_chunk::AssetFileChunk;
use crate::ffxiv::asset_file_platform::AssetFilePlatform;
use crate::ffxiv::asset_file_repository::AssetFileRepository;

#[derive(Debug, Clone)]
pub struct AssetFilePath {
    pub file_path: PathBuf,
    pub file_path_str: String,
    pub file_name: String,
    pub file_stem: String,
    pub file_extension: String,
    pub data_category: AssetFileCategory,
    pub data_repository: AssetFileRepository,
    pub data_chunk: AssetFileChunk,
    pub data_platform: AssetFilePlatform,
}

impl PartialEq for AssetFilePath {
    fn eq(&self, other: &Self) -> bool {
        self.file_stem == other.file_stem
    }
}

impl AssetFilePath {
    pub fn new(file_path: PathBuf) -> Result<AssetFilePath, String> {
        let file_path_str = file_path.as_os_str().to_str().ok_or("Failed to convert path to str.")?;
        let file_name = file_path.file_name().ok_or("Failed to get file name.")?.to_str().ok_or("Failed to get file name as str.")?;
        let file_stem = file_path.file_stem().ok_or("Failed to get file name.")?.to_str().ok_or("Failed to get file name as str.")?;
        let file_extension = file_path.extension().ok_or("Failed to get file extension.")?.to_str().ok_or("Failed to get file extension as str.")?;
        //let file_path_components: Vec<Option<&str>> = file_path.components().map(|c| c.as_os_str().to_str()).collect();

        let file_name_bytes = file_name.as_bytes();
        let file_name_regex = Regex::new(r"^(\d|[a-z]){6}\.(win32|ps3|ps4)\.(dat|index)\d*$").or(Err("Failed to create regex"))?;
        file_name_regex.captures(file_name_bytes).ok_or(format!("File name '{}' is invalid.", file_name))?;

        let category_str = String::from_utf8(file_name_bytes[0..2].to_vec()).or(Err("Failed to slice name to category"))?;
        let repository_str = String::from_utf8(file_name_bytes[2..4].to_vec()).or(Err("Failed to slice name to repository"))?;
        let chunk_str = String::from_utf8(file_name_bytes[4..6].to_vec()).or(Err("Failed to slice name to chunk"))?;

        let data_category = AssetFileCategory::from_hex_str(&category_str)?;
        let data_repository = AssetFileRepository::from_hex_str(&repository_str)?;
        let data_chunk = AssetFileChunk::from_hex_str(&chunk_str)?;
        let data_platform = AssetFilePlatform::from_str_contains(&file_name)?;


        Ok(
            AssetFilePath {
                file_path_str: String::from(file_path_str),
                file_path: file_path.clone(),
                file_name: String::from(file_name),
                file_stem: String::from(file_stem),
                file_extension: String::from(file_extension),
                data_category,
                data_repository,
                data_chunk,
                data_platform,
            }
        )
    }
}