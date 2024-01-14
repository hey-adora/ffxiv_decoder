use crate::ffxiv::asset::exh::{EXH, EXHColumnKind};
use crate::ffxiv::buffer::{Buffer, BufferReader};

pub struct EXD {
    pub signature: String,
    pub version: u16,
    pub uwknown1: u16,
    pub row_metadata_size: u32,
    pub row_data_size: u32,
    pub rows_metadata: Vec<AssetExdFileRowMetadata>,
    pub rows: Vec<Vec<EXDColumn>>
}

pub struct AssetExdFileRowMetadata {
    pub id: u32,
    pub offset: u32
}

pub enum EXDColumn {
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

impl EXD {
    pub fn new<R: BufferReader>(buffer: &mut Buffer<R>, exh: &EXH) -> EXD {
        buffer.offset_set(0);

        let signature = buffer.string(0x04);
        let version = buffer.be_u16();
        let uwknown1 = buffer.be_u16();
        let row_metadata_size = buffer.be_u32();
        let row_data_size = buffer.be_u32();

        buffer.offset_set(0x20);

        let max_file_size = (0x20 + row_metadata_size + row_data_size) as u64;
        let row_count = row_metadata_size/8;
        let rows_metadata: Vec<AssetExdFileRowMetadata> = (0..row_count).map(|i| AssetExdFileRowMetadata::new(buffer)).collect();
        let rows: Vec<Vec<EXDColumn>> = rows_metadata.iter().map(|row_metadata| {
            let mut columns: Vec<EXDColumn> = Vec::new();
            for column in &exh.columns {
                let mut data_row_column_offset = row_metadata.offset as u64 + column.offset as u64 + exh.data_offset as u64 + 6;
                let column_size = EXHColumnKind::sizes(&column.kind);
                if data_row_column_offset + column_size >= max_file_size {
                    data_row_column_offset = max_file_size - column_size;
                }
                let data = EXDColumn::new_at(buffer, &column.kind, data_row_column_offset);
                columns.push(data);
            }
            columns
        }).collect();

        EXD {
            signature,
            version,
            uwknown1,
            row_metadata_size,
            row_data_size,
            rows_metadata,
            rows,
        }
    }

    pub fn from_vec(value: Vec<u8>, exh: &EXH) -> EXD {
        let mut buff = Buffer::from_vec(value);
        let output = EXD::new(&mut buff, exh);
        output
    }

}

impl AssetExdFileRowMetadata {
    pub fn new<R: BufferReader>(buffer: &mut Buffer<R>) -> AssetExdFileRowMetadata {
        AssetExdFileRowMetadata {
            id: buffer.be_u32(),
            offset: buffer.be_u32(),
        }
    }
}

impl EXDColumn {
    pub fn new_at<R: BufferReader>(buffer: &mut Buffer<R>, kind: &EXHColumnKind, offset: u64) -> EXDColumn {
        match kind {
            EXHColumnKind::String => {
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
                EXDColumn::String(string)

            },
            EXHColumnKind::Bool => EXDColumn::Bool(buffer.u8_at(offset) == 0),
            EXHColumnKind::Int8 => EXDColumn::Int8(buffer.i8_at(offset)),
            EXHColumnKind::UInt8 => EXDColumn::UInt8(buffer.u8_at(offset)),
            EXHColumnKind::Int16 => EXDColumn::Int16(buffer.be_i16_at(offset)),
            EXHColumnKind::UInt16 => EXDColumn::UInt16(buffer.be_u16_at(offset)),
            EXHColumnKind::Int32 => EXDColumn::Int32(buffer.be_i32_at(offset)),
            EXHColumnKind::UInt32 => EXDColumn::UInt32(buffer.be_u32_at(offset)),
            EXHColumnKind::UNK1 => EXDColumn::UNK,
            EXHColumnKind::Float32 => EXDColumn::Float32(buffer.be_f32_at(offset)),
            EXHColumnKind::Int64 => EXDColumn::Int64(buffer.be_i64_at(offset)),
            EXHColumnKind::UInt64 => EXDColumn::UInt64(buffer.be_u64_at(offset)),
            EXHColumnKind::UNK2 => EXDColumn::UNK,
            EXHColumnKind::PackedBool0 => {
                let byte = buffer.u8_at(offset);
                EXDColumn::Bool(byte & 0 == byte)
            },
            EXHColumnKind::PackedBool1 => {
                let byte = buffer.u8_at(offset);
                EXDColumn::Bool(byte & (1 << 1) == byte)
            },
            EXHColumnKind::PackedBool2 => {
                let byte = buffer.u8_at(offset);
                EXDColumn::Bool(byte & (1 << 2) == byte)
            },
            EXHColumnKind::PackedBool3 => {
                let byte = buffer.u8_at(offset);
                EXDColumn::Bool(byte & (1 << 3) == byte)
            },
            EXHColumnKind::PackedBool4 => {
                let byte = buffer.u8_at(offset);
                EXDColumn::Bool(byte & (1 << 4) == byte)
            },
            EXHColumnKind::PackedBool5 => {
                let byte = buffer.u8_at(offset);
                EXDColumn::Bool(byte & (1 << 5) == byte)
            },
            EXHColumnKind::PackedBool6 => {
                let byte = buffer.u8_at(offset);
                EXDColumn::Bool(byte & (1 << 6) == byte)
            },
            EXHColumnKind::PackedBool7 => {
                let byte = buffer.u8_at(offset);
                EXDColumn::Bool(byte & (1 << 7) == byte)
            },
        }
    }

    pub fn to_string(&self) -> String {
        match &self {
            EXDColumn::String(str) => str.to_owned(),
            EXDColumn::Bool(bool) => bool.to_string(),
            EXDColumn::Int8(i) => i.to_string(),
            EXDColumn::UInt8(i) => i.to_string(),
            EXDColumn::Int16(i) => i.to_string(),
            EXDColumn::UInt16(i) => i.to_string(),
            EXDColumn::Int32(i) => i.to_string(),
            EXDColumn::UInt32(i) => i.to_string(),
            EXDColumn::Float32(i) => i.to_string(),
            EXDColumn::Int64(i) => i.to_string(),
            EXDColumn::UInt64(i) => i.to_string(),
            EXDColumn::UNK => String::from("UNKNOWN")
        }
    }
}
