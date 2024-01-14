use std::fs;
use std::path::{Path, PathBuf};
use crate::ffxiv::asset_file_path::AssetFilePath;

pub struct FFXIVAssetFiles {
    pub dat_files: Vec<AssetFilePath>,
    pub index_file: AssetFilePath,
    pub index2_file: AssetFilePath,
}

impl FFXIVAssetFiles {
    pub fn new(game_path: &str) -> Result<Vec<FFXIVAssetFiles>, String> {
        let mut file_paths: Vec<PathBuf> = Vec::new();
        FFXIVAssetFiles::get_files(game_path, &mut file_paths);

        let debug: Vec<&str> = file_paths.iter().map(|f| { f.to_str().unwrap() }).collect();

        let mut dat_files: Vec<AssetFilePath> = Vec::new();
        let mut index_files: Vec<AssetFilePath> = Vec::new();
        let mut index2_files: Vec<AssetFilePath> = Vec::new();

        for file_path in file_paths {
            let file_metadata = AssetFilePath::new(file_path);
            if let Ok(file_metadata) = file_metadata {
                if file_metadata.file_extension == "index" {
                    index_files.push(file_metadata);
                } else if file_metadata.file_extension == "index2" {
                    index2_files.push(file_metadata);
                } else {
                    dat_files.push(file_metadata);
                }
            }
        }

        let mut file_groups: Vec<FFXIVAssetFiles> = Vec::new();

        for index_file in index_files {
            let index2_file = index2_files.iter().find(|f| **f == index_file);
            if let Some(index2_file) = index2_file {
                let dat_files: Vec<AssetFilePath> = dat_files.iter().filter(|f| **f == index_file).map(|f| f.clone()).collect();
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


        Ok(file_groups)
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
