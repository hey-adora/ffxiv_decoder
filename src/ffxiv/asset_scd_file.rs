use crate::ffxiv::buffer_vec::BufferVec;

#[derive(Clone, Debug)]
pub struct AssetSCDFile {
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

impl AssetSCDFile {
    pub fn new(buffer: &mut BufferVec) -> AssetSCDFile {
        buffer.offset = 0;

        let signature = buffer.string_at(0x00, 0x08);
        let big_endian = buffer.u8_at(0x0c);
        let sscf_version = buffer.u8_at(0x0d);
        let is_big_endian = big_endian == 1;

        let version = buffer.be_le_i16_at(0x08, is_big_endian);
        let offset_to_table = buffer.be_le_i16_at(0x0e, is_big_endian);
        let file_size = buffer.be_le_u16_at(0x10, is_big_endian);

        let table_size = buffer.be_le_u16_at(offset_to_table as usize, is_big_endian);
        let table_size_of_sound_entry_offset = buffer.be_le_i16_at((offset_to_table + 0x02) as usize, is_big_endian);
        let table_header_entries = buffer.be_le_i16_at((offset_to_table + 0x04) as usize, is_big_endian);
        let table_offset = buffer.be_le_u32_at((offset_to_table + 0x08) as usize, is_big_endian);
        let table_entry_to_offset = buffer.be_le_u32_at((offset_to_table + 0x0c) as usize, is_big_endian);
        let table_offset_to_table_2 = buffer.be_le_u32_at((offset_to_table + 0x0c) as usize, is_big_endian);

        let entry_offset = buffer.be_le_u32_at((table_entry_to_offset) as usize, is_big_endian);

        let entry_stream_size = buffer.be_le_i32_at((entry_offset) as usize, is_big_endian);
        let entry_channels = buffer.be_le_i32_at((entry_offset + 0x4) as usize, is_big_endian);
        let entry_sample_rate = buffer.be_le_i32_at((entry_offset + 0x8) as usize, is_big_endian);
        let entry_codex = buffer.be_le_i32_at((entry_offset + 0xc) as usize, is_big_endian);
        let entry_loop_start = buffer.be_le_i32_at((entry_offset + 0x10) as usize, is_big_endian);
        let entry_loop_end = buffer.be_le_i32_at((entry_offset + 0x14) as usize, is_big_endian);
        let entry_extra_data_size = buffer.be_le_i32_at((entry_offset + 0x18) as usize, is_big_endian);
        let entry_aux_chunk_count = buffer.be_le_i32_at((entry_offset + 0x1c) as usize, is_big_endian);
        let entry_extra_data_offset = buffer.be_le_i32_at((entry_offset + 0x20) as usize, is_big_endian);
        let entry_frame_size = buffer.be_le_i16_at((entry_offset + 0x2c) as usize, is_big_endian);
        let entry_wave_format_ex = buffer.be_le_u16_at((entry_offset + 0x34) as usize, is_big_endian);

        let audio_offset = entry_offset + (entry_extra_data_size as u32) + 0x20;

        AssetSCDFile {
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