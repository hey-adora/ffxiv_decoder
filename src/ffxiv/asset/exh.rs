use crate::ffxiv::buffer::{Buffer, BufferReader};
use std::fmt::{Display, Formatter};

// pub fn sudo_check(auth: &Auth, requires_sudo: Option<u32>, user_id: &str) -> bool {
//     if let Some(required_sudo_group) = requires_sudo {
//         if let Some(sudoers) = auth.sudoers.get(&required_sudo_group) {
//             sudoers.contains(&String::from(user_id))
//         } else {
//             false
//         }
//     } else {
//         true
//     }
// }

//static required_sudo_group: u32 = 0;

// struct Auth {
//     pub sudoers: HashMap<u32, String>
// }
//
// impl Auth {
//     // pub fn get() -> Option<String> {
//     //     None
//     // }
// }
//
// pub fn sudo_check(auth: &Auth, requires_sudo: Option<u32>, user_id: &str) -> bool {
//     requires_sudo.and_then(|required_sudo_group| auth.sudoers.get(&required_sudo_group).and_then(|sudoers| sudoers.find(&String::from(user_id)) ))
//     match requires_sudo {
//         Some(required_sudo_group) => auth.sudoers.get(&required_sudo_group).and_then(|sudoers| sudoers.find(&String::from(user_id)) ).is_some(),
//         None => true
//     }
// }

pub struct EXH {
    pub signature: String,
    pub version: u16,
    pub data_offset: u16,
    pub column_count: u16,
    pub page_count: u16,
    pub language_count: u16,
    pub variant: u16,
    pub row_count: u32,
    pub columns: Vec<EXHColumn>,
    pub rows: Vec<AssetEXHFileRow>,
    pub languages: Vec<EXHLang>,
}

impl Display for EXH {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.columns
                .iter()
                .map(|c| c.kind.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

pub struct EXHColumn {
    pub kind: EXHColumnKind,
    pub offset_to_offset: u16,
}

pub struct AssetEXHFileRow {
    pub start_id: u32,
    pub row_count: u32,
}

#[derive(PartialEq, Eq)]
pub enum EXHLang {
    None = 0,
    Japanese = 1,
    English = 2,
    German = 3,
    French = 4,
    ChineseSimplified = 5,
    ChineseTraditional = 6,
    Korean = 7,
}

pub enum EXHColumnKind {
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
    UNK2 = 0x0C,        // unused?
    PackedBool0 = 0x19, // 0 is read like data & 1, 1 is like data & 2, 2 = data & 4, etc...
    PackedBool1 = 0x1A,
    PackedBool2 = 0x1B,
    PackedBool3 = 0x1C,
    PackedBool4 = 0x1D,
    PackedBool5 = 0x1E,
    PackedBool6 = 0x1F,
    PackedBool7 = 0x20,
}

impl Display for EXHColumnKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                EXHColumnKind::String => String::from("STRING"),
                EXHColumnKind::Bool => String::from("BOOL"),
                EXHColumnKind::Int8 => String::from("INT8"),
                EXHColumnKind::UInt8 => String::from("UINT8"),
                EXHColumnKind::Int16 => String::from("INT16"),
                EXHColumnKind::UInt16 => String::from("UINT16"),
                EXHColumnKind::Int32 => String::from("INT32"),
                EXHColumnKind::UInt32 => String::from("UINT32"),
                EXHColumnKind::UNK1 => String::from("UNKNOWN"),
                EXHColumnKind::Float32 => String::from("FLOAT32"),
                EXHColumnKind::Int64 => String::from("INT64"),
                EXHColumnKind::UInt64 => String::from("UINT64"),
                EXHColumnKind::UNK2 => String::from("UNKNOWN"),
                EXHColumnKind::PackedBool0 => String::from("BOOL"),
                EXHColumnKind::PackedBool1 => String::from("BOOL"),
                EXHColumnKind::PackedBool2 => String::from("BOOL"),
                EXHColumnKind::PackedBool3 => String::from("BOOL"),
                EXHColumnKind::PackedBool4 => String::from("BOOL"),
                EXHColumnKind::PackedBool5 => String::from("BOOL"),
                EXHColumnKind::PackedBool6 => String::from("BOOL"),
                EXHColumnKind::PackedBool7 => String::from("BOOL"),
            }
        )
    }
}

impl EXHColumnKind {
    pub fn sizes(kind: &EXHColumnKind) -> u64 {
        match kind {
            EXHColumnKind::String => 1,
            EXHColumnKind::Bool => 1,
            EXHColumnKind::Int8 => 1,
            EXHColumnKind::UInt8 => 1,
            EXHColumnKind::Int16 => 2,
            EXHColumnKind::UInt16 => 2,
            EXHColumnKind::Int32 => 4,
            EXHColumnKind::UInt32 => 4,
            EXHColumnKind::UNK1 => 0,
            EXHColumnKind::Float32 => 4,
            EXHColumnKind::Int64 => 8,
            EXHColumnKind::UInt64 => 8,
            EXHColumnKind::UNK2 => 0,
            EXHColumnKind::PackedBool0 => 1,
            EXHColumnKind::PackedBool1 => 1,
            EXHColumnKind::PackedBool2 => 1,
            EXHColumnKind::PackedBool3 => 1,
            EXHColumnKind::PackedBool4 => 1,
            EXHColumnKind::PackedBool5 => 1,
            EXHColumnKind::PackedBool6 => 1,
            EXHColumnKind::PackedBool7 => 1,
        }
    }
}

impl EXH {
    pub fn new<R: BufferReader>(buffer: &mut Buffer<R>) -> EXH {
        buffer.offset_set(0);

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

        let columns: Vec<EXHColumn> = (0..column_count).map(|i| EXHColumn::new(buffer)).collect();
        let rows: Vec<AssetEXHFileRow> = (0..page_count)
            .map(|i| AssetEXHFileRow::new(buffer))
            .collect();
        let languages: Vec<EXHLang> = (0..language_count).map(|i| EXHLang::new(buffer)).collect();

        EXH {
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
            languages,
        }
    }

    pub fn from_vec(value: Vec<u8>) -> EXH {
        let mut buff = Buffer::from_vec(value);
        let output = EXH::new(&mut buff);
        output
    }

    pub fn find_lang(&self, lang: EXHLang) -> Option<&EXHLang> {
        self.languages.iter().find(|l| **l == lang)
    }
}

impl EXHLang {
    pub fn new<R: BufferReader>(buffer: &mut Buffer<R>) -> EXHLang {
        let lang = buffer.le_u16();
        match lang {
            0 => EXHLang::None,
            1 => EXHLang::Japanese,
            2 => EXHLang::English,
            3 => EXHLang::German,
            4 => EXHLang::French,
            5 => EXHLang::ChineseSimplified,
            6 => EXHLang::ChineseTraditional,
            7 => EXHLang::Korean,
            _ => panic!("Langauge not found"),
        }
    }
}

// impl PartialEq for EXHLang {
//     fn eq(&self, other: &Self) -> bool {
//         self as u32 == other as u32
//     }
// }

impl Display for EXHLang {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                EXHLang::None => String::from(""),
                EXHLang::Japanese => String::from("ja"),
                EXHLang::English => String::from("en"),
                EXHLang::German => String::from("de"),
                EXHLang::French => String::from("fr"),
                EXHLang::ChineseSimplified => String::from("zh"),
                EXHLang::ChineseTraditional => String::from("zh"),
                EXHLang::Korean => String::from("ko"),
            }
        )
    }
}

impl EXHColumn {
    pub fn new<R: BufferReader>(buffer: &mut Buffer<R>) -> EXHColumn {
        let kind = buffer.be_u16();
        EXHColumn {
            kind: match kind {
                0x00 => EXHColumnKind::String,
                0x01 => EXHColumnKind::Bool,
                0x02 => EXHColumnKind::Int8,
                0x03 => EXHColumnKind::UInt8,
                0x04 => EXHColumnKind::Int16,
                0x05 => EXHColumnKind::UInt16,
                0x06 => EXHColumnKind::Int32,
                0x07 => EXHColumnKind::UInt32,
                0x08 => EXHColumnKind::UNK1,
                0x09 => EXHColumnKind::Float32,
                0x0A => EXHColumnKind::Int64,
                0x0B => EXHColumnKind::UInt64,
                0x0C => EXHColumnKind::UNK2,
                0x19 => EXHColumnKind::PackedBool0,
                0x1A => EXHColumnKind::PackedBool1,
                0x1B => EXHColumnKind::PackedBool2,
                0x1C => EXHColumnKind::PackedBool3,
                0x1D => EXHColumnKind::PackedBool4,
                0x1E => EXHColumnKind::PackedBool5,
                0x1F => EXHColumnKind::PackedBool6,
                0x20 => EXHColumnKind::PackedBool7,
                _ => panic!("Asset column kind '{}'", kind),
            },
            offset_to_offset: buffer.be_u16(),
        }
    }
}

impl AssetEXHFileRow {
    pub fn new<R: BufferReader>(buffer: &mut Buffer<R>) -> AssetEXHFileRow {
        AssetEXHFileRow {
            start_id: buffer.be_u32(),
            row_count: buffer.be_u32(),
        }
    }
}
