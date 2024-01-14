use std::collections::HashMap;
use std::{fs, result};
use std::fs::{create_dir_all, File};
use std::io::{BufRead, BufReader, Read, Write};
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicUsize;
use flate2::{Decompress, FlushDecompress};
use positioned_io::{RandomAccessFile, ReadAt};
use zlib_stream::{ZlibDecompressionError, ZlibStreamDecompressor};
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
#[derive(Clone)]
enum BlockType {
    Compressed(u32),
    Uncompressed(u32),
}

#[derive(Clone)]
struct DatFileNormalAssetMetadataBlock {
    offset: u32,
    uncompressed_block_size: u16,
    compressed_block_size: u16,
}

#[derive(Clone)]
struct BlockDataHeader {
    header_size: u32,
    header_version: u32,
    block_type: BlockType,
    uncompressed_block_size: u32,
}

#[derive(Clone)]
struct CompressedBlock {
    metadata_header: DatFileNormalAssetMetadataBlock,
    data_header: BlockDataHeader,
    compressed_buffer: Vec<u8>,
}

#[derive(Clone)]
struct DecompressedBlock {
    metadata_header: DatFileNormalAssetMetadataBlock,
    data_header: BlockDataHeader,
    compressed_buffer: Vec<u8>,
    decompressed_buffer: Vec<u8>,
}

struct DatFileNormalAssetMetadata {
    metadata_size: u32,
    metadata_version: u32,
    asset_size: u32,
    unknown1: u32,
    unknown2: u32,
    block_count: u32,
    blocks: Vec<DatFileNormalAssetMetadataBlock>,
}

struct DatFileNormalAssetBlockData {
    header_size: u32,
    header_version: u32,
    block_type: BlockType,
    uncompressed_block_size: u32,
    data: Vec<u8>,
}

impl DatFileNormalAssetBlockData {
    pub fn new(data_file: &mut BufferWithRandomAccess, data_file_offset: u64, dat_file_metadata: &DatFileNormalAssetMetadata, block_metadata: &DatFileNormalAssetMetadataBlock) -> DatFileNormalAssetBlockData {
        let block_offset = data_file_offset + (dat_file_metadata.metadata_size + block_metadata.offset) as u64;
        data_file.offset_set(block_offset as usize);
        let header_size = data_file.u32();
        let header_version = data_file.u32();
        let block_type = BlockType::new(data_file.u32());
        let uncompressed_block_size = data_file.u32();
        let block_data_offset = (block_offset + header_size as u64) as usize;
        let data = match block_type {
            BlockType::Compressed(n) => data_file.vec_at(block_data_offset, n as usize),
            BlockType::Uncompressed(n) => data_file.vec_at(block_data_offset, uncompressed_block_size as usize)
        };

        DatFileNormalAssetBlockData {
            header_size,
            header_version,
            block_type,
            uncompressed_block_size,
            data,
        }
    }
    pub fn from_metadata(data_file: &mut BufferWithRandomAccess, dat_file_metadata: &DatFileNormalAssetMetadata, data_file_offset: u64) -> Vec<DatFileNormalAssetBlockData> {
        data_file.offset_set((data_file_offset + dat_file_metadata.metadata_size as u64) as usize);
        dat_file_metadata.blocks.iter().map(|block_metadata| DatFileNormalAssetBlockData::new(data_file, data_file_offset, dat_file_metadata, block_metadata)).collect()
    }
}

impl DatFileNormalAssetMetadata {
    pub fn new(data_file: &mut BufferWithRandomAccess, data_file_offset: u64) -> DatFileNormalAssetMetadata {
        data_file.offset_set(data_file_offset as usize);
        let metadata_size = data_file.u32();
        let metadata_version = data_file.u32();
        let asset_size = data_file.u32();
        let unknown1 = data_file.u32();
        let unknown2 = data_file.u32();
        let block_count = data_file.u32();
        let blocks = (0..block_count).map(|i| DatFileNormalAssetMetadataBlock::from_buffer(data_file)).collect();
        DatFileNormalAssetMetadata {
            metadata_size,
            metadata_version,
            asset_size,
            unknown1,
            unknown2,
            block_count,
            blocks,
        }
    }
}

impl BlockDataHeader {
    pub fn from_buffer_at(buffer: &mut BufferWithRandomAccess, at: u32) -> BlockDataHeader {
        BlockDataHeader {
            header_size: buffer.u32_at(at as usize),
            header_version: buffer.u32_at(at as usize + 0x04),
            block_type: BlockType::new(buffer.u32_at(at as usize + 0x08)),
            uncompressed_block_size: buffer.u32_at(at as usize + 0x0C),
        }
    }
}

impl BlockType {
    pub fn new(n: u32) -> BlockType {
        match n {
            32000 => BlockType::Uncompressed(32000),
            _ => BlockType::Compressed(n)
        }
    }
}

impl DatFileNormalAssetMetadataBlock {
    pub fn from_buffer(buffer: &mut BufferWithRandomAccess) -> DatFileNormalAssetMetadataBlock {
        DatFileNormalAssetMetadataBlock {
            offset: buffer.u32(),
            compressed_block_size: buffer.u16(),
            uncompressed_block_size: buffer.u16(),
        }
    }
}

fn dat_file_normal_asset_metadata(data_file: &mut BufferWithRandomAccess, data_file_offset: usize) {}

// fn get_scd_file(data_file: &mut BufferWithRandomAccess, data_file_offset: usize) -> Vec<CompressedBlock> {
//     let offset = data_file.offset_set(data_file_offset);
//     let metadata_header_size = data_file.u32();
//     let metadata_header_version = data_file.u32();
//     let metadata_header_something_1 = data_file.u32();
//     let metadata_header_something_2 = data_file.u32();
//     let metadata_header_something_3 = data_file.u32();
//     let metadata_header_block_count = data_file.u32();
//
//     let mut blocks: Vec<CompressedBlock> = Vec::new();
//
//     let mut compressed_block = CompressedBlock {
//         metadata_header: DatFileNormalAssetMetadataBlock {
//             offset: 0,
//             uncompressed_block_size: 0,
//             compressed_block_size: 0,
//         },
//         data_header: BlockDataHeader {
//             header_size: 0,
//             header_version: 0,
//             uncompressed_block_size: 0,
//             block_type: 0,
//         },
//         compressed_buffer: Vec::new(),
//     };
//
//     for block_index in 0..metadata_header_block_count {
//         let block_metadata = DatFileNormalAssetMetadataBlock::from_buffer(data_file);
//
//         let mut at = offset + (metadata_header_size + block_offset) as usize;
//
//         let data_header_size = data_file.u32_at(at);
//         let data_header_version = data_file.u32_at(at + 0x04);
//         let data_block_type = data_file.u32_at(at + 0x08);
//         let data_uncompressed_block_size = data_file.u32_at(at + 0x0C);
//         at += data_header_size as usize;
//
//         let mut compressed_buffer = data_file.vec_at(at, (header_block_size as u32 - data_header_size) as usize);
//
//         //let len = (&compressed_block.compressed_buffer).len() as u32;
//
//         compressed_block.metadata_header.offset = at as u32;
//         compressed_block.metadata_header.uncompressed_block_size = header_offset_to_next_block;
//         compressed_block.metadata_header.compressed_block_size = header_block_size;
//
//         compressed_block.data_header.header_size = data_header_size;
//         compressed_block.data_header.header_version = data_header_version;
//         compressed_block.data_header.block_type = BlockType::new(data_block_type);
//         compressed_block.data_header.uncompressed_block_size = data_uncompressed_block_size;
//
//         compressed_block.compressed_buffer = compressed_buffer;
//
//
//         blocks.push(compressed_block.clone());
//         compressed_block.compressed_buffer = Vec::new();
//
//
//
//         //  if data_needed_block_size > 16000 {
//         //      if  { }
//         //  } else {
//         //      compressed_block.metadata_header.offset = at as u32;
//         //      compressed_block.metadata_header.uncompressed_block_size = header_offset_to_next_block;
//         //      compressed_block.metadata_header.compressed_block_size = header_block_size;
//         //
//         //      compressed_block.data_header.header_size = data_header_size;
//         //      compressed_block.data_header.header_version = data_header_version;
//         //      compressed_block.data_header.needed_block_size = data_needed_block_size;
//         //      compressed_block.data_header.uncompressed_block_size = data_offset_to_next_block;
//         //
//         //      compressed_block.compressed_buffer = compressed_buffer;
//         //
//         //
//         //      blocks.push(compressed_block.clone());
//         //      compressed_block.compressed_buffer = Vec::new();
//         //  }
//         //
//         //
//         //  if data_needed_block_size > 16000 {
//         //      if len == 0 {
//         //          compressed_block.metadata_header.offset = at as u32;
//         //          compressed_block.metadata_header.uncompressed_block_size = header_offset_to_next_block;
//         //          compressed_block.metadata_header.compressed_block_size = header_block_size;
//         //
//         //          compressed_block.data_header.header_size = data_header_size;
//         //          compressed_block.data_header.header_version = data_header_version;
//         //          compressed_block.data_header.needed_block_size = data_needed_block_size;
//         //          compressed_block.data_header.uncompressed_block_size = data_offset_to_next_block;
//         //
//         //          compressed_block.compressed_buffer = compressed_buffer;
//         //
//         //
//         //
//         //
//         //      } else if len > 0 && len <= data_needed_block_size {
//         //          compressed_block.compressed_buffer.append(&mut compressed_buffer);
//         //
//         //      } else {
//         //          compressed_block.compressed_buffer.append(&mut compressed_buffer);
//         //
//         //
//         //          blocks.push(compressed_block.clone());
//         //          compressed_block.compressed_buffer = Vec::new();
//         //      }
//         //  }
//         //  else {
//         //      if len > 0 {
//         //
//         //          blocks.push(compressed_block.clone());
//         //          compressed_block.compressed_buffer = Vec::new();
//         //      }
//         //     else {
//         //         compressed_block.metadata_header.offset = at as u32;
//         //         compressed_block.metadata_header.uncompressed_block_size = header_offset_to_next_block;
//         //         compressed_block.metadata_header.compressed_block_size = header_block_size;
//         //
//         //         compressed_block.data_header.header_size = data_header_size;
//         //         compressed_block.data_header.header_version = data_header_version;
//         //         compressed_block.data_header.needed_block_size = data_needed_block_size;
//         //         compressed_block.data_header.uncompressed_block_size = data_offset_to_next_block;
//         //
//         //         compressed_block.compressed_buffer = compressed_buffer;
//         //
//         //
//         //         blocks.push(compressed_block.clone());
//         //         compressed_block.compressed_buffer = Vec::new();
//         //     }
//         // }
//
//         fs::write(format!("./gg_{}", block_index), &compressed_block.compressed_buffer).unwrap();
//
//
//         // let len = (&compressed_block.compressed_buffer).len() as u32;
//         // if len == 0 || len == compressed_block.data_header.needed_block_size {
//         //     compressed_block.metadata_header.offset = at as u32;
//         //     compressed_block.metadata_header.offset_to_next_block = header_offset_to_next_block;
//         //     compressed_block.metadata_header.block_size = header_block_size;
//         //
//         //     compressed_block.data_header.header_size = data_header_size;
//         //     compressed_block.data_header.header_version = data_header_version;
//         //     compressed_block.data_header.needed_block_size = data_needed_block_size;
//         //     compressed_block.data_header.offset_to_next_block = data_offset_to_next_block;
//         //
//         //     compressed_block.compressed_buffer = compressed_buffer;
//         //
//         //
//         // } else if len < compressed_block.data_header.needed_block_size {
//         //     compressed_block.compressed_buffer.append(&mut compressed_buffer);
//         // } else {
//         //     panic!("Something went terrible wrong.");
//         // }
//
//         // let metadata_header = BlockMetadataHeader {
//         //     offset: at as u32,
//         //     offset_to_next_block: compressed_buffer_size,
//         //     block_size: uncompressed_buffer_size,
//         // };
//         //
//         // let data_header = BlockDataHeader {
//         //     header_size: data_header_size,
//         //     header_version: data_header_version,
//         //     block_size: data_block_size,
//         //     needed_block_size: data_needed_block_size,
//         // };
//
//
//         // let data_header_version = data_file.u32();
//         // let data_header_compressed_size = data_file.u32();
//         // let data_header_uncompressed_size = data_file.u32();
//
//
//         //blocks.push(compressed_block)
//     }
//
//     //let offset = data_file.offset_set(offset + metadata_header_size);
//
//
//     // let offset = data_file.offset_set(offset + data_header_size as usize);
//     // let compressed_file_buffer = data_file.vec(data_header_compressed_size as usize);
//     //
//     // (data_header_uncompressed_size as usize, compressed_file_buffer)
//     blocks
// }

fn decompress(blocks: Vec<DatFileNormalAssetBlockData>) -> Vec<Vec<u8>> {
    blocks.iter().map(|block| {

        //let decompressed_block = Vec::with_capacity(block.uncompressed_block_size as usize);
        match block.block_type {
            BlockType::Compressed(n) => {
                let mut decompressed_block_data: Vec<u8> = Vec::with_capacity(block.uncompressed_block_size as usize);
                let mut decompressor = Decompress::new_with_window_bits(false, 15);
                decompressor.decompress_vec(&block.data, &mut decompressed_block_data, FlushDecompress::None).unwrap();
                decompressed_block_data
            }
            BlockType::Uncompressed(n) => block.data.clone()
        }
    }).collect()
    // let mut decompressed_blocks: Vec<DecompressedBlock> = Vec::new();
    //
    // let mut decompressor: ZlibStreamDecompressor = ZlibStreamDecompressor::new();
    //
    // for compressed_block in blocks {
    //     let mut decompressed_buffer: Vec<u8> = Vec::with_capacity(640000);
    //     // match decompressor.decompress(&compressed_block.compressed_buffer) {
    //     //     Ok(mut vec) => {
    //     //         decompressed_buffer.append(&mut vec);
    //     //         println!("test");
    //     //     }
    //     //     Err(ZlibDecompressionError::NeedMoreData) => {
    //     //         println!("test");
    //     //         continue;
    //     //     }
    //     //     Err(_err) => Err(_err.to_string())?
    //     // }
    //     //
    //
    //     let mut decompressor = Decompress::new_with_window_bits(false, 15);
    //
    //     let result = decompressor.decompress_vec(&compressed_block.compressed_buffer, &mut decompressed_buffer, FlushDecompress::Finish);
    //     if let Ok(status) = result {
    //         decompressed_blocks.push(DecompressedBlock {
    //             decompressed_buffer,
    //             compressed_buffer: compressed_block.compressed_buffer,
    //             metadata_header: compressed_block.metadata_header,
    //             data_header: compressed_block.data_header,
    //         })
    //     } else {
    //         // for block in blocks {
    //         //
    //         // }
    //         fs::write("./ooo.txt", &compressed_block.compressed_buffer).unwrap();
    //         result.unwrap();
    //         println!("tset");
    //     }
    //
    //     // match result {
    //     //     Ok(_) => {
    //     //         println!("nice");
    //     //     },
    //     //     Err(DecompressError::)
    //     // }
    // }
    // Ok(decompressed_blocks)
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

            let asset_metadata = DatFileNormalAssetMetadata::new(&mut data_file, item.data_file_offset);
            let compressed_asset_data_blocks = DatFileNormalAssetBlockData::from_metadata(&mut data_file, &asset_metadata, item.data_file_offset);
            let decompressed_asset_data_blocks = decompress(compressed_asset_data_blocks);
            write_to_file(decompressed_asset_data_blocks, &parsed_index_path);

            //let compressed_blocks = get_scd_file(&mut data_file, item.data_file_offset as usize);

            // let decompressed_blocks = decompress(compressed_blocks).unwrap();
            //
            // let decompressed_file_path = format!("./media/here/{}_{}.scd", parsed_index_path.index1_hash, parsed_index_path.file_stem);
            // let decompressed_file_path_buf = PathBuf::from(decompressed_file_path);
            // if decompressed_file_path_buf.exists() {
            //     fs::remove_file(&decompressed_file_path_buf).unwrap();
            // }
            //
            // // if decompressed_file_path_buf.exists() {
            // //     let mut scd_file = File::open(decompressed_file_path_buf).unwrap();
            // //     for decompressed_block in decompressed_blocks {
            // //         scd_file.write(&decompressed_block.decompressed_buffer).unwrap();
            // //     }
            // // } else {
            // //
            // // }
            //
            // let dir = decompressed_file_path_buf.parent().unwrap();
            // create_dir_all(dir).unwrap();
            // let mut scd_file = File::create(decompressed_file_path_buf).unwrap();
            // for decompressed_block in decompressed_blocks {
            //     scd_file.write_all(&decompressed_block.decompressed_buffer).unwrap();
            // }


            // fs::write(&decompressed_file_path, decompressed_buffer).unwrap();

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

pub struct SaveFilePath {
    pub file_path_str: String,
    pub file_path_buf: PathBuf,
    pub exists: bool,
}

impl SaveFilePath {
    pub fn new(index_path: &IndexPath, file_extension: String) -> SaveFilePath {
        //let file_stem = format!("{}_{}", index_path.index1_hash, index_path.file_stem);
        let file_path_str = format!("./media/here/{}_{}.{}", index_path.index1_hash, index_path.file_stem, file_extension);
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

    pub fn write_decompressed_blocks(&self, data_path: &String, offset: u64) {
        let mut data_file = BufferWithRandomAccess::from_file(data_path);
        let asset_metadata = DatFileNormalAssetMetadata::new(&mut data_file, offset);
        let compressed_asset_data_blocks = DatFileNormalAssetBlockData::from_metadata(&mut data_file, &asset_metadata, offset);
        let decompressed_asset_data_blocks = decompress(compressed_asset_data_blocks);
        self.write_blocks(decompressed_asset_data_blocks);
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

pub fn write_to_file(blocks: Vec<Vec<u8>>, index_path: &IndexPath) {
    let decompressed_file_path = format!("./media/here/{}_{}.scd", index_path.index1_hash, index_path.file_stem);
    let decompressed_file_path_buf = PathBuf::from(&decompressed_file_path);
    if decompressed_file_path_buf.exists() {
        fs::remove_file(&decompressed_file_path_buf).unwrap();
    }

    let dir = decompressed_file_path_buf.parent().unwrap();
    create_dir_all(dir).unwrap();
    let mut scd_file = File::create(&decompressed_file_path_buf).unwrap();
    for decompressed_block in blocks {
        scd_file.write_all(&decompressed_block).unwrap();
    }

    Command::new("vgmstream-cli").arg(decompressed_file_path).spawn().unwrap();
}

pub fn write_to_file2(blocks: Vec<Vec<u8>>, save_file_path: &SaveFilePath) {
    let dir = save_file_path.file_path_buf.parent().unwrap();
    create_dir_all(dir).unwrap();
    let mut scd_file = File::create(&save_file_path.file_path_buf).unwrap();
    for decompressed_block in blocks {
        scd_file.write_all(&decompressed_block).unwrap();
    }

    Command::new("vgmstream-cli").arg(&save_file_path.file_path_str).spawn().unwrap();
}

pub fn test_exd(game_path: &str) {
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
            if path.file_extension == "exl" {
                let mut scd_file_path = SaveFilePath::new(path, String::from("exl"));

                if !scd_file_path.exists {
                    scd_file_path.write_decompressed_blocks(data_path, index1data1item.data_file_offset);
                }
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
                let mut scd_file_path = SaveFilePath::new(path, String::from("scd"));

                if scd_file_path.exists {
                    let wav_file_path = scd_file_path.as_new("wav");

                    if !wav_file_path.exists {
                        scd_file_path.decode_to_wav();
                    }

                    scd_file_path.remove();
                } else {
                    let wav_file_path = scd_file_path.as_new("wav");

                    if !wav_file_path.exists {
                        scd_file_path.write_decompressed_blocks(data_path, index1data1item.data_file_offset);
                        scd_file_path.decode_to_wav();
                        scd_file_path.remove();
                    }
                }


                // let mut data_file = BufferWithRandomAccess::from_file(data_path);
                // let (data_header_uncompressed_size, compressed_file_buffer) = get_scd_file(&mut data_file, index1data1item.data_file_offset as usize);
                // let decompressed_buffer = decompress(compressed_file_buffer, data_header_uncompressed_size);

                // if let Ok(decompressed_buffer) = decompressed_buffer {
                //     let decompressed_file_path = format!("./media/here/{}_{}.scd", path.index1_hash, path.file_stem);
                //     let re_encoded_file_path = format!("./media/there/{}_{}.scd", path.index1_hash, path.file_stem);
                //     fs::write(&decompressed_file_path, decompressed_buffer).unwrap();
                //     //Command::new("vgmstream-cli").arg(decompressed_file_path).arg(format!("-o {}.wav", re_encoded_file_path)).spawn().unwrap();
                //     Command::new("vgmstream-cli").arg(decompressed_file_path).spawn().unwrap();
                // } else {
                //     let msg = format!("failed to decompress: {}", path.full_path);
                //     println!("{}", msg);
                //     //error_log.write(format!("failed to decompress: {}", path.full_path).as_bytes()).unwrap();
                // }
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
