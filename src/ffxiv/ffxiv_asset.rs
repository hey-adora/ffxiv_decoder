use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use crc::{Crc, CRC_32_JAMCRC};
use flate2::{Decompress, FlushDecompress};
use crate::ffxiv::asset_files::FFXIVAssetFiles;
use regex::bytes::Regex as RegByte;
use regex::Regex as Reg;
use crate::ffxiv::ffxiv_buffer::{FFXIVBuffer, FFXIVBufferReader};

// pub struct FFXIVAsset {
//
// }
//
// enum FFXIVAssetType {
//     Raw(Vec<u8>)
// }
//
//
//
// impl FFXIVAsset {
//     pub fn from_dat_file(dat_path: &str) {
//         let dat_path = FFXIVAssetPathDat::from_str(dat_path).unwrap();
//         dat_path
//         let index_file = FFXIVAssetParserIndex::from_index1_file(file_path);
//         FFXIVAssetParserDat::from_dat_file()
//     }
// }

//==================================================================================================

pub struct FFXIVAssetParserDat {
    pub header: FFXIVAssetParserDatHeader,
    pub raw_data_blocks: Vec<FFXIVAssetParserDatBlock>,
}


impl FFXIVAssetParserDat {
    pub fn from_dat_files(dat_files: &Vec<FFXIVAssetPathFile>, dat_id: u32, offset: u64) -> FFXIVAssetParserDat {
        let find_this_dat =  format!("dat{}", dat_id);
        let dat_file = dat_files.iter().find(|d| d.path_extension == find_this_dat).ok_or("Data file could not be found.").unwrap();
        FFXIVAssetParserDat::from_file_path(&dat_file.path_str, offset)
    }

    pub fn from_file_path<P: AsRef<Path>>(file_path: P, offset: u64) -> FFXIVAssetParserDat {
        let mut dat_file = FFXIVBuffer::from_file_path(file_path);
        FFXIVAssetParserDat::from_dat_file(&mut dat_file, offset)
    }

    pub fn from_dat_file<R: FFXIVBufferReader>(data_file: &mut FFXIVBuffer<R>, dat_file_offset: u64) -> FFXIVAssetParserDat {
        let header = FFXIVAssetParserDatHeader::new(data_file, dat_file_offset);
        let raw_data_blocks = FFXIVAssetParserDatBlock::from_header(data_file, &header, dat_file_offset);

        FFXIVAssetParserDat {
            header,
            raw_data_blocks
        }
    }

    pub fn to_decompressed(&self) -> Vec<Vec<u8>> {
        self.raw_data_blocks.iter().map(|block| {
            match block.block_type {
                FFXIVAssetParserDatBlockType::Compressed(n) => {
                    let mut decompressed_block_data: Vec<u8> = Vec::with_capacity(block.uncompressed_block_size as usize);
                    let mut decompressor = Decompress::new(false);
                    decompressor.decompress_vec(&block.data, &mut decompressed_block_data, FlushDecompress::None).unwrap();
                    decompressed_block_data
                }
                FFXIVAssetParserDatBlockType::Uncompressed(n) => block.data.clone()
            }
        }).collect()
    }
}


#[derive(Clone)]
pub struct FFXIVAssetParserDatHeader {
    pub header_size: u32,
    pub data_type: FFXIVAssetParserDatHeaderType,
    pub asset_size: u32,
    pub unknown1: u32,
    pub unknown2: u32,
    pub block_count: u32,
    pub blocks: Vec<FFXIVAssetParserDatHeaderBlock>,
}

impl FFXIVAssetParserDatHeader {
    pub fn new<R: FFXIVBufferReader>(data_file: &mut FFXIVBuffer<R>, data_file_offset: u64) -> FFXIVAssetParserDatHeader {
        data_file.offset_set(data_file_offset);
        let header_size = data_file.le_u32();
        let data_type = data_file.le_u32();
        let asset_size = data_file.le_u32();
        let unknown1 = data_file.le_u32();
        let unknown2 = data_file.le_u32();
        let block_count = data_file.le_u32();
        let blocks = (0..block_count).map(|i| FFXIVAssetParserDatHeaderBlock::from_buffer(data_file)).collect();
        FFXIVAssetParserDatHeader {
            data_type: FFXIVAssetParserDatHeaderType::new(data_type).unwrap(),
            header_size,
            asset_size,
            unknown1,
            unknown2,
            block_count,
            blocks,
        }
    }
}


#[derive(Clone)]
pub struct FFXIVAssetParserDatHeaderBlock {
    pub offset: u32,
    pub uncompressed_block_size: u16,
    pub compressed_block_size: u16,
}

impl FFXIVAssetParserDatHeaderBlock {
    pub fn from_buffer<R: FFXIVBufferReader>(buffer: &mut FFXIVBuffer<R>) -> FFXIVAssetParserDatHeaderBlock {
        FFXIVAssetParserDatHeaderBlock {
            offset: buffer.le_u32(),
            compressed_block_size: buffer.le_u16(),
            uncompressed_block_size: buffer.le_u16(),
        }
    }
}


#[derive(Clone)]
pub enum FFXIVAssetParserDatHeaderType {
    Empty = 1,
    Standard = 2,
    Model = 3,
    Texture = 4,
}

impl FFXIVAssetParserDatHeaderType {
    pub fn new(n: u32) -> Result<FFXIVAssetParserDatHeaderType, String> {
        match n {
            1 => Ok(FFXIVAssetParserDatHeaderType::Empty),
            2 => Ok(FFXIVAssetParserDatHeaderType::Standard),
            3 => Ok(FFXIVAssetParserDatHeaderType::Model),
            4 => Ok(FFXIVAssetParserDatHeaderType::Texture),
            _ => Err(format!("Data type '{}' not found.", n))
        }
    }
}


#[derive(Clone)]
pub struct FFXIVAssetParserDatBlock {
    pub header_size: u32,
    pub header_version: u32,
    pub block_type: FFXIVAssetParserDatBlockType,
    pub uncompressed_block_size: u32,
    pub data: Vec<u8>,
}

impl FFXIVAssetParserDatBlock {
    pub fn new<R: FFXIVBufferReader>(data_file: &mut FFXIVBuffer<R>, data_file_offset: u64, asset_dat_file_header: &FFXIVAssetParserDatHeader, block_metadata: &FFXIVAssetParserDatHeaderBlock) -> FFXIVAssetParserDatBlock {
        let block_offset = data_file_offset + (asset_dat_file_header.header_size + block_metadata.offset) as u64;
        data_file.offset_set(block_offset);
        let header_size = data_file.le_u32();
        let header_version = data_file.le_u32();
        let block_type = FFXIVAssetParserDatBlockType::new(data_file.le_u32());
        let uncompressed_block_size = data_file.le_u32();
        let block_data_offset = (block_offset + header_size as u64);
        let data = match block_type {
            FFXIVAssetParserDatBlockType::Compressed(n) => data_file.vec_at(block_data_offset, n as usize),
            FFXIVAssetParserDatBlockType::Uncompressed(n) => data_file.vec_at(block_data_offset, uncompressed_block_size as usize)
        };

        FFXIVAssetParserDatBlock {
            header_size,
            header_version,
            block_type,
            uncompressed_block_size,
            data,
        }
    }

    pub fn from_header<R: FFXIVBufferReader>(data_file: &mut FFXIVBuffer<R>, asset_dat_file_header: &FFXIVAssetParserDatHeader, data_file_offset: u64) -> Vec<FFXIVAssetParserDatBlock> {
        data_file.offset_set(data_file_offset + asset_dat_file_header.header_size as u64);
        asset_dat_file_header.blocks.iter().map(|block_metadata| {
            FFXIVAssetParserDatBlock::new(data_file, data_file_offset, asset_dat_file_header, block_metadata)
        }).collect()
    }
}

#[derive(Clone)]
pub enum FFXIVAssetParserDatBlockType {
    Compressed(u32),
    Uncompressed(u32),
}

impl FFXIVAssetParserDatBlockType {
    pub fn new(n: u32) -> FFXIVAssetParserDatBlockType {
        match n {
            32000 => FFXIVAssetParserDatBlockType::Uncompressed(32000),
            _ => FFXIVAssetParserDatBlockType::Compressed(n)
        }
    }
}




//==================================================================================================

#[derive(Debug, Clone)]
pub struct FFXIVAssetParserIndex1Data1Item {
    pub hash: u64,
    pub data: u32,
    pub data_file_id: u32,
    pub data_file_offset: u64,
}

#[derive(Debug, Clone)]
pub struct FFXIVAssetParserIndex2Data1Item {
    pub hash: u32,
    pub data: u32,
    pub data_file_id: u32,
    pub data_file_offset: u64,
}


#[derive(Debug, Clone)]
pub struct FFXIVAssetParserIndex<T> {
    pub file_signature: String,
    pub file_platform: FFXIVAssetPlatform,
    pub file_header_offset: u32,
    pub file_version: u32,
    pub file_type: u32,
    pub header_size: u32,
    pub header_type: u32,
    pub header_data_offset: u32,
    pub header_data_size: u32,
    pub header2_size: u32,
    pub header2_offset: u32,
    pub header2_empty_space_size: u32,
    pub header2_data_size: u32,
    pub header3_offset: u32,
    pub header3_data_size: u32,
    pub header4_offset: u32,
    pub header4_data_size: u32,
    pub data1: Vec<T>,
}

impl <T> FFXIVAssetParserIndex<T> {
    fn parse_header<R: FFXIVBufferReader>(buffer: &mut FFXIVBuffer<R>) -> FFXIVAssetParserIndex<T> {
        buffer.offset = 0;

        let file_signature = buffer.string_at(0x00, 0x08);
        let file_platform = FFXIVAssetPlatform::from_u32(buffer.u8_at(0x08) as u32).unwrap();
        let file_header_offset = buffer.le_u32_at(0x0C);
        let file_version = buffer.le_u32_at(0x10);
        let file_type = buffer.le_u32_at(0x10);

        let offset = file_header_offset as u64;

        let header_size = buffer.le_u32_at(offset);
        let header_type = buffer.le_u32_at(offset + 0x04);
        let header_data_offset = buffer.le_u32_at(offset + 0x08);
        let header_data_size = buffer.le_u32_at(offset + 0x0C);

        let offset = file_header_offset as u64 + 0x50;

        let header2_size = buffer.le_u32_at(offset);
        let header2_offset = buffer.le_u32_at(offset + 0x04);
        let header2_empty_space_size = buffer.le_u32_at(offset + 0x08);
        let header2_data_size = buffer.le_u32_at(offset + 0x0C);

        let offset = file_header_offset as u64 + 0x90;

        let header3_offset = buffer.le_u32_at(offset + 0x0C);
        let header3_data_size = buffer.le_u32_at(offset + 0x10);

        let offset = file_header_offset as u64 + 0xE0;

        let header4_offset = buffer.le_u32_at(offset + 0x04);
        let header4_data_size = buffer.le_u32_at(offset + 0x08);

        let mut data1: Vec<T> = Vec::new();


        FFXIVAssetParserIndex {
            file_signature,
            file_platform,
            file_header_offset,
            file_version,
            file_type,
            header_size,
            header_type,
            header_data_offset,
            header_data_size,
            header2_size,
            header2_offset,
            header2_empty_space_size,
            header2_data_size,
            header3_offset,
            header3_data_size,
            header4_offset,
            header4_data_size,
            data1,
        }
    }
}

//==================================================================================================

impl FFXIVAssetParserIndex<FFXIVAssetParserIndex1Data1Item> {
    pub fn from_index1_file<P: AsRef<Path>>(file_path: P) -> FFXIVAssetParserIndex<FFXIVAssetParserIndex1Data1Item> {
        let mut buffer = FFXIVBuffer::from_file_path(&file_path);
        let mut index = FFXIVAssetParserIndex::parse_header(&mut buffer);
        FFXIVAssetParserIndex::parse_index1_data(&mut buffer, &mut index.data1, index.header_data_offset as usize, index.header_data_size as usize);
        index
    }

    pub fn from_index1_vec(vec: Vec<u8>) -> FFXIVAssetParserIndex<FFXIVAssetParserIndex1Data1Item> {
        let mut buffer = FFXIVBuffer::from_vec(vec);
        let mut index = FFXIVAssetParserIndex::parse_header(&mut buffer);
        FFXIVAssetParserIndex::parse_index1_data(&mut buffer, &mut index.data1, index.header_data_offset as usize, index.header_data_size as usize);
        index
    }

    pub fn from_index1_buffer<R: FFXIVBufferReader>(buffer: &mut FFXIVBuffer<R>) -> FFXIVAssetParserIndex<FFXIVAssetParserIndex1Data1Item> {
        let mut index = FFXIVAssetParserIndex::parse_header(buffer);
        FFXIVAssetParserIndex::parse_index1_data(buffer, &mut index.data1, index.header_data_offset as usize, index.header_data_size as usize);
        index
    }

    pub fn contains(&self, hash: u64) -> bool {
        self.data1.iter().position(|item| item.hash == hash) == None
    }

    pub fn find(&self, hash: u64) -> Option<&FFXIVAssetParserIndex1Data1Item> {
        self.data1.iter().find(|item| item.hash == hash)
    }

    fn parse_index1_data<R: FFXIVBufferReader>(buffer: &mut FFXIVBuffer<R>, output: &mut Vec<FFXIVAssetParserIndex1Data1Item>, header_data_offset: usize, header_data_size: usize) {
        for offset_line in (0..header_data_size).step_by(16) {
            let offset = (header_data_offset + offset_line) as u64;
            let hash = buffer.le_u64_at(offset);
            let data = buffer.le_u32_at(offset + 0x08);
            let data_file_id = (data & 0b1110) >> 1;
            let data_file_offset = (data as u64 & !0xF) * 0x08;
            output.push(FFXIVAssetParserIndex1Data1Item {
                hash,
                data,
                data_file_id,
                data_file_offset,
            })
        };
    }


}

//==================================================================================================

impl FFXIVAssetParserIndex<FFXIVAssetParserIndex2Data1Item> {
    pub fn from_index2_file(file_path: &str) -> FFXIVAssetParserIndex<FFXIVAssetParserIndex2Data1Item> {
        let mut buffer = FFXIVBuffer::from_file_path(&file_path);
        let mut index = FFXIVAssetParserIndex::parse_header(&mut buffer);
        FFXIVAssetParserIndex::parse_index2_data(&mut buffer, &mut index.data1, index.header_data_offset as usize, index.header_data_size as usize);
        index
    }

    pub fn from_index2_vec(vec: Vec<u8>) -> FFXIVAssetParserIndex<FFXIVAssetParserIndex2Data1Item> {
        let mut buffer = FFXIVBuffer::from_vec(vec);
        let mut index = FFXIVAssetParserIndex::parse_header(&mut buffer);
        FFXIVAssetParserIndex::parse_index2_data(&mut buffer, &mut index.data1, index.header_data_offset as usize, index.header_data_size as usize);
        index
    }

    pub fn from_index2_buffer<R: FFXIVBufferReader>(buffer: &mut FFXIVBuffer<R>) -> FFXIVAssetParserIndex<FFXIVAssetParserIndex2Data1Item> {
        let mut index = FFXIVAssetParserIndex::parse_header(buffer);
        FFXIVAssetParserIndex::parse_index2_data(buffer, &mut index.data1, index.header_data_offset as usize, index.header_data_size as usize);
        index
    }

    fn parse_index2_data<R: FFXIVBufferReader>(buffer: &mut FFXIVBuffer<R>, output: &mut Vec<FFXIVAssetParserIndex2Data1Item>, header_data_offset: usize, header_data_size: usize) {
        for offset_line in (0..header_data_size).step_by(8) {
            let offset = (header_data_offset + offset_line) as u64;
            let hash = buffer.le_u32_at(offset);
            let data = buffer.le_u32_at(offset + 0x04);
            let data_file_id = (data & 0b1110) >> 1;
            let data_file_offset = (data as u64 & !0xF) * 0x08;
            output.push(FFXIVAssetParserIndex2Data1Item {
                hash,
                data,
                data_file_id,
                data_file_offset,
            })
        };
    }
}



//==================================================================================================

#[derive(Debug, Clone)]
pub struct FFXIVAssetPathFile {
    pub path: PathBuf,
    pub path_str: String,
    pub path_name: String,
    pub path_stem: String,
    pub path_extension: String,
    pub data_category: FFXIVAssetCategory,
    pub data_repository: FFXIVAssetRepository,
    pub data_chunk: FFXIVAssetChunk,
    pub data_platform: FFXIVAssetPlatform,
}

impl PartialEq for FFXIVAssetPathFile {
    fn eq(&self, other: &Self) -> bool {
        self.path_stem == other.path_stem
    }
}

impl FFXIVAssetPathFile {
    pub fn new(file_path: PathBuf) -> Result<FFXIVAssetPathFile, String> {
        let file_path_str = file_path.as_os_str().to_str().ok_or("Failed to convert path to str.")?;
        let file_name = file_path.file_name().ok_or("Failed to get file name.")?.to_str().ok_or("Failed to get file name as str.")?;
        let file_stem = file_path.file_stem().ok_or("Failed to get file name.")?.to_str().ok_or("Failed to get file name as str.")?;
        let file_extension = file_path.extension().ok_or("Failed to get file extension.")?.to_str().ok_or("Failed to get file extension as str.")?;
        //let file_path_components: Vec<Option<&str>> = file_path.components().map(|c| c.as_os_str().to_str()).collect();

        let file_name_bytes = file_name.as_bytes();
        let file_name_regex = RegByte::new(r"^(\d|[a-z]){6}\.(win32|ps3|ps4)\.(dat|index)\d*$").or(Err("Failed to create regex"))?;
        file_name_regex.captures(file_name_bytes).ok_or(format!("File name '{}' is invalid.", file_name))?;

        let category_str = String::from_utf8(file_name_bytes[0..2].to_vec()).or(Err("Failed to slice name to category"))?;
        let repository_str = String::from_utf8(file_name_bytes[2..4].to_vec()).or(Err("Failed to slice name to repository"))?;
        let chunk_str = String::from_utf8(file_name_bytes[4..6].to_vec()).or(Err("Failed to slice name to chunk"))?;

        let data_category = FFXIVAssetCategory::from_hex_str(&category_str)?;
        let data_repository = FFXIVAssetRepository::from_hex_str(&repository_str)?;
        let data_chunk = FFXIVAssetChunk::from_hex_str(&chunk_str)?;
        let data_platform = FFXIVAssetPlatform::from_str_contains(&file_name)?;


        Ok(
            FFXIVAssetPathFile {
                path_str: String::from(file_path_str),
                path: file_path.clone(),
                path_name: String::from(file_name),
                path_stem: String::from(file_stem),
                path_extension: String::from(file_extension),
                data_category,
                data_repository,
                data_chunk,
                data_platform,
            }
        )
    }
}

//==================================================================================================

#[derive(Debug, Clone)]
pub struct FFXIVAssetPathDat {
    pub path: PathBuf,
    pub path_str: String,
    pub path_extension: String,
    pub path_stem: String,
    pub path_dir: String,
    pub index1_hash: u64,
    pub index2_hash: u32,
    pub data_repo: FFXIVAssetRepository,
    pub data_category: FFXIVAssetCategory,
    pub data_platform: FFXIVAssetPlatform,
}

impl FFXIVAssetPathDat {
    pub fn from_str(path: &str) -> Result<FFXIVAssetPathDat, String> {
        let path = PathBuf::from(path);
        FFXIVAssetPathDat::new(path)
    }
    pub fn new(path: PathBuf) -> Result<FFXIVAssetPathDat, String> {
        let full_path = path.as_os_str().to_str().ok_or("Failed to convert path to str.")?.to_lowercase();
        let path_name = path.file_name().ok_or("Failed to get file name.")?.to_str().ok_or("Failed to get file name as str.")?;
        let path_stem = path.file_stem().ok_or("Failed to get file name.")?.to_str().ok_or("Failed to get file name as str.")?;
        let path_extension = path.extension().ok_or("Failed to get file extension.")?.to_str().ok_or("Failed to get file extension as str.")?;
        let path_dir = path.parent().ok_or("Failed to get parent dir.")?.to_str().ok_or("Failed to convert parent dir to str.")?;

        let components: Vec<Option<&str>> = path.components().map(|c| c.as_os_str().to_str()).collect();
        let data_category = FFXIVAssetCategory::from_str(components.get(0).ok_or("Failed to get category name.")?.ok_or("Failed to get category name as str.")?)?;
        let data_repo = FFXIVAssetRepository::from_str(components.get(1).ok_or("Failed to get repository name.")?.ok_or("Failed to get repository name as str.")?);

        let data_folder = FFXIVAssetPathDat::hash(path_dir);
        //let data_category_hash = AssetPath::hash(data_category.name.as_str());
        let data_name_hash = FFXIVAssetPathDat::hash(path_name);
        let data_platform = FFXIVAssetPlatform::from_u32(0)?;

        let index1_hash = FFXIVAssetPathDat::double_hash(data_folder, data_name_hash);
        let index2_hash = FFXIVAssetPathDat::hash(&full_path);

        Ok(
            FFXIVAssetPathDat {
                path: path.clone(),
                path_str: String::from(full_path),
                path_extension: String::from(path_extension),
                path_stem: String::from(path_stem),
                path_dir: String::from(path_dir),
                index1_hash,
                index2_hash,
                data_repo,
                data_category,
                data_platform,
            }
        )
    }


    fn hash(string: &str) -> u32 {
        let crc = Crc::<u32>::new(&CRC_32_JAMCRC);
        let mut digest = crc.digest();
        digest.update(string.as_bytes());
        digest.finalize()
    }

    fn double_hash(category_hash: u32, file_name_hash: u32) -> u64 {
        return ((category_hash as u64) << 32) | (file_name_hash as u64);
    }
}

//==================================================================================================

#[derive(Debug, Clone)]
pub struct FFXIVAssetCategory {
    pub name: String,
    pub id: u32
}

impl FFXIVAssetCategory {
    pub fn from_hex_str(cat_hex_str: &str) -> Result<FFXIVAssetCategory, String> {
        let expansion_id: u32 = u32::from_str_radix(cat_hex_str, 16).or(Err(format!("Failed to convert '{}' to number.", cat_hex_str)))?;
        FFXIVAssetCategory::from_number(expansion_id)
    }

    pub fn from_str(cat: &str) -> Result<FFXIVAssetCategory, String> {
        let id = match cat {
            "common" => 0x00,
            "bgcommon" => 0x01,
            "bg" => 0x02,
            "cut" => 0x03,
            "chara" => 0x04,
            "shader" => 0x05,
            "ui" => 0x06,
            "sound" => 0x07,
            "vfx" => 0x08,
            "ui_script" => 0x09,
            "exd" => 0x0A,
            "game_script" => 0x0B,
            "music" => 0x0C,
            "sqpack_test" => 0x12,
            "debug" => 0x13,
            _ => {
                return Err(format!("Category '{}' not found", cat));
            }
        };

        Ok(
            FFXIVAssetCategory {
                name: String::from(cat),
                id
            }
        )


    }

    pub fn from_number(cat: u32) -> Result<FFXIVAssetCategory, String> {
        let name = match cat {
            0x00 => "common",
            0x01 => "bgcommon",
            0x02 => "bg",
            0x03 => "cut",
            0x04 => "chara",
            0x05 => "shader",
            0x06 => "ui",
            0x07 => "sound",
            0x08 => "vfx",
            0x09 => "ui_script",
            0x0A => "exd",
            0x0B => "game_script",
            0x0C => "music",
            0x12 => "sqpack_test",
            0x13 => "debug",
            _ => {
                return Err(format!("Category '{}' not found", cat));
            }
        };

        Ok(
            FFXIVAssetCategory {
                name: String::from(name),
                id: cat
            }
        )


    }
}



//==================================================================================================


#[derive(Debug, Clone)]
pub struct FFXIVAssetChunk {
    pub hex: String,
    pub id: u32,
}

impl FFXIVAssetChunk {
    pub fn from_hex_str(chunk_hex_str: &str) -> Result<FFXIVAssetChunk, String> {
        let chunk_number: u32 = u32::from_str_radix(chunk_hex_str, 16).or(Err(format!("Failed to parse chunk '{}' to a number.", chunk_hex_str)))?;
        if chunk_number > 255 {
            return Err(format!("Chunk '{}' is out of range 0:255", chunk_number));
        }
        Ok(FFXIVAssetChunk {
            hex: String::from(chunk_hex_str),
            id: chunk_number,
        })
    }

    pub fn from_u32(chunk_number: u32) -> Result<FFXIVAssetChunk, String> {
        if chunk_number > 255 {
            return Err(format!("Chunk '{}' is out of range 0:255", chunk_number));
        }
        let chunk_name: String = format!("{:02x}", chunk_number);
        Ok(FFXIVAssetChunk {
            hex: chunk_name,
            id: chunk_number,
        })
    }

}

//==================================================================================================


#[derive(Debug, Clone)]
pub struct FFXIVAssetRepository {
    pub name: String,
    pub id: u32
}

impl FFXIVAssetRepository {
    pub fn from_hex_str(repo_hex_str: &str) -> Result<FFXIVAssetRepository, String> {
        let expansion_id: u32 = u32::from_str_radix(repo_hex_str, 16).or(Err(format!("Failed to convert '{}' to number.", repo_hex_str)))?;
        Ok(FFXIVAssetRepository::from_u32(expansion_id))
    }

    pub fn from_str(repo: &str) -> FFXIVAssetRepository {
        let regex = Reg::new(r"^ex\d+$").unwrap();
        let captured = regex.captures(repo);
        if let Some(r) = captured{
            let expansion: &str = &repo[2..];
            let expansion: Result<u32, _> = expansion.parse();
            if let Ok(id) = expansion{
                return FFXIVAssetRepository {
                    name: String::from(repo),
                    id
                };
            }
        }
        FFXIVAssetRepository {
            name: String::from("ffxiv"),
            id: 0
        }
    }

    pub fn from_u32(number: u32) -> FFXIVAssetRepository {
        let mut expansion = String::new();
        if number > 0 {
            expansion = format!("ex{}", number);
        } else {
            expansion = String::from("ffxiv")
        }
        FFXIVAssetRepository {
            id: number,
            name: expansion
        }
    }
}

//==================================================================================================

#[derive(Debug, Clone)]
pub struct FFXIVAssetPlatform {
    pub name: String,
    pub id: u32
}

impl FFXIVAssetPlatform {
    pub fn from_u32(n: u32) -> Result<FFXIVAssetPlatform, String> {
        let name = match n {
            0 => "win32",
            1 => "ps3",
            2 => "ps4",
            _ => {
                return Err(format!("Platform id out of range 0:2, got: {}", n));
            }
        };

        Ok(FFXIVAssetPlatform {
            name: String::from(name),
            id: n,
        })
    }

    pub fn from_str(platform: &str) -> Result<FFXIVAssetPlatform, String> {
        let id = match platform {
            "win32" => 0u32,
            "ps3" => 1u32,
            "ps4" => 2u32,
            _ => {
                return Err(format!("Platform '{}' not found.", platform));
            }
        };

        Ok(FFXIVAssetPlatform {
            name: String::from(platform),
            id,
        })
    }

    pub fn from_hex_str(platform_hex_str: &str) -> Result<FFXIVAssetPlatform, String> {
        let expansion_id: u32 = u32::from_str_radix(platform_hex_str, 16).or(Err(format!("Failed to convert '{}' to number.", platform_hex_str)))?;
        FFXIVAssetPlatform::from_u32(expansion_id)
    }

    pub fn from_str_contains(string: &str) -> Result<FFXIVAssetPlatform, String> {
        let mut name: String = String::new();
        let mut id = 0;
        if string.contains("win32") {
            name = String::from("win32");
        } else if string.contains("ps3") {
            name = String::from("ps3");
            id = 3;
        } else if string.contains("ps4") {
            name = String::from("ps4");
            id = 4;
        } else {
            return Err(String::from("String doesn't contain win32, ps3 or ps4"));
        }

        Ok(
            FFXIVAssetPlatform {
                name,
                id
            }
        )
    }
}