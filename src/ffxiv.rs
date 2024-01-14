use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use flate2::{Decompress, FlushDecompress};
use positioned_io::{RandomAccessFile, ReadAt};
use crate::ffxiv::parser::ffxiv_data::assets::index::{Index, Index1Data1Item};
use crate::ffxiv::parser::ffxiv_data::FFXIVAssetFiles;

use crate::ffxiv::parser::ffxiv_data::metadata::FFXIVFileMetadata;
use crate::ffxiv::parser::ffxiv_data::metadata::index_path::IndexPath;
use crate::ffxiv::reader::buffer_with_log::BufferWithLog;
use crate::ffxiv::reader::buffer_with_random_access::BufferWithRandomAccess;

mod decoder;
mod parser;
mod reader;
mod visualizer;

pub struct FFXIV {
    asset_files: Vec<FFXIVAssetFiles>,
}

fn get_scd_file(){

}

pub fn test(game_path: &str, index_path: &str) {
    let parsed_index_path = IndexPath::from_str(index_path).unwrap();

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
        let mut index1_file = BufferWithLog::new(index1_file_handle);
        let parsed_index = Index::from_index1(&mut index1_file);
        let index_item = parsed_index.data1.iter().find(|item| item.hash == parsed_index_path.index1_hash);
        if let Some(item) = index_item {
            let data_item = possible_asset_file.dat_files.iter().find(|d| d.data_chunk.id == item.data_file_id).ok_or("Data file could not be found.").unwrap();
            let mut data_file = BufferWithRandomAccess::from_file(&data_item.file_path.as_os_str().to_str().unwrap());

            let offset = data_file.offset_set(item.data_file_offset as usize);
            let metadata_header_size = data_file.u32();

            let offset = data_file.offset_set(offset + metadata_header_size as usize);
            let data_header_size = data_file.u32();
            let data_header_version = data_file.u32();
            let data_header_compressed_size = data_file.u32();
            let data_header_uncompressed_size = data_file.u32();

            let offset = data_file.offset_set(offset + data_header_size as usize);
            let compressed_file_buffer = data_file.vec(data_header_compressed_size as usize);

            fs::write("./media/compressed_file.scd", &compressed_file_buffer).unwrap();

            //let mut outt: Vec<u8> = vec![0; data_header_uncompressed_size as usize * 8];
            let mut decompressed_buffer: Vec<u8> = Vec::with_capacity(data_header_uncompressed_size as usize * 8);
            let mut decompressor = Decompress::new(false);
            decompressor.decompress_vec(&compressed_file_buffer, &mut decompressed_buffer, FlushDecompress::Finish).unwrap();
            fs::write("./media/decompressed.scd", &decompressed_buffer).unwrap();

            let mut decompressed_scd_buffer_with_log = BufferWithLog::new(decompressed_buffer);
            let metadata = parser::ffxiv_data::assets::dat::dat_scd::SCD::new(&mut decompressed_scd_buffer_with_log);

            let decoded = decoder::audio::sqex_scd::decode(&metadata, &mut decompressed_scd_buffer_with_log);
            fs::write("./media/decoded.scd", &decoded).unwrap();

            let spec = hound::WavSpec {
                channels: metadata.entry_channels as u16,
                sample_rate: metadata.entry_sample_rate as u32,
                bits_per_sample: 16,
                sample_format: hound::SampleFormat::Int,
            };
            let mut writer = hound::WavWriter::create("encoded.wav", spec).unwrap();
            for index in (0..decoded.len()).step_by(2) {
                writer.write_sample(i16::from_le_bytes([decoded[index], decoded[index + 1]])).unwrap();
            }

            println!("{}", data_header_compressed_size);
            //
            // let mut metadata_header_buffer_static = vec![0; 0x10];
            // let mut data_header_buffer_static = [0; 0x10];
            //
            // let data_file_handle = RandomAccessFile::open(&data_item.file_path).unwrap();
            //
            // let offset: u64 = item.data_file_offset;
            //
            // let bytes_read = data_file_handle.read_at(offset, &mut metadata_header_buffer_static).unwrap();
            //
            // let mut metadata_header_buffer_wrapper = BufferWithLog::new(metadata_header_buffer_static.clone());
            //
            // let offset = metadata_header_buffer_wrapper.u32(0x00) as u64 + offset;
            //
            // let bytes_read = data_file_handle.read_at(offset, &mut data_header_buffer_static).unwrap();
            //
            // println!("{:?}", metadata_header_buffer_static);
            // println!("{:?}", data_header_buffer_static);
            // break;
        }
    }
}

pub fn test2() {

}

impl FFXIV {
    pub fn new(game_path: &str) -> FFXIV {
        let asset_files: Vec<FFXIVAssetFiles> = FFXIVAssetFiles::new(game_path).unwrap();

        let mut path_hashes: HashMap<u64, IndexPath> = HashMap::new();
        let paths_file = File::open("./media/all_paths.txt").unwrap();
        let reader = BufReader::new(paths_file);


        let max_index: f32 = 1837802.0;
        let check_every: f32 = 10000.0;
        for (index, path) in reader.lines().enumerate() {
            let path = path.unwrap();
            let parsed_path = IndexPath::from_str(&path);
            if let Ok(parsed_path) = parsed_path {
                path_hashes.insert(parsed_path.index1_hash, parsed_path);

                let index = index as f32;
                if index % check_every == 0.0 {
                    let done = (index / max_index) * 100.0;
                    println!("Reading path: {}%.\n", done);
                }
            }
        }


        let mut index1_hashes: HashMap<u64, Index1Data1Item> = HashMap::new();
        //let index2_hashes: HashMap<u64, String> = HashMap::new();

        let max_index: f32 = asset_files.len() as f32;
        let check_every: f32 = max_index / 100.0;
        for (index, asset_file) in asset_files.iter().enumerate() {
            let file = fs::read(&asset_file.index_file.file_path).unwrap();
            let mut file_buffer = BufferWithLog::new(file);
            let parsed_index = Index::from_index1(&mut file_buffer);
            for data in parsed_index.data1 {
                index1_hashes.insert(data.hash, data);
            }

            // let file = fs::read(&asset_file.index2_file.file_path).unwrap();
            // let mut file_buffer = BufferWithLog::new(file);
            // let parsed_index2 = Index::from_index2(&mut file_buffer);

            let index = index as f32;
            if index % check_every == 0.0 {
                let done = (index / max_index) * 100.0;
                println!("Parsing path: {}%.\n", done);
            }
        }

        let mut output: String = String::new();

        let max_index: f32 = asset_files.len() as f32;
        let check_every: f32 = max_index / 100.0;
        for (index, (hash, index1data1item)) in index1_hashes.iter().enumerate() {
            let path = path_hashes.get(&hash);
            if let Some(path) = path {
                output.push_str(&format!("{} {}\n", hash, path.full_path));
            } else {
                output.push_str(&format!("{}\n", hash));
            }

            let index = index as f32;
            if index % check_every == 0.0 {
                let done = (index / max_index) * 100.0;
                println!("Writing path: {}%.\n", done);
            }
        }

        fs::write("./media/has2.txt", output).unwrap();


        FFXIV {
            asset_files
        }
    }
}
