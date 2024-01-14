use std::fmt::Error;
use std::io;
use std::path::{Path, PathBuf};
use crc::{Crc, CRC_32_JAMCRC, Digest};
use crate::ffxiv::buffer_vec::BufferVec;
use crate::ffxiv::asset_file_platform::AssetFilePlatform;

#[derive(Debug)]
pub struct Index1Data1Item {
    pub hash: u64,
    pub data: u32,
    pub data_file_id: u32,
    pub data_file_offset: u64,
}

#[derive(Debug)]
pub struct Index2Data1Item {
    pub hash: u32,
    pub data: u32,
    pub data_file_id: u32,
    pub data_file_offset: u64,
}


#[derive(Debug)]
pub struct AssetIndexFile<T> {
    pub file_signature: String,
    pub file_platform: AssetFilePlatform,
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

trait AssetIndexFileParser<T> {
    fn parse_header(buffer: &mut BufferVec) -> AssetIndexFile<T> {
        let file_signature = buffer.string(0x00, 0x08);
        let file_platform = AssetFilePlatform::from_u32(buffer.u8(0x08) as u32).unwrap();
        let file_header_offset = buffer.u32(0x0C);
        let file_version = buffer.u32(0x10);
        let file_type = buffer.u32(0x10);

        let offset = file_header_offset as usize;

        let header_size = buffer.u32(offset);
        let header_type = buffer.u32(offset + 0x04);
        let header_data_offset = buffer.u32(offset + 0x08);
        let header_data_size = buffer.u32(offset + 0x0C);

        let offset = file_header_offset as usize + 0x50;

        let header2_size = buffer.u32(offset);
        let header2_offset = buffer.u32(offset + 0x04);
        let header2_empty_space_size = buffer.u32(offset + 0x08);
        let header2_data_size = buffer.u32(offset + 0x0C);

        let offset = file_header_offset as usize + 0x90;

        let header3_offset = buffer.u32(offset + 0x0C);
        let header3_data_size = buffer.u32(offset + 0x10);

        let offset = file_header_offset as usize + 0xE0;

        let header4_offset = buffer.u32(offset + 0x04);
        let header4_data_size = buffer.u32(offset + 0x08);

        let mut data1: Vec<T> = Vec::new();


        AssetIndexFile {
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

impl AssetIndexFileParser<Index1Data1Item> for AssetIndexFile<Index1Data1Item> {

}


impl AssetIndexFileParser<Index2Data1Item> for AssetIndexFile<Index2Data1Item> {

}

impl AssetIndexFile<Index1Data1Item> {
    pub fn from_index1(buffer: &mut BufferVec) -> AssetIndexFile<Index1Data1Item> {
        let mut index = AssetIndexFile::parse_header(buffer);
        AssetIndexFile::parse_index1_data(buffer, &mut index.data1, index.header_data_offset as usize, index.header_data_size as usize);
        index
    }

    fn parse_index1_data(buffer: &mut BufferVec, output: &mut Vec<Index1Data1Item>, header_data_offset: usize, header_data_size: usize) {
        for offset_line in (0..header_data_size).step_by(16) {
            let offset = (header_data_offset + offset_line) as usize;
            let hash = buffer.u64(offset);
            let data = buffer.u32(offset + 0x08);
            let data_file_id = (data & 0b1110) >> 1;
            let data_file_offset = (data as u64 & !0xF) * 0x08;
            output.push(Index1Data1Item {
                hash,
                data,
                data_file_id,
                data_file_offset,
            })
        };
    }
}

impl AssetIndexFile<Index2Data1Item> {
    pub fn from_index2(buffer: &mut BufferVec) -> AssetIndexFile<Index2Data1Item> {
        let mut index = AssetIndexFile::parse_header(buffer);
        AssetIndexFile::parse_index2_data(buffer, &mut index.data1, index.header_data_offset as usize, index.header_data_size as usize);
        index
    }

    fn parse_index2_data(buffer: &mut BufferVec, output: &mut Vec<Index2Data1Item>, header_data_offset: usize, header_data_size: usize) {
        for offset_line in (0..header_data_size).step_by(8) {
            let offset = (header_data_offset + offset_line) as usize;
            let hash = buffer.u32(offset);
            let data = buffer.u32(offset + 0x04);
            let data_file_id = (data & 0b1110) >> 1;
            let data_file_offset = (data as u64 & !0xF) * 0x08;
            output.push(Index2Data1Item {
                hash,
                data,
                data_file_id,
                data_file_offset,
            })
        };
    }
}


