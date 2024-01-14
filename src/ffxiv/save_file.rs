use std::fs::{self, create_dir_all, File};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use crate::ffxiv::asset_dat_file::{AssetDatFile};
use crate::ffxiv::asset_path::AssetPath;
use crate::ffxiv::buffer_file::BufferFile;

pub struct SaveFilePath {
    pub file_path_str: String,
    pub file_path_buf: PathBuf,
    pub exists: bool,
}

impl SaveFilePath {
    pub fn from_index_path(index_path: &AssetPath) -> SaveFilePath {
        //let file_stem = format!("{}_{}", index_path.index1_hash, index_path.file_stem);
        let file_path_str = format!("./media/here/{}_{}.{}", index_path.index1_hash, index_path.file_stem, index_path.file_extension);
        let file_path_buf = PathBuf::from(&file_path_str);
        let exists = file_path_buf.exists();
        SaveFilePath {
            file_path_str,
            file_path_buf,
            exists,
        }
    }


    pub fn write_blocks(&self, blocks: Vec<Vec<u8>>) {
        let dir = self.file_path_buf.parent().unwrap();
        create_dir_all(dir).unwrap();
        let mut scd_file = File::create(&self.file_path_buf).unwrap();
        for decompressed_block in blocks {
            scd_file.write_all(&decompressed_block).unwrap();
        }
    }

    pub fn decompress_and_write_blocks(&self, data_path: &String, offset: u64) {
        let mut data_file = BufferFile::from_file_path(data_path);
        let asset_dat_file = AssetDatFile::new(&mut data_file, offset);
        self.write_blocks(asset_dat_file.to_decompressed());
    }

    pub fn as_new(&self, extension: &str) -> SaveFilePath {
        let mut decoded_file_path = self.file_path_buf.clone();
        decoded_file_path.set_extension(extension);
        let decoded_file_path_str = decoded_file_path.to_str().unwrap().to_owned();
        SaveFilePath {
            file_path_buf: decoded_file_path,
            file_path_str: decoded_file_path_str,
            exists: true,
        }
    }

    pub fn decode_to_wav(&self) {
        Command::new("vgmstream-cli").arg(&self.file_path_str).spawn().unwrap();
    }

    pub fn remove(&mut self) {
        fs::remove_file(&self.file_path_buf).unwrap();
        self.exists = false;
    }
}