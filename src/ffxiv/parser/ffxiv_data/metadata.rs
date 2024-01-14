pub mod file_name;
pub mod platform;
pub mod category;
pub mod repository;
pub mod index_path;
pub mod chunk;

use std::path::PathBuf;
use regex::bytes::Regex;
use crate::ffxiv::parser::ffxiv_data::metadata::category::Category;
use crate::ffxiv::parser::ffxiv_data::metadata::chunk::Chunk;
use crate::ffxiv::parser::ffxiv_data::metadata::platform::Platform;
use crate::ffxiv::parser::ffxiv_data::metadata::repository::Repository;

pub struct FFXIVFileMetadata {
    pub file_path: PathBuf,
    pub file_name: String,
    pub file_extension: String,
    pub data_category: Category,
    pub data_repository: Repository,
    pub data_chunk: Chunk,
    pub data_platform: Platform,
}

impl FFXIVFileMetadata {
    pub fn new(file_path: PathBuf) -> Result<FFXIVFileMetadata, String> {
        let full_path = file_path.as_os_str().to_str().ok_or("Failed to convert path to str.")?;
        let file_name = file_path.file_name().ok_or("Failed to get file name.")?.to_str().ok_or("Failed to get file name as str.")?;
        let file_stem = file_path.file_stem().ok_or("Failed to get file name.")?.to_str().ok_or("Failed to get file name as str.")?;
        let file_extension = file_path.extension().ok_or("Failed to get file extension.")?.to_str().ok_or("Failed to get file extension as str.")?;
        let file_path_components: Vec<Option<&str>> = file_path.components().map(|c| c.as_os_str().to_str()).collect();

        let file_name_regex =  Regex::new(r"^\d{6}\.(win32|ps3|ps4)\.index\d$").or(Err("Failed to create regex"))?;
        let file_name_valid = file_name_regex.captures(file_name.as_bytes()).ok_or(format!("File name '{}' is invalid.", file_name))?;
        let



        0
    }
}