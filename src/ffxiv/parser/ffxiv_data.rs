pub mod assets;
pub mod metadata;

use std::path::PathBuf;
use crate::ffxiv::parser::ffxiv_data::assets::index::{Index, Index1Data1Item, Index2Data1Item};
use crate::ffxiv::parser::ffxiv_data::metadata::FFXIVFileMetadata;

pub enum ItemType {
    SCD(),
    Unsupported
}

pub struct FFXIVData {
    metadata: FFXIVFileMetadata,
    dat_file_path: PathBuf,
    parsed_index_file: Index<Index1Data1Item>,
    parsed_index2_file: Index<Index2Data1Item>,
}

pub struct FFXIVIndexFile {
    path: PathBuf,
    items: String, //parsed
}

pub struct FFXIVIndex2File {
    path: PathBuf,
    items: String, //parsed
}

pub impl FFXIVData {
    
}