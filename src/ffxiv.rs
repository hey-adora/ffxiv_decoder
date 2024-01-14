use std::fs;
use crate::ffxiv::asset_dat_file::AssetDatFile;
use crate::ffxiv::asset_files::FFXIVAssetFiles;
use crate::ffxiv::asset_index_file::AssetIndexFile;
use crate::ffxiv::asset_path::AssetPath;
use crate::ffxiv::buffer_file::BufferFile;
use crate::ffxiv::buffer_vec::BufferVec;
use crate::ffxiv::save_file::SaveFilePath;

pub mod asset_files;
pub mod asset_file_path;
pub mod asset_path;
pub mod asset_index_file;
pub mod asset_file_category;
pub mod asset_file_repository;
pub mod asset_file_platform;
pub mod asset_file_chunk;
pub mod buffer_file;
pub mod buffer_vec;
pub mod asset_scd_file;
pub mod asset_dat_file;
pub mod asset;
pub mod game;
pub mod save_file;

pub struct FFXIV {
    game_path: String,
    asset_files: Vec<FFXIVAssetFiles>,
}

impl FFXIV {
    pub fn new(game_path: &str) -> FFXIV {
        let asset_files: Vec<FFXIVAssetFiles> = FFXIVAssetFiles::new(game_path).unwrap();
        FFXIV {
            asset_files,
            game_path: String::from(game_path)
        }
    }

    pub fn get_asset_from_path(&self, asset_path: &str) -> Option<AssetDatFile> {
        let asset_path = AssetPath::from_str(asset_path).unwrap();
        let possible_asset_files = self.find_asset_files_by_asset_path(&asset_path);

        for possible_asset_file in possible_asset_files {
            let index_asset = AssetIndexFile::from_index1_file(&possible_asset_file.index_file.file_path);
            if let Some(item) = index_asset.find(asset_path.index1_hash) {
                let dat_file = AssetDatFile::from_dat_files(&possible_asset_file.dat_files, item.data_file_id, item.data_file_offset);
                return Some(dat_file);
            }
        };

        None
    }

    pub fn find_asset_files_by_asset_path(&self, asset_path: &AssetPath) -> Vec<&FFXIVAssetFiles>{
        let mut possible_asset_files: Vec<&FFXIVAssetFiles> = Vec::new();

        for asset_file in &self.asset_files {
            if asset_file.index_file.data_category.id == asset_path.data_category.id &&
                asset_file.index_file.data_repository.id == asset_path.data_repo.id {
                possible_asset_files.push(asset_file.clone());
            }
        }

        possible_asset_files
    }
}
