use std::fmt::Error;

use std::path::{Path, PathBuf};
use std::ptr::hash;

#[derive(Debug)]
pub struct Data {
    pub repository: String,
    pub category: String,
    pub data_path: String,
    pub data_name_hash: u32,
    pub data_category_hash: u32,
    pub index_hash: u64,
    pub index_hash2: u32,
    // pub file_offset: u32,
    // pub file_path: PathBuf

}

impl Data {
    // pub fn new(full_path: &str) -> Result<Data, String> {
    //
    //
    // }


}



// pub fn crc32() {
//
// }