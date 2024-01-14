use std::fs;
use std::path::{Path, PathBuf};
use crate::ffxiv::parser::ffxiv_data::FFXIVAssetFiles;

use crate::ffxiv::parser::ffxiv_data::metadata::FFXIVFileMetadata;

mod decoder;
mod parser;
mod reader;

pub struct FFXIV {
    asset_files: Vec<FFXIVAssetFiles>
}

impl FFXIV {
    pub fn new(game_path: &str) -> FFXIV {
        let asset_files: Vec<FFXIVAssetFiles> = FFXIVAssetFiles::new(game_path);

        FFXIV {
            asset_files
        }
    }


}
