use std::collections::HashMap;
use std::fs;
use std::fs::create_dir_all;
use std::path::PathBuf;
use crate::ffxiv::asset_dat_file::AssetDatFile;
use crate::ffxiv::asset_exd_file::AssetEXDFile;
use crate::ffxiv::asset_exh_file::{AssetEXHFileColumnKind, AssetEXHFileLanguage};
use crate::ffxiv::asset_files::FFXIVAssetFiles;
use crate::ffxiv::asset_index_file::{AssetIndexFile, Index1Data1Item};
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
pub mod ffxiv_asset;
pub mod ffxiv_game;
pub mod save_file;
pub mod asset_exh_file;
pub mod asset_exd_file;
pub mod ffxiv_buffer;

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

    pub fn save_all (&self) {
        let hashes = self.get_hashes();

        let max_index: f32 = hashes.len() as f32;
        let check_every: f32 = (max_index / 100.0).floor();
        for (index, (hash, (path, item))) in hashes.iter().enumerate() {
            let dat_file = AssetDatFile::from_file_path(&path, item.data_file_offset);
            dat_file.save_decompressed(&format!("./data/{}", hash));

            let index = index as f32;
            if index % check_every == 0.0 {
                let done = (index / max_index) * 100.0;
                println!("Extracting data: {}%.\n", done);
            }
        }
    }

    pub fn get_hashes(&self) -> HashMap<u64, (String, Index1Data1Item)> {
        let mut index1_hashes: HashMap<u64, (String, Index1Data1Item)> = HashMap::new();
        //let index2_hashes: HashMap<u64, String> = HashMap::new();

        let max_index: f32 = self.asset_files.len() as f32;
        let check_every: f32 = (max_index / 100.0).floor();
        for (index, asset_file) in self.asset_files.iter().enumerate() {
            let parsed_index = AssetIndexFile::from_index1_file(&asset_file.index_file.file_path);
            for data in parsed_index.data1 {
                index1_hashes.insert(data.hash, (asset_file.dat_files.iter().find(|f| f.file_extension == format!("dat{}", data.data_file_id)).unwrap().file_path_str.clone(), data));
            }

            let index = index as f32;
            if index % check_every == 0.0 {
                let done = (index / max_index) * 100.0;
                println!("Parsing path: {}%.\n", done);
            }
        }

        index1_hashes
    }

    pub fn save_all_cvs(&self) {
        let exl =  self.get_asset_from_path("exd/root.exl").unwrap().to_exl();
        for (name, ukwn) in exl {
            let name = name.to_lowercase();
            let asset_path = &format!("exd/{}.exh", name);
            let exh =  self.get_asset_from_path(asset_path);

            if let Some(exh) = exh {

                let exh = exh.to_exh();
                let exd_lang_prefix = match &exh.languages[0] {
                    AssetEXHFileLanguage::None => String::from(".exd"),
                    _ => String::from("_en.exd")
                };

                for row in &exh.rows{
                    let file_path_str = format!("./csvs/{}_{}_en.csv", name, &row.start_id);
                    let file_path_buf = PathBuf::from(&file_path_str);
                    let file_path_dir = file_path_buf.parent().unwrap();

                    if !file_path_buf.exists() {
                        let mut rows: String = exh.columns.iter().map(|c|AssetEXHFileColumnKind::names(&c.kind)).collect::<Vec<String>>().join(",");
                        rows.push_str("\n\n");

                        let exd_asset_path = &format!("exd/{}_{}{}", name, &row.start_id, exd_lang_prefix);
                        let exd =  self.get_asset_from_path(exd_asset_path).unwrap().to_exd(&exh);

                        for row in &exd.rows {
                            let row: String = row.iter().map(|c| {
                                let value = c.to_string();
                                if value.len() > 0 {
                                    return value;
                                } else {
                                    return String::from("EMPTY");
                                }
                            }).collect::<Vec<String>>().join(",");
                            rows.push('\n');
                            rows.push_str(&row)
                        }

                        create_dir_all(file_path_dir).unwrap();
                        fs::write(file_path_buf, rows).unwrap();
                        println!("Saved {}", file_path_str);
                    } else {
                        println!("Skipped: {}", asset_path)
                    }
                }
            } else {
                println!("Not found: {}", asset_path)
            }
        }
    }




}
