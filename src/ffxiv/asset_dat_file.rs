use crate::ffxiv::buffer_file::BufferFile;

#[derive(Clone)]
pub struct AssetDatFileHeader {
    header_size: u32,
    header_version: u32,
    asset_size: u32,
    unknown1: u32,
    unknown2: u32,
    block_count: u32,
    blocks: Vec<AssetDatFileHeaderBlock>,
}

#[derive(Clone)]
pub struct AssetDatFileHeaderBlock {
    offset: u32,
    uncompressed_block_size: u16,
    compressed_block_size: u16,
}

#[derive(Clone)]
pub struct AssetDatFileDataBlock {
    header_size: u32,
    header_version: u32,
    block_type: BlockType,
    uncompressed_block_size: u32,
    data: Vec<u8>,
}

#[derive(Clone)]
pub enum BlockType {
    Compressed(u32),
    Uncompressed(u32),
}

pub struct AssetDatFile {
    header: AssetDatFileHeader,
    data: Vec<AssetDatFileDataBlock>
}

impl AssetDatFile {
    pub fn new(data_file: &mut BufferFile, data_file_offset: u64) {

    }
}

impl BlockType {
    pub fn new(n: u32) -> BlockType {
        match n {
            32000 => BlockType::Uncompressed(32000),
            _ => BlockType::Compressed(n)
        }
    }
}

impl AssetDatFileHeader {
    pub fn new(data_file: &mut BufferFile, data_file_offset: u64) -> AssetDatFileHeader {
        data_file.offset_set(data_file_offset as usize);
        let header_size = data_file.u32();
        let header_version = data_file.u32();
        let asset_size = data_file.u32();
        let unknown1 = data_file.u32();
        let unknown2 = data_file.u32();
        let block_count = data_file.u32();
        let blocks = (0..block_count).map(|i| AssetDatFileHeaderBlock::from_buffer(data_file)).collect();
        AssetDatFileHeader {
            header_size,
            header_version,
            asset_size,
            unknown1,
            unknown2,
            block_count,
            blocks,
        }
    }
}

impl AssetDatFileHeaderBlock {
    pub fn from_buffer(buffer: &mut BufferFile) -> AssetDatFileHeaderBlock {
        AssetDatFileHeaderBlock {
            offset: buffer.u32(),
            compressed_block_size: buffer.u16(),
            uncompressed_block_size: buffer.u16(),
        }
    }
}

impl AssetDatFileDataBlock {
    pub fn new(data_file: &mut BufferFile, data_file_offset: u64, asset_dat_file_header: &AssetDatFileHeader, block_metadata: &AssetDatFileHeaderBlock) -> AssetDatFileDataBlock {
        let block_offset = data_file_offset + (asset_dat_file_header.header_size + block_metadata.offset) as u64;
        data_file.offset_set(block_offset as usize);
        let header_size = data_file.u32();
        let header_version = data_file.u32();
        let block_type = BlockType::new(data_file.u32());
        let uncompressed_block_size = data_file.u32();
        let block_data_offset = (block_offset + header_size as u64) as usize;
        let data = match block_type {
            BlockType::Compressed(n) => data_file.vec_at(block_data_offset, n as usize),
            BlockType::Uncompressed(n) => data_file.vec_at(block_data_offset, uncompressed_block_size as usize)
        };

        AssetDatFileDataBlock {
            header_size,
            header_version,
            block_type,
            uncompressed_block_size,
            data,
        }
    }
    pub fn from_metadata(data_file: &mut BufferFile, asset_dat_file_header: &AssetDatFileHeader, data_file_offset: u64) -> Vec<AssetDatFileDataBlock> {
        data_file.offset_set((data_file_offset + asset_dat_file_header.header_size as u64) as usize);
        asset_dat_file_header.blocks.iter().map(|block_metadata| AssetDatFileDataBlock::new(data_file, data_file_offset, asset_dat_file_header, block_metadata)).collect()
    }
}