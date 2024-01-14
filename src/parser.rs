pub mod decoder;
pub mod reader;
//pub mod visualizer;

//pub use crate::parser::visualizer;

#[derive(Clone)]
struct HexValue<T> {
    value: T,
    hex_list: Vec<u8>,
    start: usize,
    end: usize,
    size: usize,
}

#[derive(Clone)]
pub struct SqexScd {
    init_signature: String,
    init_version: i16,
    init_big_endian: u8,
    init_sscf_version: u8,
    init_table_offset: i16,
    init_file_size: u16,
    table_size: u16,
    table_size_of_sound_entry_offset: i16,
    table_header_entries: i16,
    table_offset: u32,
    table_entry_to_offset: u32,
    table_offset_to_table_2: u32,
    entry_offset: u32,
    entry_stream_size: i32,
    entry_channels: i32,
    entry_sample_rate: i32,
    entry_codex: i32,
    entry_loop_start: i32,
    entry_loop_end: i32,
    entry_extra_data_size: i32,
    entry_aux_chunk_count: i32,
    entry_extra_data_offset: i32,
    entry_frame_size: i16,
    entry_wave_format_ex: u16,
    audio_offset: u32
}

impl SqexScd {
    pub fn new(buffer: &mut reader::Buffer) -> SqexScd {
        let init_signature = buffer.string(0x00, 0x08);
        let init_version = buffer.i16(0x08);
        let init_big_endian = buffer.u8(0x0c);
        let init_sscf_version = buffer.u8(0x0d);
        let init_table_offset = buffer.i16(0x0e);
        let init_file_size = buffer.u16(0x0e);

        let table_size = buffer.u16(init_table_offset as usize);
        let table_size_of_sound_entry_offset = buffer.i16((init_table_offset + 0x02) as usize);
        let table_header_entries = buffer.i16((init_table_offset + 0x04) as usize);
        let table_offset = buffer.u32((init_table_offset + 0x08) as usize);
        let table_entry_to_offset = buffer.u32((init_table_offset + 0x0c) as usize);
        let table_offset_to_table_2 = buffer.u32((init_table_offset + 0x0c) as usize);

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

        SqexScd {
            init_signature,
            init_version,
            init_big_endian,
            init_sscf_version,
            init_table_offset,
            init_file_size,

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

            audio_offset
        }
    }

}