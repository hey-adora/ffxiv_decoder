use std::fmt::{Display, Formatter};
use std::io::ErrorKind;
use std::path::PathBuf;
use crc::{Crc, CRC_32_JAMCRC};
use egui::Key::N;
use regex::bytes::Regex as RegByte;

use thiserror::Error;
use crate::ffxiv::metadata::{Category, Chunk, Platform, Repository};

#[derive(Debug, Clone)]
pub struct DatPath {
    pub path: PathBuf,
    pub path_str: String,
    pub path_extension: String,
    pub path_name: String,
    pub path_stem: String,
    pub path_dir: String,
    pub index1_hash: u64,
    pub index2_hash: u32,
    pub data_repo: Repository,
    pub data_category: Category,
    pub data_platform: Platform,
}

impl DatPath {
    pub fn new(full_path: &str) -> Result<DatPath, String> {
        let full_path = full_path.to_lowercase();
        let path = PathBuf::from(&full_path);
        let path_name = path.file_name().ok_or("Failed to get file name.")?.to_str().ok_or("Failed to get file name as str.")?;
        let path_stem = path.file_stem().ok_or("Failed to get file name.")?.to_str().ok_or("Failed to get file name as str.")?;
        let path_extension = path.extension().ok_or("Failed to get file extension.")?.to_str().ok_or("Failed to get file extension as str.")?;
        let path_dir = path.parent().ok_or("Failed to get parent dir.")?.to_str().ok_or("Failed to convert parent dir to str.")?;

        let components: Vec<Option<&str>> = path.components().map(|c| c.as_os_str().to_str()).collect();
        let data_category = Category::from_str(components.get(0).ok_or("Failed to get category name.")?.ok_or("Failed to get category name as str.")?).unwrap();
        let data_repo = Repository::from_str(components.get(1).ok_or("Failed to get repository name.")?.ok_or("Failed to get repository name as str.")?);

        let data_folder = DatPath::hash(path_dir);
        //let data_category_hash = AssetPath::hash(data_category.name.as_str());
        let data_name_hash = DatPath::hash(path_name);
        let data_platform = Platform::from_u32(0)?;

        let index1_hash = DatPath::double_hash(data_folder, data_name_hash);
        let index2_hash = DatPath::hash(&full_path);

        Ok(
            DatPath {
                path: path.clone(),
                path_str: String::from(full_path),
                path_extension: String::from(path_extension),
                path_stem: String::from(path_stem),
                path_dir: String::from(path_dir),
                path_name: String::from(path_name),
                index1_hash,
                index2_hash,
                data_repo,
                data_category,
                data_platform,
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

//==================================================================================================

#[derive(Debug, Clone)]
pub struct FilePath {
    pub path: PathBuf,
    pub path_str: String,
    pub path_name: String,
    pub path_stem: String,
    pub path_extension: String,
    pub data_category: Category,
    pub data_repository: Repository,
    pub data_chunk: Chunk,
    pub data_platform: Platform,
}

impl PartialEq for FilePath {
    fn eq(&self, other: &Self) -> bool {
        self.path_stem == other.path_stem
    }
}

impl FilePath {
    pub fn new(file_path: PathBuf) -> Result<FilePath, String> {
        let file_path_str = file_path.as_os_str().to_str().ok_or("Failed to convert path to str.")?;
        let file_name = file_path.file_name().ok_or("Failed to get file name.")?.to_str().ok_or("Failed to get file name as str.")?;
        let file_stem = file_path.file_stem().ok_or("Failed to get file name.")?.to_str().ok_or("Failed to get file name as str.")?;
        let file_extension = file_path.extension().ok_or("Failed to get file extension.")?.to_str().ok_or("Failed to get file extension as str.")?;
        //let file_path_components: Vec<Option<&str>> = file_path.components().map(|c| c.as_os_str().to_str()).collect();

        let file_name_bytes = file_name.as_bytes();
        let file_name_regex = RegByte::new(r"^(\d|[a-z]){6}\.(win32|ps3|ps4)\.(dat|index)\d*$").or(Err("Failed to create regex"))?;
        file_name_regex.captures(file_name_bytes).ok_or(format!("File name '{}' is invalid.", file_name))?;

        let category_str = String::from_utf8(file_name_bytes[0..2].to_vec()).or(Err("Failed to slice name to category"))?;
        let repository_str = String::from_utf8(file_name_bytes[2..4].to_vec()).or(Err("Failed to slice name to repository"))?;
        let chunk_str = String::from_utf8(file_name_bytes[4..6].to_vec()).or(Err("Failed to slice name to chunk"))?;

        let data_category = Category::from_hex_str(&category_str).unwrap();
        let data_repository = Repository::from_hex_str(&repository_str)?;
        let data_chunk = Chunk::from_hex_str(&chunk_str)?;
        let data_platform = Platform::from_str_contains(&file_name)?;


        Ok(
            FilePath {
                path_str: String::from(file_path_str),
                path: file_path.clone(),
                path_name: String::from(file_name),
                path_stem: String::from(file_stem),
                path_extension: String::from(file_extension),
                data_category,
                data_repository,
                data_chunk,
                data_platform,
            }
        )
    }
}

//==================================================================================================

#[derive(Error, Debug)]
pub enum PathError {
    #[error("Asset Path: `{0}`")]
    Invalid(String),
}
