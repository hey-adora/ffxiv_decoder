use std::fs::File;
use crate::reader::Buffer;

struct DatScd {
    metadata_size: u32,
    header_size: u32,
    header_version: u32,
    header_compressed_data_size: u32,
    header_uncompressed_data_size: u32,
    data_compressed: Vec<u8>,
    data_uncompressed: Vec<u8>,
}

impl DatScd {

    pub fn from_file(data_file_offset: usize) {
        // let file = File::open()
        // let buffer: Buffer = Buffer::new();
        //
        // let metadata_size: u32 = buffer.u32(0x00);
        //
        // let offset = metadata_size as usize;

        // let header_size: u32 = buffer.u32(offset);
        // let header_version: u32 = buffer.u32(offset + 0x04);
        // let header_compressed_data_size: u32 = buffer.u32(offset + 0x08);
        // let header_uncompressed_data_size: u32 = buffer.u32(offset + 0x0C);
        //

    }

}