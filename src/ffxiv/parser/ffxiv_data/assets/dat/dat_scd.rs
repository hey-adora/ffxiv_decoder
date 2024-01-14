// use std::fs::File;
// use crate::reader::Buffer;
//
// struct DatScd {
//     metadata_size: u32,
//     header_size: u32,
//     header_version: u32,
//     header_compressed_data_size: u32,
//     header_uncompressed_data_size: u32,
//     data_compressed: Vec<u8>,
//     data_uncompressed: Vec<u8>,
// }
//
// impl DatScd {
//
//     pub fn from_file(data_file_offset: usize) {
//         // let file = File::open()
//         // let buffer: Buffer = Buffer::new();
//         //
//         // let metadata_size: u32 = buffer.u32(0x00);
//         //
//         // let offset = metadata_size as usize;
//
//         // let header_size: u32 = buffer.u32(offset);
//         // let header_version: u32 = buffer.u32(offset + 0x04);
//         // let header_compressed_data_size: u32 = buffer.u32(offset + 0x08);
//         // let header_uncompressed_data_size: u32 = buffer.u32(offset + 0x0C);
//         //
//
//     }
//
// }


use crate::ffxiv::reader::buffer::Buffer;

#[derive(Clone, Debug)]
struct HexValue<T> {
    value: T,
    hex_list: Vec<u8>,
    start: usize,
    end: usize,
    size: usize,
}

#[derive(Clone, Debug)]
pub struct SCD {
    pub signature: String,
    pub version: i16,
    pub big_endian: u8,
    pub sscf_version: u8,
    pub offset_to_table: i16,
    pub file_size: u16,
    pub table_size: u16,
    pub table_size_of_sound_entry_offset: i16,
    pub table_header_entries: i16,
    pub table_offset: u32,
    pub table_entry_to_offset: u32,
    pub table_offset_to_table_2: u32,
    pub entry_offset: u32,
    pub entry_stream_size: i32,
    pub entry_channels: i32,
    pub entry_sample_rate: i32,
    pub entry_codex: i32,
    pub entry_loop_start: i32,
    pub entry_loop_end: i32,
    pub entry_extra_data_size: i32,
    pub entry_aux_chunk_count: i32,
    pub entry_extra_data_offset: i32,
    pub entry_frame_size: i16,
    pub entry_wave_format_ex: u16,
    pub audio_offset: u32,
}

impl SCD {
    pub fn new(buffer: &mut Buffer) -> SCD {
        let signature = buffer.string(0x00, 0x08);
        let version = buffer.i16(0x08);
        let big_endian = buffer.u8(0x0c);
        let sscf_version = buffer.u8(0x0d);
        let offset_to_table = buffer.i16(0x0e);
        let file_size = buffer.u16(0x10);

        let table_size = buffer.u16(offset_to_table as usize);
        let table_size_of_sound_entry_offset = buffer.i16((offset_to_table + 0x02) as usize);
        let table_header_entries = buffer.i16((offset_to_table + 0x04) as usize);
        let table_offset = buffer.u32((offset_to_table + 0x08) as usize);
        let table_entry_to_offset = buffer.u32((offset_to_table + 0x0c) as usize);
        let table_offset_to_table_2 = buffer.u32((offset_to_table + 0x0c) as usize);

        let entry_offset = buffer.u32((table_entry_to_offset) as usize);

        let entry_stream_size = buffer.i32((entry_offset) as usize);
        let entry_channels = buffer.i32((entry_offset + 0x4) as usize);
        let entry_sample_rate = buffer.i32((entry_offset + 0x8) as usize);
        let entry_codex = buffer.i32((entry_offset + 0xc) as usize);
        let entry_loop_start = buffer.i32((entry_offset + 0x10) as usize);
        let entry_loop_end = buffer.i32((entry_offset + 0x14) as usize);
        let entry_extra_data_size = buffer.i32((entry_offset + 0x18) as usize);
        let entry_aux_chunk_count = buffer.i32((entry_offset + 0x1c) as usize);
        let entry_extra_data_offset = buffer.i32((entry_offset + 0x20) as usize);
        let entry_frame_size = buffer.i16((entry_offset + 0x2c) as usize);
        let entry_wave_format_ex = buffer.u16((entry_offset + 0x34) as usize);

        let audio_offset = entry_offset + (entry_extra_data_size as u32) + 0x20;

        SCD {
            signature,
            version,
            big_endian,
            sscf_version,
            offset_to_table,
            file_size,

            table_size,
            table_size_of_sound_entry_offset,
            table_header_entries,
            table_offset,
            table_entry_to_offset,
            table_offset_to_table_2,

            entry_offset,

            entry_stream_size,
            entry_channels,
            entry_sample_rate,
            entry_codex,
            entry_loop_start,
            entry_loop_end,
            entry_extra_data_size,
            entry_aux_chunk_count,
            entry_extra_data_offset,
            entry_frame_size,
            entry_wave_format_ex,

            audio_offset,
        }
    }
}