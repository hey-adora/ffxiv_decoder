use crate::ffxiv::asset_exh_file::{AssetEXHFile, AssetEXHFileColumnKind};
use crate::ffxiv::buffer_vec::BufferVec;

pub struct AssetEXDFile {
    pub signature: String,
    pub version: u16,
    pub uwknown1: u16,
    pub row_metadata_size: u32,
    pub row_data_size: u32,
    pub rows_metadata: Vec<AssetExdFileRowMetadata>,
    pub rows: Vec<Vec<AssetEXDFileColumnKind>>
}

pub struct AssetExdFileRowMetadata {
    pub id: u32,
    pub offset: u32
}

pub enum AssetEXDFileColumnKind {
    String(String),
    Bool(bool),
    Int8(i8),
    UInt8(u8),
    Int16(i16),
    UInt16(u16),
    Int32(i32),
    UInt32(u32),
    Float32(f32),
    Int64(i64),
    UInt64(u64),
    UNK,
}

impl AssetEXDFile {
    pub fn new(buffer: &mut BufferVec, exh: &AssetEXHFile)  -> AssetEXDFile {
        let signature = buffer.string(0x04);
        let version = buffer.be_u16();
        let uwknown1 = buffer.be_u16();
        let row_metadata_size = buffer.be_u32();
        let row_data_size = buffer.be_u32();
        buffer.offset_set(0x20);
        let max_file_size = (0x20 + row_metadata_size + row_data_size) as usize;
        let row_count = row_metadata_size/8;
        let rows_metadata: Vec<AssetExdFileRowMetadata> = (0..row_count).map(|i| AssetExdFileRowMetadata::new(buffer)).collect();
        let rows: Vec<Vec<AssetEXDFileColumnKind>> = rows_metadata.iter().map(|row_metadata| {
            let mut columns: Vec<AssetEXDFileColumnKind> = Vec::new();
            for column in &exh.columns {
                let mut data_row_column_offset = row_metadata.offset as usize + column.offset as usize + exh.data_offset as usize + 6;
                if data_row_column_offset >= max_file_size {
                    let column_size = AssetEXHFileColumnKind::sizes(&column.kind);
                    data_row_column_offset = max_file_size - column_size;
                }
                let data = AssetEXDFileColumnKind::new_at(buffer, &column.kind, data_row_column_offset);
                columns.push(data);
            }
            columns
        }).collect();

        AssetEXDFile {
            signature,
            version,
            uwknown1,
            row_metadata_size,
            row_data_size,
            rows_metadata,
            rows
        }
    }
}

impl AssetExdFileRowMetadata {
    pub fn new(buffer: &mut BufferVec) -> AssetExdFileRowMetadata {
        AssetExdFileRowMetadata {
            id: buffer.be_u32(),
            offset: buffer.be_u32(),
        }
    }
}

impl AssetEXDFileColumnKind {
    pub fn new_at(buffer: &mut BufferVec, kind: &AssetEXHFileColumnKind, offset: usize) -> AssetEXDFileColumnKind {
        match kind {
            AssetEXHFileColumnKind::String => {
                let mut string: String = String::new();
                let mut index = 0;
                loop {
                    let at = offset + index;
                    let current_char: u8 = buffer.u8_at(at);
                    if current_char == 0 {
                        break;
                    } else {
                        string.push(current_char as char)
                    }
                    index += 1;
                }
                AssetEXDFileColumnKind::String(string)

            },
            AssetEXHFileColumnKind::Bool => AssetEXDFileColumnKind::Bool(buffer.u8_at(offset) == 0),
            AssetEXHFileColumnKind::Int8 => AssetEXDFileColumnKind::Int8(buffer.i8_at(offset)),
            AssetEXHFileColumnKind::UInt8 => AssetEXDFileColumnKind::UInt8(buffer.u8_at(offset)),
            AssetEXHFileColumnKind::Int16 => AssetEXDFileColumnKind::Int16(buffer.be_i16_at(offset)),
            AssetEXHFileColumnKind::UInt16 => AssetEXDFileColumnKind::UInt16(buffer.be_u16_at(offset)),
            AssetEXHFileColumnKind::Int32 => AssetEXDFileColumnKind::Int32(buffer.be_i32_at(offset)),
            AssetEXHFileColumnKind::UInt32 => AssetEXDFileColumnKind::UInt32(buffer.be_u32_at(offset)),
            AssetEXHFileColumnKind::UNK1 => AssetEXDFileColumnKind::UNK,
            AssetEXHFileColumnKind::Float32 => AssetEXDFileColumnKind::Float32(buffer.be_f32_at(offset)),
            AssetEXHFileColumnKind::Int64 => AssetEXDFileColumnKind::Int64(buffer.be_i64_at(offset)),
            AssetEXHFileColumnKind::UInt64 => AssetEXDFileColumnKind::UInt64(buffer.be_u64_at(offset)),
            AssetEXHFileColumnKind::UNK2 => AssetEXDFileColumnKind::UNK,
            AssetEXHFileColumnKind::PackedBool0 => {
                let byte = buffer.u8_at(offset);
                AssetEXDFileColumnKind::Bool(byte & 0 == byte)
            },
            AssetEXHFileColumnKind::PackedBool1 => {
                let byte = buffer.u8_at(offset);
                AssetEXDFileColumnKind::Bool(byte & (1 << 1) == byte)
            },
            AssetEXHFileColumnKind::PackedBool2 => {
                let byte = buffer.u8_at(offset);
                AssetEXDFileColumnKind::Bool(byte & (1 << 2) == byte)
            },
            AssetEXHFileColumnKind::PackedBool3 => {
                let byte = buffer.u8_at(offset);
                AssetEXDFileColumnKind::Bool(byte & (1 << 3) == byte)
            },
            AssetEXHFileColumnKind::PackedBool4 => {
                let byte = buffer.u8_at(offset);
                AssetEXDFileColumnKind::Bool(byte & (1 << 4) == byte)
            },
            AssetEXHFileColumnKind::PackedBool5 => {
                let byte = buffer.u8_at(offset);
                AssetEXDFileColumnKind::Bool(byte & (1 << 5) == byte)
            },
            AssetEXHFileColumnKind::PackedBool6 => {
                let byte = buffer.u8_at(offset);
                AssetEXDFileColumnKind::Bool(byte & (1 << 6) == byte)
            },
            AssetEXHFileColumnKind::PackedBool7 => {
                let byte = buffer.u8_at(offset);
                AssetEXDFileColumnKind::Bool(byte & (1 << 7) == byte)
            },
        }
    }
}

