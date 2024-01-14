pub mod assets;
pub mod metadata;

use std::fs;
use std::path::{Path, PathBuf};
use crate::ffxiv::parser::ffxiv_data::assets::index::{Index, Index1Data1Item, Index2Data1Item};
use crate::ffxiv::parser::ffxiv_data::metadata::FFXIVFileMetadata;
use crate::ffxiv::reader::buffer::BufferWithLog;

pub enum ItemType {
    SCD(),
    Unsupported
}


pub struct FFXIVAssetFiles {
    pub dat_files: Vec<FFXIVFileMetadata>,
    pub index_file: FFXIVFileMetadata,
    pub index2_file: FFXIVFileMetadata,
    pub parsed_index: Index<Index1Data1Item>,
    pub parsed_index2: Index<Index2Data1Item>,
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

impl FFXIVAssetFiles {
    pub fn new(game_path: &str) -> Result<Vec<FFXIVAssetFiles>, String> {
        let mut file_paths: Vec<PathBuf> = Vec::new();
        FFXIVAssetFiles::get_files(game_path, &mut file_paths);

        let mut dat_files: Vec<FFXIVFileMetadata> = Vec::new();
        let mut index_files: Vec<FFXIVFileMetadata> = Vec::new();
        let mut index2_files: Vec<FFXIVFileMetadata> = Vec::new();

        for file_path in file_paths {
            let file_metadata = FFXIVFileMetadata::new(file_path);
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
                let dat_files: Vec<FFXIVFileMetadata> = dat_files.iter().filter(|f| **f == index_file).map(|f|f.clone()).collect();
                if dat_files.len() == 0 {
                    continue;
                }

                let file = fs::read(&index_file.file_path).or(Err(format!("Error reading file {}", &index_file.file_path_str)))?;
                let mut file_buffer = BufferWithLog::new(file);
                let parsed_index = Index::from_index1(&mut file_buffer);

                let file = fs::read(&index2_file.file_path).or(Err(format!("Error reading file {}", &index2_file.file_path_str)))?;
                let mut file_buffer = BufferWithLog::new(file);
                let parsed_index2 = Index::from_index2(&mut file_buffer);

                file_groups.push( FFXIVAssetFiles {
                    index_file,
                    index2_file: (*index2_file).clone(),
                    dat_files,
                    parsed_index,
                    parsed_index2
                } )
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

//
// impl FFXIVData {
//
// }