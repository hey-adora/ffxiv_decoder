use std::ffi::c_ushort;
use imgui_winit_support::winit::platform::unix::x11::ffi::Bool;
use crate::ffxiv::buffer_vec::BufferVec;

pub struct AssetEXHFile {
    pub signature: String,
    pub version: u16,
    pub data_offset: u16,
    pub column_count: u16,
    pub page_count: u16,
    pub language_count: u16,
    pub variant: u16,
    pub row_count: u32,
    pub columns: Vec<AssetEXHFileColumn>,
    pub rows: Vec<AssetEXHFileRow>,
    pub languages: Vec<AssetEXHFileLanguage>,
}

pub struct AssetEXHFileColumn {
    pub kind: AssetEXHFileColumnKind,
    pub offset: u16
}

pub struct AssetEXHFileRow {
    pub start_id: u32,
    pub row_count: u32
}

pub enum AssetEXHFileLanguage {
    None = 0,
    Japanese = 1,
    English = 2,
    German = 3,
    French = 4,
    ChineseSimplified = 5,
    ChineseTraditional = 6,
    Korean = 7
}

pub enum AssetEXHFileColumnKind {
    String = 0x00,
    Bool = 0x01,
    Int8 = 0x02,
    UInt8 = 0x03,
    Int16 = 0x04,
    UInt16 = 0x05,
    Int32 = 0x06,
    UInt32 = 0x07,
    UNK1 = 0x08, // unused?
    Float32 = 0x09,
    Int64 = 0x0A,
    UInt64 = 0x0B,
    UNK2 = 0x0C, // unused?
    PackedBool0 = 0x19, // 0 is read like data & 1, 1 is like data & 2, 2 = data & 4, etc...
    PackedBool1 = 0x1A,
    PackedBool2 = 0x1B,
    PackedBool3 = 0x1C,
    PackedBool4 = 0x1D,
    PackedBool5 = 0x1E,
    PackedBool6 = 0x1F,
    PackedBool7 = 0x20,

}


impl AssetEXHFileColumnKind {
    pub fn sizes(kind: &AssetEXHFileColumnKind) -> usize {
        match kind {
            AssetEXHFileColumnKind::String => 1,
            AssetEXHFileColumnKind::Bool => 1,
            AssetEXHFileColumnKind::Int8 => 1,
            AssetEXHFileColumnKind::UInt8 => 1,
            AssetEXHFileColumnKind::Int16 => 2,
            AssetEXHFileColumnKind::UInt16 => 2,
            AssetEXHFileColumnKind::Int32 => 4,
            AssetEXHFileColumnKind::UInt32 => 4,
            AssetEXHFileColumnKind::UNK1 => 0,
            AssetEXHFileColumnKind::Float32 => 4,
            AssetEXHFileColumnKind::Int64 => 8,
            AssetEXHFileColumnKind::UInt64 => 8,
            AssetEXHFileColumnKind::UNK2 => 0,
            AssetEXHFileColumnKind::PackedBool0 => 1,
            AssetEXHFileColumnKind::PackedBool1 => 1,
            AssetEXHFileColumnKind::PackedBool2 => 1,
            AssetEXHFileColumnKind::PackedBool3 => 1,
            AssetEXHFileColumnKind::PackedBool4 => 1,
            AssetEXHFileColumnKind::PackedBool5 => 1,
            AssetEXHFileColumnKind::PackedBool6 => 1,
            AssetEXHFileColumnKind::PackedBool7 => 1,
        }
    }
}



impl AssetEXHFile {
    pub fn new(buffer: &mut BufferVec) -> AssetEXHFile {
        //let mut offset = buffer.offset_set(0);

        let signature: String = buffer.string(0x04);
        let version: u16 = buffer.be_u16();
        let data_offset: u16 = buffer.be_u16();
        let column_count: u16 = buffer.be_u16();
        let page_count: u16 = buffer.be_u16();
        let language_count: u16 = buffer.be_u16();

        buffer.offset_skip(0x02);

        let variant = buffer.be_u16();

        buffer.offset_skip(0x02);

        let row_count = buffer.be_u32();

        buffer.offset_skip(0x08);

        let columns: Vec<AssetEXHFileColumn> = (0..column_count).map(|i| AssetEXHFileColumn::new(buffer)).collect();
        let rows: Vec<AssetEXHFileRow> = (0..page_count).map(|i| AssetEXHFileRow::new(buffer)).collect();
        let languages: Vec<AssetEXHFileLanguage> = (0..language_count).map(|i| AssetEXHFileLanguage::new(buffer)).collect();


        AssetEXHFile {
            signature,
            version,
            data_offset,
            column_count,
            page_count,
            language_count,
            variant,
            row_count,
            columns,
            rows,
            languages
        }
    }
}

impl AssetEXHFileLanguage {
    pub fn new(buffer: &mut BufferVec) -> AssetEXHFileLanguage {
        let lang = buffer.le_u16();
        match lang {
            0 => AssetEXHFileLanguage::None,
            1 => AssetEXHFileLanguage::Japanese,
            2 => AssetEXHFileLanguage::English,
            3 => AssetEXHFileLanguage::German,
            4 => AssetEXHFileLanguage::French,
            5 => AssetEXHFileLanguage::ChineseSimplified,
            6 => AssetEXHFileLanguage::ChineseTraditional,
            7 => AssetEXHFileLanguage::Korean,
            _ => panic!("Langauge not found")
        }
    }
}

impl AssetEXHFileColumn {
    pub fn new(buffer: &mut BufferVec) -> AssetEXHFileColumn {
        let kind = buffer.be_u16();
        // String = 0x00,
        // Bool = 0x01,
        // Int8 = 0x02,
        // UInt8 = 0x03,
        // Int16 = 0x04,
        // UInt16 = 0x05,
        // Int32 = 0x06,
        // UInt32 = 0x07,
        // UNK1 = 0x08, // unused?
        // Float32 = 0x09,
        // Int64 = 0x0A,
        // UInt64 = 0x0B,
        // UNK2 = 0x0C, // unused?
        // PackedBool0 = 0x19, // 0 is read like data & 1, 1 is like data & 2, 2 = data & 4, etc...
        // PackedBool1 = 0x1A,
        // PackedBool2 = 0x1B,
        // PackedBool3 = 0x1C,
        // PackedBool4 = 0x1D,
        // PackedBool5 = 0x1E,
        // PackedBool6 = 0x1F,
        // PackedBool7 = 0x20,
        AssetEXHFileColumn {
            kind: match kind {
                0x00 => AssetEXHFileColumnKind::String,
                0x01 => AssetEXHFileColumnKind::Bool,
                0x02 => AssetEXHFileColumnKind::Int8,
                0x03 => AssetEXHFileColumnKind::UInt8,
                0x04 => AssetEXHFileColumnKind::Int16,
                0x05 => AssetEXHFileColumnKind::UInt16,
                0x06 => AssetEXHFileColumnKind::Int32,
                0x07 => AssetEXHFileColumnKind::UInt32,
                0x08 => AssetEXHFileColumnKind::UNK1,
                0x09 => AssetEXHFileColumnKind::Float32,
                0x0A => AssetEXHFileColumnKind::Int64,
                0x0B => AssetEXHFileColumnKind::UInt64,
                0x0C => AssetEXHFileColumnKind::UNK2,
                0x19 => AssetEXHFileColumnKind::PackedBool0,
                0x1A => AssetEXHFileColumnKind::PackedBool1,
                0x1B => AssetEXHFileColumnKind::PackedBool2,
                0x1C => AssetEXHFileColumnKind::PackedBool3,
                0x1D => AssetEXHFileColumnKind::PackedBool4,
                0x1E => AssetEXHFileColumnKind::PackedBool5,
                0x1F => AssetEXHFileColumnKind::PackedBool6,
                0x20 => AssetEXHFileColumnKind::PackedBool7,
                _ => panic!("Asset column kind '{}'", kind)
            },
            offset: buffer.be_u16(),
        }
    }
}

impl AssetEXHFileRow {
    pub fn new(buffer: &mut BufferVec) -> AssetEXHFileRow {
        AssetEXHFileRow {
            start_id: buffer.be_u32(),
            row_count: buffer.be_u32(),
        }
    }
}