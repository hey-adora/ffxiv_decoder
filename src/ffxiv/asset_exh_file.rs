use std::ffi::c_ushort;
use crate::ffxiv::buffer_vec::BufferVec;

pub struct AssetEXHFile {
    signature: String,
    version: u16,
    data_offset: u16,
    column_count: u16,
    page_count: u16,
    language_count: u16,
    variant: u16,
    row_count: u32,
    columns: Vec<AssetEXHFileColumn>,
    rows: Vec<AssetEXHFileRow>,
    languages: Vec<AssetEXHFileLanguage>,
}

pub struct AssetEXHFileColumn {
    kind: u16,
    offset: u16
}

pub struct AssetEXHFileRow {
    start_id: u32,
    row_count: u32
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
        match buffer.le_u16() {
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
        AssetEXHFileColumn {
            kind: buffer.be_u16(),
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