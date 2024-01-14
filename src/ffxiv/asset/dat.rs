use std::fmt::{Display, Formatter};
use flate2::{Decompress, FlushDecompress};
use thiserror::Error;
use crate::ffxiv::buffer::{Buffer, BufferReader};

#[derive(Clone)]
pub struct StandardFile {
    pub header_size: u32,
    pub data_type: DatHeaderType,
    pub asset_size: u32,
    pub unknown1: u32,
    pub unknown2: u32,
    pub block_count: u32,
    pub blocks: Vec<StandardHeaderBlock>,
    pub data: Vec<DatBlock>
}

impl StandardFile {
    pub fn new_at<R: BufferReader>(data_file: &mut Buffer<R>, data_file_offset: u64) -> StandardFile {
        data_file.offset_set(data_file_offset);
        let header_size = data_file.le_u32();
        let data_type = data_file.le_u32();
        let asset_size = data_file.le_u32();
        let unknown1 = data_file.le_u32();
        let unknown2 = data_file.le_u32();
        let block_count = data_file.le_u32();
        let blocks: Vec<StandardHeaderBlock> = (0..block_count).map(|i| StandardHeaderBlock::from_buffer(data_file)).collect();
        let data: Vec<DatBlock> = blocks.iter().map(|b| DatBlock::from_buffer_at(data_file, data_file_offset + header_size as u64 + b.offset as u64)).collect();
        StandardFile {
            data_type: DatHeaderType::new(data_type).unwrap(),
            header_size,
            asset_size,
            unknown1,
            unknown2,
            block_count,
            blocks,
            data
        }
    }

    pub fn decompress(&self) -> Result<Vec<u8>, DecompressError> {
        let mut data: Vec<u8> = Vec::with_capacity(self.asset_size as usize);
        for (i, b) in self.data.iter().enumerate() {
            let mut decompressed = DatBlock::decompress(b).or_else(|msg|Err(DecompressError::BlockDecompressionError(format!("{} | BlockIndex: {} | HeaderSize: {}, HeaderVersion: {}, BlockType: {}, UncompressedBlockSize: {} ", msg, i, b.header_size, b.header_version,  b.block_type, b.uncompressed_block_size))))?;
            data.append(&mut decompressed);
        }
        Ok(data)
    }
}

#[derive(Clone)]
pub struct StandardHeaderBlock {
    pub offset: u32,
    pub uncompressed_block_size: u16,
    pub compressed_block_size: u16,
}

impl StandardHeaderBlock {
    pub fn from_buffer<R: BufferReader>(buffer: &mut Buffer<R>) -> StandardHeaderBlock {
        StandardHeaderBlock {
            offset: buffer.le_u32(),
            compressed_block_size: buffer.le_u16(),
            uncompressed_block_size: buffer.le_u16(),
        }
    }
}

//==================================================================================================

#[derive(Clone)]
pub struct TextureFile {
    pub header_size: u32,
    pub data_type: DatHeaderType,
    pub asset_size: u32,
    pub unknown1: u32,
    pub unknown2: u32,
    pub block_count: u32,
    pub blocks: Vec<TextureHeaderBlock>,
    pub block_offsets: Vec<u16>,
    pub mipmap_data: Option<Vec<u8>>,
    pub data: Vec<Vec<DatBlock>>
}

impl TextureFile {
    pub fn new_at<R: BufferReader>(data_file: &mut Buffer<R>, data_file_offset: u64) -> TextureFile {
        data_file.offset_set(data_file_offset);
        let header_size = data_file.le_u32();
        let data_type = data_file.le_u32();
        let asset_size = data_file.le_u32();
        let unknown1 = data_file.le_u32();
        let unknown2 = data_file.le_u32();
        let block_count = data_file.le_u32();
        let blocks: Vec<TextureHeaderBlock> = (0..block_count).map(|i| TextureHeaderBlock::from_buffer(data_file)).collect();

        let last_block = blocks.last();
        let block_offset_count = if let Some(last_block) = last_block { last_block.block_count + last_block.block_offset } else { 0 };
        let block_offsets: Vec<u16> = (0..block_offset_count).map(|i| data_file.le_u16()).collect();

        let first_block = blocks.first();
        let mipmap_data: Option<Vec<u8>> = if let Some(first_block) = first_block {
            if first_block.compressed_offset > 0 {
                Some(data_file.vec_at(data_file_offset + header_size as u64, first_block.compressed_offset as usize))
            } else {
                None
            }
        } else { None };

        let data: Vec<Vec<DatBlock>> = blocks.iter().map(|b|{
            let mut blocks: Vec<DatBlock> = Vec::new();
            let mut block_offset: u64 = 0;
            for i in b.block_offset..b.block_offset + b.block_count {
                let offset = data_file_offset + header_size as u64 + b.compressed_offset as u64 + block_offset;

                blocks.push(DatBlock::from_buffer_at(data_file, offset));

                block_offset += block_offsets[i as usize] as u64;
            }
            blocks
        }).collect();


        TextureFile {
            data_type: DatHeaderType::new(data_type).unwrap(),
            header_size,
            asset_size,
            unknown1,
            unknown2,
            block_count,
            blocks,
            block_offsets,
            mipmap_data,
            data
        }
    }

    pub fn decompress(&self) -> Result<Vec<u8>, DecompressError> {
        let mut data: Vec<u8> = if let Some(mipmap) = &self.mipmap_data {
            mipmap.clone()
        } else {
            Vec::new()
        };


        for (i, datum) in self.data.iter().enumerate() {
            for (block_i, block) in datum.iter().enumerate() {
                data.extend(DatBlock::decompress(block).or_else(|msg|Err(DecompressError::BlockDecompressionError(format!("{} | BlockChunkIndex: {} BlockIndex: {} | HeaderSize: {}, HeaderVersion: {}, BlockType: {}, UncompressedBlockSize: {} ", msg, i, block_i, block.header_size, block.header_version,  block.block_type, block.uncompressed_block_size))))?)
            }
        }

        Ok(data)
    }
}

#[derive(Clone)]
pub struct TextureHeaderBlock {
    pub compressed_offset: u32,
    pub compressed_size: u32,
    pub decompressed_size: u32,
    pub block_offset: u32,
    pub block_count: u32,
}

impl TextureHeaderBlock {
    pub fn from_buffer<R: BufferReader>(buffer: &mut Buffer<R>) -> TextureHeaderBlock {
        TextureHeaderBlock {
            compressed_offset: buffer.le_u32(),
            compressed_size: buffer.le_u32(),
            decompressed_size: buffer.le_u32(),
            block_offset: buffer.le_u32(),
            block_count: buffer.le_u32(),
        }
    }
}



//==================================================================================================

#[derive(Clone)]
pub struct DatBlock {
    pub header_size: u32,
    pub header_version: u32,
    pub block_type: DatBlockType,
    pub uncompressed_block_size: u32,
    pub data: Vec<u8>,
}

impl DatBlock {
    pub fn from_buffer<R: BufferReader>(data_file: &mut Buffer<R>) -> DatBlock {
        let header_size = data_file.le_u32();
        let header_version = data_file.le_u32();
        let block_type = DatBlockType::new(data_file.le_u32());
        let uncompressed_block_size = data_file.le_u32();
        let data = match block_type {
            DatBlockType::Compressed(n) => data_file.vec(n as usize),
            DatBlockType::Uncompressed(n) => data_file.vec(uncompressed_block_size as usize)
        };

        DatBlock {
            header_size,
            header_version,
            block_type,
            uncompressed_block_size,
            data,
        }
    }

    pub fn from_buffer_at<R: BufferReader>(data_file: &mut Buffer<R>, block_offset: u64) -> DatBlock {
        data_file.offset_set(block_offset);
        DatBlock::from_buffer(data_file)
    }

    pub fn decompress(block: &DatBlock) -> Result<Vec<u8>, String>{
        match block.block_type {
            DatBlockType::Compressed(n) => {
                let mut decompressed_block_data: Vec<u8> = Vec::with_capacity(block.uncompressed_block_size as usize);
                let mut decompressor = Decompress::new(false);
                decompressor.decompress_vec(&block.data, &mut decompressed_block_data, FlushDecompress::None).or_else(|e|Err(e.to_string()))?;
                Ok(decompressed_block_data)
            }
            DatBlockType::Uncompressed(n) => Ok(block.data.clone())
        }
    }
}

//==================================================================================================

#[derive(Clone)]
pub enum DatHeaderType {
    Empty = 1,
    Standard = 2,
    Model = 3,
    Texture = 4,
}

impl DatHeaderType {
    pub fn new(n: u32) -> Result<DatHeaderType, DatHeaderTypeError> {
        match n {
            1 => Ok(DatHeaderType::Empty),
            2 => Ok(DatHeaderType::Standard),
            3 => Ok(DatHeaderType::Model),
            4 => Ok(DatHeaderType::Texture),
            _ => Err(DatHeaderTypeError::Invalid(n))
        }
    }

    pub fn check_at<R: BufferReader>(buffer: &mut Buffer<R>, block_offset: u64) -> Result<DatHeaderType, DatHeaderTypeError> {
        DatHeaderType::new(buffer.le_u32_at(block_offset + 0x04))
    }
}

//==================================================================================================

#[derive(Clone)]
pub enum DatBlockType {
    Compressed(u32),
    Uncompressed(u32),
}

impl Display for DatBlockType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            DatBlockType::Compressed(i) => format!("Compressed: {}", i),
            DatBlockType::Uncompressed(i) => format!("Uncompressed: {}", i)
        })
    }
}

impl DatBlockType {
    pub fn new(n: u32) -> DatBlockType {
        match n {
            32000 => DatBlockType::Uncompressed(32000),
            _ => DatBlockType::Compressed(n)
        }
    }
}

//==================================================================================================

#[derive(Error, Debug)]
pub enum DecompressError {
    #[error("Failed to decompress block: {0}")]
    BlockDecompressionError(String),
}

#[derive(Error, Debug)]
pub enum DatHeaderTypeError {
    #[error("Header type not found: '{0}', must be 1, 2, 3 or 4")]
    Invalid(u32)
}