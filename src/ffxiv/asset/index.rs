use std::path::Path;
use crate::ffxiv::buffer::{Buffer, BufferReader};
use crate::ffxiv::metadata::Platform;

#[derive(Debug, Clone)]
pub struct Index<T> {
    pub file_signature: String,
    pub file_platform: Platform,
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

impl <T> Index<T> {
    fn parse_header<R: BufferReader>(buffer: &mut Buffer<R>) -> Index<T> {
        buffer.offset_set(0);

        let file_signature = buffer.string_at(0x00, 0x08);
        let file_platform = Platform::from_u32(buffer.u8_at(0x08) as u32).unwrap();
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


        Index {
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

#[derive(Debug, Clone)]
pub struct Index1Data1Item {
    pub hash: u64,
    pub data: u32,
    pub data_file_id: u32,
    pub data_file_offset: u64,
}

impl Index<Index1Data1Item> {
    pub fn from_index1_file<P: AsRef<Path>>(file_path: P) -> Index<Index1Data1Item> {
        let mut buffer = Buffer::from_file_path(&file_path);
        let mut index = Index::parse_header(&mut buffer);
        Index::parse_index1_data(&mut buffer, &mut index.data1, index.header_data_offset as usize, index.header_data_size as usize);
        index
    }

    pub fn from_index1_vec(vec: Vec<u8>) -> Index<Index1Data1Item> {
        let mut buffer = Buffer::from_vec(vec);
        let mut index = Index::parse_header(&mut buffer);
        Index::parse_index1_data(&mut buffer, &mut index.data1, index.header_data_offset as usize, index.header_data_size as usize);
        index
    }

    pub fn from_index1_buffer<R: BufferReader>(buffer: &mut Buffer<R>) -> Index<Index1Data1Item> {
        let mut index = Index::parse_header(buffer);
        Index::parse_index1_data(buffer, &mut index.data1, index.header_data_offset as usize, index.header_data_size as usize);
        index
    }

    pub fn contains(&self, hash: u64) -> bool {
        self.data1.iter().position(|item| item.hash == hash) == None
    }

    pub fn find(&self, hash: u64) -> Option<&Index1Data1Item> {
        self.data1.iter().find(|item| item.hash == hash)
    }

    fn parse_index1_data<R: BufferReader>(buffer: &mut Buffer<R>, output: &mut Vec<Index1Data1Item>, header_data_offset: usize, header_data_size: usize) {
        for offset_line in (0..header_data_size).step_by(16) {
            let offset = (header_data_offset + offset_line) as u64;
            let hash = buffer.le_u64_at(offset);
            let data = buffer.le_u32_at(offset + 0x08);
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

//==================================================================================================

#[derive(Debug, Clone)]
pub struct Index2Data1Item {
    pub hash: u32,
    pub data: u32,
    pub data_file_id: u32,
    pub data_file_offset: u64,
}

impl Index<Index2Data1Item> {
    pub fn from_index2_file(file_path: &str) -> Index<Index2Data1Item> {
        let mut buffer = Buffer::from_file_path(&file_path);
        let mut index = Index::parse_header(&mut buffer);
        Index::parse_index2_data(&mut buffer, &mut index.data1, index.header_data_offset as usize, index.header_data_size as usize);
        index
    }

    pub fn from_index2_vec(vec: Vec<u8>) -> Index<Index2Data1Item> {
        let mut buffer = Buffer::from_vec(vec);
        let mut index = Index::parse_header(&mut buffer);
        Index::parse_index2_data(&mut buffer, &mut index.data1, index.header_data_offset as usize, index.header_data_size as usize);
        index
    }

    pub fn from_index2_buffer<R: BufferReader>(buffer: &mut Buffer<R>) -> Index<Index2Data1Item> {
        let mut index = Index::parse_header(buffer);
        Index::parse_index2_data(buffer, &mut index.data1, index.header_data_offset as usize, index.header_data_size as usize);
        index
    }

    fn parse_index2_data<R: BufferReader>(buffer: &mut Buffer<R>, output: &mut Vec<Index2Data1Item>, header_data_offset: usize, header_data_size: usize) {
        for offset_line in (0..header_data_size).step_by(8) {
            let offset = (header_data_offset + offset_line) as u64;
            let hash = buffer.le_u32_at(offset);
            let data = buffer.le_u32_at(offset + 0x04);
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
