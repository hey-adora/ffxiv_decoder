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

pub fn test(game_path: &str, index_path: &str) {
    let parsed_index_path = AssetPath::from_str(index_path).unwrap();

    let mut possible_asset_files: Vec<FFXIVAssetFiles> = Vec::new();

    let asset_files: Vec<FFXIVAssetFiles> = FFXIVAssetFiles::new(game_path).unwrap();

    for asset_file in asset_files {
        if asset_file.index_file.data_category.id == parsed_index_path.data_category.id &&
            asset_file.index_file.data_repository.id == parsed_index_path.data_repo.id {
            possible_asset_files.push(asset_file);
        }
    }


    for possible_asset_file in possible_asset_files {
        let index1_file_handle = fs::read(possible_asset_file.index_file.file_path).unwrap();
        let mut index1_file = BufferVec::new(index1_file_handle);
        let parsed_index1 = AssetIndexFile::from_index1(&mut index1_file);
        let index1_item = parsed_index1.data1.iter().find(|item| item.hash == parsed_index_path.index1_hash);
        if let Some(item) = index1_item {
            let data_item = possible_asset_file.dat_files.iter().find(|d| d.data_chunk.id == item.data_file_id).ok_or("Data file could not be found.").unwrap();
            let data_item_path = data_item.file_path.as_os_str().to_str().unwrap();
            let mut data_file = BufferFile::from_file_path(data_item_path);

            let asset_dat_file = AssetDatFile::new(&mut data_file, item.data_file_offset);
            let save_file = SaveFilePath::from_index_path(&parsed_index_path);
            save_file.write_blocks(asset_dat_file.to_decompressed());
        }
    }
}
