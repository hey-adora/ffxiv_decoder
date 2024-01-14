use crc::{Crc, CRC_32_JAMCRC};
use egui::Key::N;
use regex::bytes::Regex as RegByte;
use std::fmt::{Display, Formatter};
use std::io::ErrorKind;
use std::path::PathBuf;

use crate::ffxiv::metadata::{Category, Chunk, MetadataError, Platform, Repository};
use thiserror::Error;

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
    pub fn new(full_path: &str) -> Result<DatPath, PathError> {
        // let path_name = path
        //     .file_name()
        //     .ok_or(PathError::Invalid(format!(
        //         "Failed to get file name: {}",
        //         full_path
        //     )))?
        //     .to_str()
        //     .ok_or(PathError::Invalid(format!(
        //         "Failed to get file name as str: {}",
        //         full_path
        //     )))?;
        // let path_stem = path
        //     .file_stem()
        //     .ok_or(PathError::Invalid(format!(
        //         "Failed to get file name: {}",
        //         full_path
        //     )))?
        //     .to_str()
        //     .ok_or(PathError::Invalid(format!(
        //         "Failed to get file name as str."
        //     )))?;
        // let path_extension = path
        //     .extension()
        //     .ok_or(PathError::Invalid(format!(
        //         "Failed to get file extension: {}",
        //         full_path
        //     )))?
        //     .to_str()
        //     .ok_or(PathError::Invalid(format!(
        //         "Failed to get file extension as str: {}",
        //         full_path
        //     )))?;
        // let path_dir = path
        //     .parent()
        //     .ok_or(PathError::Invalid(format!("Failed to get parent dir.")))?
        //     .to_str()
        //     .ok_or(PathError::Invalid(format!(
        //         "Failed to convert parent dir to str: {}",
        //         full_path
        //     )))?;
        //
        // let components: Vec<Option<&str>> =
        //     path.components().map(|c| c.as_os_str().to_str()).collect();

        let full_path = full_path.to_lowercase();
        let path = PathBuf::from(&full_path);
        let (path_stem, path_stem_pos, path_extension, path_extension_position) =
            split_rev(&full_path, '/', '.').ok_or(PathError::Invalid(format!(
                "Failed to get file name and extension from '{}'",
                &full_path
            )))?;
        let path_name = &full_path[path_stem_pos..];
        let path_dir = &full_path[..path_stem_pos];

        let (cat, repo) = split(&full_path, '/', '/');

        let (cat, cat_pos) = cat.ok_or(PathError::General(format!(
            "Failed to get category from '{}'",
            &full_path
        )))?;
        let data_category = Category::from_str(cat).or_else(|e| {
            Err(PathError::General(format!(
                "{} from '{}'",
                e.to_string(),
                &full_path
            )))
        })?;

        let data_repo = if let Some((rep, rep_pos)) = repo {
            Repository::from_str(rep)
        } else {
            Repository::default()
        };

        let data_folder = DatPath::hash(path_dir);
        //let data_category_hash = AssetPath::hash(data_category.name.as_str());
        let data_name_hash = DatPath::hash(path_name);
        let data_platform = Platform::from_u32(0).or_else(|e| {
            Err(PathError::General(format!(
                "{} from '{}'",
                e.to_string(),
                &full_path
            )))
        })?;

        let index1_hash = DatPath::double_hash(data_folder, data_name_hash);
        let index2_hash = DatPath::hash(&full_path);

        Ok(DatPath {
            path: path.clone(),
            path_str: full_path.to_owned(),
            path_extension: String::from(path_extension),
            path_stem: String::from(path_stem),
            path_dir: String::from(path_dir),
            path_name: String::from(path_name),
            index1_hash,
            index2_hash,
            data_repo,
            data_category,
            data_platform,
        })
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
    pub fn new(file_path: PathBuf) -> Result<FilePath, PathError> {
        let file_path_str = file_path
            .as_os_str()
            .to_str()
            .ok_or(PathError::Invalid(format!("Failed to convert path to str")))?;
        let file_name = file_path
            .file_name()
            .ok_or(PathError::Invalid(format!(
                "Failed to get file name from: {}",
                file_path_str
            )))?
            .to_str()
            .ok_or(PathError::Invalid(format!(
                "Failed to get file name as str: {}",
                file_path_str
            )))?;
        let file_stem = file_path
            .file_stem()
            .ok_or(PathError::Invalid(format!(
                "Failed to get file name: {}",
                file_path_str
            )))?
            .to_str()
            .ok_or(PathError::Invalid(format!(
                "Failed to get file name as str."
            )))?;
        let file_extension = file_path
            .extension()
            .ok_or(PathError::Invalid(format!("Failed to get file extension.")))?
            .to_str()
            .ok_or(PathError::Invalid(format!(
                "Failed to get file extension as str."
            )))?;
        //let file_path_components: Vec<Option<&str>> = file_path.components().map(|c| c.as_os_str().to_str()).collect();

        let file_name_bytes = file_name.as_bytes();
        let file_name_regex = RegByte::new(r"^(\d|[a-z]){6}\.(win32|ps3|ps4)\.(dat|index)\d*$")
            .or(Err(PathError::General(String::from(
                "Failed to create regex",
            ))))?;
        file_name_regex
            .captures(file_name_bytes)
            .ok_or(PathError::Invalid(format!(
                "File name '{}' is invalid.",
                file_name
            )))?;

        let category_str = String::from_utf8(file_name_bytes[0..2].to_vec()).or(Err(
            PathError::General(format!("Failed to slice name to category: {}", file_stem)),
        ))?;
        let repository_str = String::from_utf8(file_name_bytes[2..4].to_vec()).or(Err(
            PathError::General(format!("Failed to slice name to repository: {}", file_stem)),
        ))?;
        let chunk_str = String::from_utf8(file_name_bytes[4..6].to_vec()).or(Err(
            PathError::General(format!("Failed to slice name to chunk: {}", file_stem)),
        ))?;

        let data_category = Category::from_hex_str(&category_str).or_else(|e| {
            Err(PathError::General(format!(
                "{} from {}",
                e.to_string(),
                file_path_str
            )))
        })?;
        let data_repository = Repository::from_hex_str(&repository_str).or_else(|e| {
            Err(PathError::General(format!(
                "{} from {}",
                e.to_string(),
                file_path_str
            )))
        })?;
        let data_chunk = Chunk::from_hex_str(&chunk_str).or_else(|e| {
            Err(PathError::General(format!(
                "{} from {}",
                e.to_string(),
                file_path_str
            )))
        })?;
        let data_platform = Platform::from_str_contains(&file_name).or_else(|e| {
            Err(PathError::General(format!(
                "{} from {}",
                e.to_string(),
                file_path_str
            )))
        })?;

        Ok(FilePath {
            path_str: String::from(file_path_str),
            path: file_path.clone(),
            path_name: String::from(file_name),
            path_stem: String::from(file_stem),
            path_extension: String::from(file_extension),
            data_category,
            data_repository,
            data_chunk,
            data_platform,
        })
    }
}

//==================================================================================================

#[derive(Error, Debug)]
pub enum PathError {
    #[error("File Path: `{0}`")]
    Invalid(String),

    // #[error("Failed to parse metadata: `{0}`")]
    // Metadata(#[from] MetadataError),
    #[error("File Path Regex: `{0}`")]
    General(String),
}

pub fn split<'a>(
    str: &'a str,
    split_b: char,
    split_a: char,
) -> (Option<(&'a str, usize)>, Option<(&'a str, usize)>) {
    let len = str.len();
    let pos_a = str.chars().position(|c| c == split_a);
    if let Some(pos_a) = pos_a {
        let pos_b = str.chars().skip(pos_a + 1).position(|c| c == split_b);
        let a: &'a str = &str[..pos_a];
        if let Some(pos_b) = pos_b {
            let pos_b: usize = pos_b + pos_a;
            let b: &'a str = &str[pos_a + 1..pos_b + 1];
            return (Some((a, pos_a)), Some((b, pos_b)));
        } else {
            let a: &'a str = &str[..pos_a];
            return (Some((a, pos_a)), None);
        }
    } else {
        return (None, None);
    }
}

pub fn split_rev<'a>(
    str: &'a str,
    split_b: char,
    split_a: char,
) -> Option<(&'a str, usize, &'a str, usize)> {
    let len = str.len();
    let pos_a = str.chars().rev().position(|c| c == split_a)?;
    let pos_b = str
        .chars()
        .rev()
        .skip(pos_a + 1)
        .position(|c| c == split_b)?;
    let pos_b: usize = len - pos_a - 1 - pos_b;
    let pos_a: usize = len - pos_a;
    let a: &'a str = &str[pos_a..];
    let b: &'a str = &str[pos_b..pos_a - 1];
    Some((b, pos_b, a, pos_a))
}
