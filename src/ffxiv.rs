use std::fs;
use std::path::{Path, PathBuf};
use crate::ffxiv::parser::ffxiv_data::FFXIVData;

mod decoder;
mod parser;
mod reader;

pub struct FFXIV {
    assets: Vec<FFXIVData>
}

impl FFXIV {
    pub fn new(game_path: &str) -> Vec<FFXIVData> {
        let mut file_paths: Vec<PathBuf> = Vec::new();
        FFXIV::get_files(game_path, &mut file_paths);

        let assets: Vec<FFXIVData> = Vec::new();



        assets
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
                FFXIV::get_files(path.to_str().unwrap(), output);
            }
        }
    }
}
