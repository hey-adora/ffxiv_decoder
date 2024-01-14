pub mod assets;
pub mod metadata;

use std::fs;
use std::path::{Path, PathBuf};
use crate::ffxiv::parser::ffxiv_data::assets::index::{Index, Index1Data1Item, Index2Data1Item};
use crate::ffxiv::parser::ffxiv_data::metadata::FFXIVFileMetadata;
use crate::ffxiv::reader::buffer_with_log::BufferWithLog;

pub enum ItemType {
    SCD(),
    Unsupported
}







pub struct FFXIVFile<T> {
    metadata: FFXIVFileMetadata,
    data: T
}

pub struct FFXIVIndexFile {
    path: PathBuf,
    items: String, //parsed
}

pub struct FFXIVIndex2File {
    path: PathBuf,
    items: String, //parsed
}


//
// impl FFXIVData {
//
// }