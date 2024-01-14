use std::fs;
use std::path::{Path, PathBuf};
use crate::ffxiv::ffxiv_asset::{FFXIVAssetParserDat, FFXIVAssetParserIndex, FFXIVAssetPathDat, FFXIVAssetPathFile};

pub struct FFXIVGame {
    game_path: String,
    asset_files: Vec<FFXIVAssetFiles>
}

impl FFXIVGame {
    pub fn new(game_path: &str) -> FFXIVGame {
        let asset_files: Vec<FFXIVAssetFiles> = FFXIVAssetFiles::new(&game_path).unwrap();
        FFXIVGame {
            game_path: String::from(game_path),
            asset_files
        }
    }

    pub fn get_asset_from_dat(&self, dat_path: &str) -> Option<FFXIVAssetParserDat> {
        let path_dat = FFXIVAssetPathDat::from_str(dat_path).unwrap();
        let possible_asset_files = self.find_asset_files_by_asset_path(&path_dat);

        for possible_asset_file in possible_asset_files {
            let index_asset = FFXIVAssetParserIndex::from_index1_file(&possible_asset_file.index_file.path);
            if let Some(item) = index_asset.find(path_dat.index1_hash) {
                let dat_file = FFXIVAssetParserDat::from_dat_files(&possible_asset_file.dat_files, item.data_file_id, item.data_file_offset);
                return Some(dat_file);
            }
        };

        None
    }

    fn find_asset_files_by_asset_path(&self, asset_path: &FFXIVAssetPathDat) -> Vec<&FFXIVAssetFiles>{
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



pub struct FFXIVAssetFiles {
    pub dat_files: Vec<FFXIVAssetPathFile>,
    pub index_file: FFXIVAssetPathFile,
    pub index2_file: FFXIVAssetPathFile,
}

impl FFXIVAssetFiles {

    pub fn new(game_path: &str) -> Result<Vec<FFXIVAssetFiles>, String> {
        let mut file_paths: Vec<PathBuf> = Vec::new();
        FFXIVAssetFiles::get_files(game_path, &mut file_paths);

        let mut dat_files: Vec<FFXIVAssetPathFile> = Vec::new();
        let mut index_files: Vec<FFXIVAssetPathFile> = Vec::new();
        let mut index2_files: Vec<FFXIVAssetPathFile> = Vec::new();

        for file_path in file_paths {
            let file_metadata = FFXIVAssetPathFile::new(file_path);
            if let Ok(file_metadata) = file_metadata {
                if file_metadata.path_extension == "index" {
                    index_files.push(file_metadata);
                } else if file_metadata.path_extension == "index2" {
                    index2_files.push(file_metadata);
                } else {
                    dat_files.push(file_metadata);
                }
            }
        }


        let grouped_files = FFXIVAssetFiles::group_files(dat_files, index_files, index2_files);

        Ok(grouped_files)
    }

    fn group_files(dat_files: Vec<FFXIVAssetPathFile>, index_files: Vec<FFXIVAssetPathFile>, index2_files: Vec<FFXIVAssetPathFile>) -> Vec<FFXIVAssetFiles> {
        let mut file_groups: Vec<FFXIVAssetFiles> = Vec::new();

        for index_file in index_files {
            let index2_file = index2_files.iter().find(|f| **f == index_file);
            if let Some(index2_file) = index2_file {
                let dat_files: Vec<FFXIVAssetPathFile> = dat_files.iter().filter(|f| **f == index_file).map(|f| f.clone()).collect();
                if dat_files.len() == 0 {
                    continue;
                }


                file_groups.push(FFXIVAssetFiles {
                    index_file,
                    index2_file: (*index2_file).clone(),
                    dat_files,

                })
            }
        }
        file_groups
    }

    fn get_files(input_path: &str, output: &mut Vec<PathBuf>) {
        let verify = Path::new(input_path);
        let flag = verify.is_dir();
        if !flag {
            panic!("Path is not a directory: {}", input_path)
        }
        let flag = verify.exists();
        if !flag {
            panic!("Path doesn't not exist: {}", input_path)
        }
        let paths = fs::read_dir(input_path).unwrap();
        for path in paths {
            let path = path.unwrap().path();
            if path.is_file() {
                output.push(path)
            } else {
                FFXIVAssetFiles::get_files(path.to_str().unwrap(), output);
            }
        }
    }
}
