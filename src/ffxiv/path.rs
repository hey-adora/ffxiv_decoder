use std::fmt::{Display, Formatter};
use std::io::ErrorKind;
use std::path::PathBuf;
use crc::{Crc, CRC_32_JAMCRC};
use egui::Key::N;
use regex::bytes::Regex as RegByte;
use crate::ffxiv::category::{Category, CategoryError};
use crate::ffxiv::chunk::Chunk;
use crate::ffxiv::platform::Platform;
use crate::ffxiv::repository::Repository;
use thiserror::Error;

// macro_rules! path_impl {
//     ($name: tt, $input_t: ty, $output_t: ty , $val: expr) => {
//         pub fn $name(&mut self) -> Result<&$output_t, PathError> {
//             if let None = &self.$name {
//                 let val: $input_t = $val(self)?;
//                 self.$name = Some(val.clone());
//                 if let Some(val) = &self.$name {
//                     return Ok(val);
//                 }
//             } else if let Some(val) = &self.$name {
//                 return Ok(val);
//             }
//             return Err(PathError::Invalid("Failed to set value".to_owned()));
//         }
//     }
// }
//
// macro_rules! path_asset_impl {
//     ($name: tt, $input_t: ty, $output_t: ty, $val: expr) => {
//         pub fn $name(&mut self) -> Result<&$output_t, PathError> {
//             if let Some(val) = &self.asset.$name {
//                 return Ok(val);
//             } else {
//                 let val: $input_t = $val(self)?;
//                 self.asset.$name = Some(val.clone());
//                 if let Some(val) = &self.asset.$name {
//                     return Ok(val);
//                 }
//                 return Err(PathError::Invalid("Failed to set value".to_owned()));
//             }
//         }
//     }
// }
//
// macro_rules! cache_impl {
//     ($name: tt, $input_t: ty, $output_t: ty, $val: expr) => {
//         pub fn $name(&mut self) -> Result<&$output_t, PathError> {
//             if let Some(val) = &self.asset.$name {
//                 return Ok(val);
//             } else {
//                 let val: $input_t = $val(self)?;
//                 self.asset.$name = Some(val.clone());
//                 if let Some(val) = &self.asset.$name {
//                     return Ok(val);
//                 }
//                 return Err(PathError::Invalid("Failed to set value".to_owned()));
//             }
//         }
//     }
// }

#[derive(Debug, Clone)]
pub struct AssetPath<T: Default> {
    pub path: String,
    buf: Option<PathBuf>,
    name: Option<String>,
    name_bytes: Option<Vec<u8>>,
    stem: Option<String>,
    extension: Option<String>,
    asset: T
}

impl <T: Default> PartialEq  for AssetPath<T> {
    fn eq(&self, other: &Self) -> bool {
        self.stem == other.stem
    }
}

impl <T: Default> AssetPath<T> {



    pub fn buf(&mut self) -> &PathBuf {
        if let None = &self.buf {
            self.buf = Some(PathBuf::new());
            if let Some(buf) = &self.buf {
                return buf;
            }
        } else if let Some(buf) = &self.buf {
            return buf;
        }
        panic!("Failed to set path buffer");
    }

    pub fn validate(&mut self) -> Result<(), PathError> {
        let file_name_regex = RegByte::new(r"^(\d|[a-z]){6}\.(win32|ps3|ps4)\.(dat|index)\d*$").or(Err(PathError::Invalid("Failed to create validation regex".to_owned())))?;
        file_name_regex.captures(&self.name_bytes()?).ok_or(PathError::Invalid(format!("File name '{}' is invalid.", self.name()?)))?;
        Ok(())
    }
    //
    // pub fn nnn(&mut self) -> Result<&str, PathError> {
    //     if let Some(val) = &self.name {
    //         return Ok(val);
    //     } else {
    //     let val: String = String::new();
    //     self.name = Some(val.clone());
    //     if let Some(val) = &self.name {
    //         return Ok(val);
    //     }
    //         return Err(PathError::Invalid("Failed to set value".to_owned()));
    //     }
    // }

    pub fn name(&mut self) -> Result<&str, PathError> {
        if let None = &self.name {
            let ame = self.buf().file_name();
            let a = self.stem()?;
            let i = format!("Failed to get file_name from '{}'", a);
            let val: String =
                ame.ok_or(PathError::Invalid(i))?.to_str()
                .ok_or(PathError::Invalid(format!("Failed to_str from '{}' as str", self.path)))?.to_string();
            self.name = Some(val);
        }
        if let Some(val) = &self.name {
            return Ok(val);
        }
        return Err(PathError::Invalid("Failed to set value".to_owned()));
    }

    pub fn stem(&mut self) -> Result<&str, PathError> {
        if let None = &self.stem {
            let val: String = self.buf().file_stem()
                .ok_or(PathError::Invalid(format!("Failed to get file_stem from '{}'", self.path)))?.to_str()
                .ok_or(PathError::Invalid(format!("Failed to_str from '{}' as str", self.path)))?.to_string();
            self.stem = Some(val);
        }
        if let Some(val) = &self.stem {
            return Ok(val);
        }
        return Err(PathError::Invalid("Failed to set value".to_owned()));
    }

    pub fn extension(&mut self) -> Result<&str, PathError> {
        if let None = &self.extension {
            let val: String = self.buf().extension()
                .ok_or(PathError::Invalid(format!("Failed to get file_stem from '{}'", self.path)))?.to_str()
                .ok_or(PathError::Invalid(format!("Failed to_str from '{}' as str", self.path)))?.to_string();
            self.extension = Some(val);
        }
        if let Some(val) = &self.extension {
            return Ok(val);
        }
        return Err(PathError::Invalid("Failed to set value".to_owned()));
    }

    pub fn name_bytes(&mut self) -> Result<&[u8], PathError> {
        if let None = &self.name_bytes {
            let val: Vec<u8> = self.name()?.as_bytes().to_owned();
            self.name_bytes = Some(val);
        }
        if let Some(val) = &self.name_bytes {
            return Ok(val);
        }
        return Err(PathError::Invalid("Failed to set value".to_owned()));
    }
}

//==================================================================================================

#[derive(Debug, Clone)]
pub struct IndexPath {
    category: Option<Category>,
    repository: Option<Repository>,
    chunk: Option<Chunk>,
    platform: Option<Platform>,
}

impl AssetPath<IndexPath> {
    pub fn new(path: String) -> AssetPath<IndexPath> {
        AssetPath {
            path,
            buf: None,
            name: None,
            name_bytes: None,
            stem: None,
            extension: None,
            asset: IndexPath::default()
        }
    }

    // path_asset_impl!(category, Category, Category, |s: &mut AssetPath<IndexPath>| -> Result<Category, PathError> {
    //     let str = String::from_utf8(s.name_bytes()?[0..2].to_owned()).or(Err(PathError::Invalid(format!("Failed to get category from '{}'", s.path))))?;
    //     let val = Category::from_hex_str(&str).or_else(|e| Err(PathError::Invalid(e.to_string())))?;
    //     Ok(val)
    // });
    //
    // path_asset_impl!(repository, Repository, Repository, |s: &mut AssetPath<IndexPath>| -> Result<Repository, PathError> {
    //     let str = String::from_utf8(s.name_bytes()?[2..4].to_owned()).or(Err(PathError::Invalid(format!("Failed to get repository from '{}'", s.path))))?;
    //     let val = Repository::from_hex_str(&str).or_else(|e| Err(PathError::Invalid(e.to_string())))?;
    //     Ok(val)
    // });
    //
    // path_asset_impl!(chunk, Chunk, Chunk, |s: &mut AssetPath<IndexPath>| -> Result<Chunk, PathError> {
    //     let str = String::from_utf8(s.name_bytes()?[4..6].to_owned()).or(Err(PathError::Invalid(format!("Failed to get chunk from '{}'", s.path))))?;
    //     let val = Chunk::from_hex_str(&str).or_else(|e| Err(PathError::Invalid(e.to_string())))?;
    //     Ok(val)
    // });
    //
    // path_asset_impl!(platform, Platform, Platform, |s: &mut AssetPath<IndexPath>| -> Result<Platform, PathError> {
    //     let val = Platform::from_str_contains(&s.name()?).or_else(|e| Err(PathError::Invalid(e.to_string())))?;
    //     Ok(val)
    // });
}

impl Default for IndexPath {
    fn default() -> Self {
        Self {
            category: None,
            repository: None,
            chunk: None,
            platform: None
        }
    }
}

//==================================================================================================

#[derive(Debug, Clone)]
pub struct DatPath {
    dir: Option<String>,
    dir_hash: Option<u32>,
    name_hash: Option<u32>,
    index1_hash: Option<u64>,
    index2_hash: Option<u32>,
    repository: Option<Repository>,
    category: Option<Category>,
    platform: Option<Platform>,
    components: Option<Vec<String>>,
}

impl Default for DatPath {
    fn default() -> Self {
        Self {
            dir: None,
            dir_hash: None,
            name_hash: None,
            index1_hash: None,
            index2_hash: None,
            repository: None,
            category: None,
            platform: None,
            components: None
        }
    }
}

impl AssetPath<DatPath> {

    pub fn new(path: String) -> AssetPath<DatPath> {
        AssetPath {
            path: path.to_lowercase(),
            buf: None,
            name: None,
            name_bytes: None,
            stem: None,
            extension: None,
            asset: DatPath::default()
        }
    }

    // path_asset_impl!(components, Vec<String>, Vec<String>, |s: &mut AssetPath<DatPath>| -> Result<Vec<String>, PathError> {
    //     let mut components: Vec<String> = Vec::new();
    //     for c in s.buf().components() {
    //         let component = c.as_os_str().to_str().ok_or(PathError::Invalid(format!("Failed to convert parent dir to str.")))?.to_owned();
    //         components.push(component);
    //     }
    //     Ok(components)
    // });
    //
    // path_asset_impl!(dir, String, str, |s: &mut AssetPath<DatPath>| -> Result<String, PathError> {
    //     Ok(s.buf().parent().ok_or(PathError::Invalid(format!("Failed to get parent dir.")))?.to_str().ok_or(PathError::Invalid(format!("Failed to convert parent dir to str.")))?.to_owned())
    // });
    //
    // path_asset_impl!(dir_hash, u32, u32, |s: &mut AssetPath<DatPath>| -> Result<u32, PathError> {
    //     let dir = s.dir()?;
    //     let bytes = dir.as_bytes();
    //     Ok(s.hash(bytes))
    // });
    //
    // path_asset_impl!(name_hash, u32, u32, |s: &mut AssetPath<DatPath>| -> Result<u32, PathError> {
    //     let name = s.name()?;
    //     let bytes = name.as_bytes();
    //     Ok(s.hash(bytes))
    // });
    //
    // path_asset_impl!(index1_hash, u64, u64, |s: &mut AssetPath<DatPath>| -> Result<u64, PathError> {
    //     let dir_hash = s.dir_hash()?.to_owned();
    //     let name_hash = s.name_hash()?.to_owned();
    //     Ok(s.double_hash(dir_hash, name_hash))
    // });
    //
    // path_asset_impl!(index2_hash, u32, u32, |s: &mut AssetPath<DatPath>| -> Result<u32, PathError> {
    //     let path = s.path.clone();
    //     let path = path.as_bytes();
    //     let hash = s.hash(path);
    //     Ok(hash)
    // });
    //
    // fn hash(&mut self, string: &[u8]) -> u32 {
    //     let crc = Crc::<u32>::new(&CRC_32_JAMCRC);
    //     let mut digest = crc.digest();
    //     digest.update(string);
    //     digest.finalize()
    // }
    //
    // fn double_hash(&mut self, category_hash: u32, file_name_hash: u32) -> u64 {
    //     return ((category_hash as u64) << 32) | (file_name_hash as u64);
    // }
}

//==================================================================================================

#[derive(Error, Debug)]
pub enum PathError {
    #[error("Asset Path: `{0}`")]
    Invalid(String),
}
