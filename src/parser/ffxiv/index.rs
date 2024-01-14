use crate::reader::Buffer;

#[derive(Debug)]
pub enum Platform {
    Win32 = 0,
    PS3 = 1,
    PS4 = 2,
}

impl Platform {
    pub fn from(n: u8) -> Platform {
        match n {
            1 => Platform::PS3,
            2 => Platform::PS4,
            _ => Platform::Win32
        }
    }
}

#[derive(Debug)]
pub struct Data1 {
    pub hash: u64,
    pub data: u32,
    pub data_file_offset: u64,
    pub data_file_part: u32
}

#[derive(Debug)]
pub struct Metadata {
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
    pub data1: Vec<Data1>
}

impl Metadata {
    pub fn new(buffer: &mut Buffer, file_name: &str) -> Metadata {
        
        let file_signature = buffer.string(0x00, 0x08);
        let file_platform = Platform::from(buffer.u8(0x08));
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

        let mut data1: Vec<Data1> = Vec::new();
        for offset_line in (0..header_data_size / 16).step_by(16) {
            let offset = (header_data_offset + offset_line) as usize;
            let hash = buffer.u64(offset);
            let data = buffer.u32(offset + 0x08) ;
            let data_file_offset =  (data as u64 & !0xF ) * 0x08;
            let data_file_part = (data & 0b1110) >> 1;
            data1.push(Data1 {
                hash,
                data,
                data_file_offset,
                data_file_part
            })
        }
        
        
        Metadata {
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
            data1
        }
    }
}

