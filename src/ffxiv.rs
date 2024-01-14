use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicUsize;
use flate2::{Decompress, FlushDecompress};
use positioned_io::{RandomAccessFile, ReadAt};
use crate::ffxiv::parser::ffxiv_data::assets::dat::dat_scd::SCD;
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

// pub struct CompressedSCD {
//
// }

struct Block {
    uncompressed_size: usize,
    compressed_data: Vec<u8>
}

fn get_scd_file(data_file: &mut BufferWithRandomAccess, data_file_offset: usize) -> (usize, Vec<u8>) {
    let offset = data_file.offset_set(data_file_offset);
    let metadata_header_size = data_file.u32();

    let offset = data_file.offset_set(offset + metadata_header_size as usize);
    let data_header_size = data_file.u32();
    let data_header_version = data_file.u32();
    let data_header_compressed_size = data_file.u32();
    let data_header_uncompressed_size = data_file.u32();

    let offset = data_file.offset_set(offset + data_header_size as usize);
    let compressed_file_buffer = data_file.vec(data_header_compressed_size as usize);

    (data_header_uncompressed_size as usize, compressed_file_buffer)
}

fn decompress(buffer: Vec<u8>, data_header_uncompressed_size: usize) -> Result<Vec<u8>, String> {
    let mut decompressed_buffer: Vec<u8> = Vec::with_capacity(data_header_uncompressed_size);
    let mut decompressor = Decompress::new(false);
    decompressor.decompress_vec(&buffer, &mut decompressed_buffer, FlushDecompress::Finish).or(Err("Failed to decompress"))?;
    Ok(decompressed_buffer)
}

// fn decode(decompressed_buffer: Vec<u8>) -> (SCD, Vec<u8>) {
//     let mut decompressed_scd_buffer_with_log = BufferWithLog::new(decompressed_buffer);
//     let metadata = SCD::new(&mut decompressed_scd_buffer_with_log);
//     let decoded = decoder::audio::sqex_scd::decode(&metadata, &mut decompressed_scd_buffer_with_log);
//     (metadata, decoded)
// }

fn decode(decompressed_buffer: Vec<u8>) -> (SCD, Vec<u8>) {
    let mut decompressed_scd_buffer_with_log = BufferWithLog::new(decompressed_buffer);
    let metadata = SCD::new(&mut decompressed_scd_buffer_with_log);
    let decoded = decoder::audio::sqex_scd::decode(&metadata, &mut decompressed_scd_buffer_with_log);
    (metadata, decoded)
}

fn re_encode_as_wav_and_save(metadata: SCD, decoded_buffer: Vec<u8>, name: &str) {
    let spec = hound::WavSpec {
        channels: metadata.entry_channels as u16,
        sample_rate: metadata.entry_sample_rate as u32,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create(format!("./media/here/{}.scd", name), spec).unwrap();
    for index in (0..decoded_buffer.len()).step_by(2) {
        writer.write_sample(i16::from_le_bytes([decoded_buffer[index], decoded_buffer[index + 1]])).unwrap();
    }
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
        let parsed_index1 = Index::from_index1(&mut index1_file);
        let index1_item = parsed_index1.data1.iter().find(|item| item.hash == parsed_index_path.index1_hash);
        if let Some(item) = index1_item {
            let data_item = possible_asset_file.dat_files.iter().find(|d| d.data_chunk.id == item.data_file_id).ok_or("Data file could not be found.").unwrap();
            let data_item_path = data_item.file_path.as_os_str().to_str().unwrap();
            let mut data_file = BufferWithRandomAccess::from_file(data_item_path);

            let (data_header_uncompressed_size, compressed_file_buffer) = get_scd_file(&mut data_file, item.data_file_offset as usize);
            let l = compressed_file_buffer.len();
            let decompressed_buffer = decompress(compressed_file_buffer, data_header_uncompressed_size).unwrap();

            let decompressed_file_path = format!("./media/here/{}_{}.scd", parsed_index_path.index1_hash, parsed_index_path.file_stem);
            fs::write(&decompressed_file_path, decompressed_buffer).unwrap();

            //let (metadata, decoded) = decode(decompressed_buffer);
            //re_encode_as_wav_and_save(metadata, decoded, &parsed_index_path.file_stem);
        }

        // let index2_file_handle = fs::read(possible_asset_file.index2_file.file_path).unwrap();
        // let mut index2_file = BufferWithLog::new(index2_file_handle);
        // let parsed_index2 = Index::from_index2(&mut index2_file);
        // let index2_item = parsed_index2.data1.iter().find(|item| item.hash == parsed_index_path.index2_hash);
        // if let Some(item) = index2_item {
        //     let data_item = possible_asset_file.dat_files.iter().find(|d| d.data_chunk.id == item.data_file_id).ok_or("Data file could not be found.").unwrap();
        //     let mut data_file = BufferWithRandomAccess::from_file(&data_item.file_path.as_os_str().to_str().unwrap());
        //
        //     let (data_header_uncompressed_size, compressed_file_buffer) = get_scd_file(&mut data_file, item.data_file_offset as usize);
        //     let decompressed_buffer = decompress(compressed_file_buffer, data_header_uncompressed_size).unwrap();
        //
        //     let decompressed_file_path = format!("./media/here/{}_{}.scd", parsed_index_path.index1_hash, parsed_index_path.file_stem);
        //     fs::write(&decompressed_file_path, decompressed_buffer).unwrap();
        //     //let (metadata, decoded) = decode(decompressed_buffer);
        //     //re_encode_as_wav_and_save(metadata, decoded, &parsed_index_path.file_stem);
        // }
    }
}

pub fn test2(game_path: &str) {
    let hash_names = get_game_asset_hash_names();
    let hashes = get_game_asset_hashes(game_path);

    let a = hash_names.len();
    let b = hashes.len();


    //let mut output: String = String::new();

    //let mut error_log = File::open("error_log.txt").unwrap();

    let max_index: f32 = hashes.len() as f32;
    let check_every: f32 = (max_index / 100.0).floor();
    for (index, (hash, (data_path, index1data1item))) in hashes.iter().enumerate() {
        let path = hash_names.get(&hash);
        if let Some(path) = path {
            if path.file_extension == "scd" {
                let mut data_file = BufferWithRandomAccess::from_file(data_path);
                let (data_header_uncompressed_size, compressed_file_buffer) = get_scd_file(&mut data_file, index1data1item.data_file_offset as usize);
                let decompressed_buffer = decompress(compressed_file_buffer, data_header_uncompressed_size);
                if let Ok(decompressed_buffer) = decompressed_buffer {
                    let decompressed_file_path = format!("./media/here/{}_{}.scd", path.index1_hash, path.file_stem);
                    let re_encoded_file_path = format!("./media/there/{}_{}.scd", path.index1_hash, path.file_stem);
                    fs::write(&decompressed_file_path, decompressed_buffer).unwrap();
                    //Command::new("vgmstream-cli").arg(decompressed_file_path).arg(format!("-o {}.wav", re_encoded_file_path)).spawn().unwrap();
                    Command::new("vgmstream-cli").arg(decompressed_file_path).spawn().unwrap();
                } else {
                    let msg = format!("failed to decompress: {}", path.full_path);
                    println!("{}", msg);
                    //error_log.write(format!("failed to decompress: {}", path.full_path).as_bytes()).unwrap();
                }
                //let (metadata, decoded) = decode(decompressed_buffer);

                //re_encode_as_wav_and_save(metadata, decoded, &path.file_stem);
            }

            //output.push_str(&format!("{} {}\n", hash, path.full_path));
        } else {
            //output.push_str(&format!("{}\n", hash));
        }

        let index = index as f32;
        if index % check_every == 0.0 {
            let done = (index / max_index) * 100.0;
            println!("Writing path: {}%.\n", done);
        }
    }

    //fs::write("./media/has2.txt", output).unwrap();
}

fn get_game_asset_hash_names() -> HashMap<u64, IndexPath> {
    let mut thread_handles = vec![];
    let mut thread_count = std::thread::available_parallelism().unwrap().get();

    let mut path_hashes: HashMap<u64, IndexPath> = HashMap::new();
    //let path_hashes2: HashMap<u64, IndexPath> = HashMap::new();
    let paths_file = File::open("./media/all_paths.txt").unwrap();
    //let reader = BufReader::new(paths_file);

    //let line_count = reader.lines().count();
    let paths_file = fs::read_to_string("./media/all_paths.txt").unwrap();
    let paths: Vec<&str> = paths_file.split("\n").collect();
    let line_count = paths.len();
    if thread_count > line_count {
        thread_count = line_count;
    }

    let path_chunks: Vec<&[&str]> = paths.chunks(line_count / thread_count).collect();
    // let line_count = paths.len();
    // let parse_step_count = line_count / thread_count;
    // let parse_last_step_count = parse_step_count + (line_count % thread_count);


    let path_hashes_arc_mutex = Mutex::new(path_hashes);
    let path_hashes_arc = Arc::new(path_hashes_arc_mutex);
    for thread_index in 0..thread_count {
        let chunk = path_chunks[thread_index];

        // let mut parse_step_count = parse_step_count;
        // if index < thread_count {
        //     parse_step_count = parse_last_step_count;
        // }
        // let block_start_index = index * parse_step_count;


        let paths_block: Vec<String> = chunk.to_vec().iter().map(|p| (*p).to_owned()).collect();
        let line_count = paths_block.len();
        let path_hashes_clone = Arc::clone(&path_hashes_arc);

        let handle = std::thread::spawn(move || {
            let max_index: f32 = line_count as f32;
            let check_every: f32 = (max_index / 100.0).floor();
            let mut path_hashes: HashMap<u64, IndexPath> = HashMap::new();

            for (index, path) in paths_block.iter().enumerate() {
                let parsed_path = IndexPath::from_str(&path);
                if let Ok(parsed_path) = parsed_path {
                    path_hashes.insert(parsed_path.index1_hash, parsed_path);

                    let index = index as f32;
                    if index % check_every == 0.0 {
                        let done = (index / max_index) * 100.0;
                        println!("Thread {} Reading path: {}%.\n", thread_index, done);
                    }
                }
            }
            let mut map = path_hashes_clone.lock().unwrap().extend(path_hashes);
        });
        thread_handles.push(handle);
    }


    for thread_handle in thread_handles {
        thread_handle.join().unwrap();
    }

    //
    // let max_index: f32 = 1837802.0;
    // let check_every: f32 = (max_index / 100.0).floor();
    //
    // for (index, path) in reader.lines().enumerate() {
    //     let path = path.unwrap();
    //     let parsed_path = IndexPath::from_str(&path);
    //     if let Ok(parsed_path) = parsed_path {
    //         path_hashes.insert(parsed_path.index1_hash, parsed_path);
    //
    //         let index = index as f32;
    //         if index % check_every == 0.0 {
    //             let done = (index / max_index) * 100.0;
    //             println!("Reading path: {}%.\n", done);
    //         }
    //     }
    // }
    let mut gg = Arc::try_unwrap(path_hashes_arc).unwrap().into_inner().unwrap();

    gg
}

fn get_game_asset_hashes(game_path: &str) -> HashMap<u64, (String, Index1Data1Item)> {
    let asset_files: Vec<FFXIVAssetFiles> = FFXIVAssetFiles::new(game_path).unwrap();
    let mut index1_hashes: HashMap<u64, (String, Index1Data1Item)> = HashMap::new();
    //let index2_hashes: HashMap<u64, String> = HashMap::new();

    let max_index: f32 = asset_files.len() as f32;
    let check_every: f32 = (max_index / 100.0).floor();
    for (index, asset_file) in asset_files.iter().enumerate() {
        let file = fs::read(&asset_file.index_file.file_path).unwrap();
        let mut file_buffer = BufferWithLog::new(file);
        let parsed_index = Index::from_index1(&mut file_buffer);
        for data in parsed_index.data1 {
            index1_hashes.insert(data.hash, (asset_file.dat_files.iter().find(|f| f.file_extension == format!("dat{}", data.data_file_id)).unwrap().file_path_str.clone(), data));
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

    index1_hashes
}

// impl FFXIV {
//     pub fn new() -> FFXIV {
//
//
//
//
//
//         FFXIV {
//             asset_files
//         }
//     }
// }
