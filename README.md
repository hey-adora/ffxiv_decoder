 
## .scd 8bit binary file structure
```
0x00 | i64 | signature = SEDBSSCF
0x08 | i16 | version = 3 //v2: demo PS3, v3: common; v4: Kingdom Hearts (PS4)
0x0C | u8 | big_endian = 0 // size_offset: 0 = 0x10; 1 = 0x14
0x0D | u8 | sscf_version = 4 // V4/3/2
0x0E | i16 | tables_offset =0x30 // usually 0x30 or 0x20
0x34 | i16 | headers_entries =0x1 // (tables_offset + 0x04 = 0x34)
0x3C | i32 | headers_offset =0x70 // (tables_offset + 0x0C = 0x3C)
0x70 | i32 | entry_offset =0x230//i<headers_entries; headers_offset + i * 4 =0x70
0x230 | i32 | stream_size = 37940 // entry_offset + 0x00 = 0x230
0x234 | i32 | channels = 1 // entry_offset + 0x04 = 0x234
0x238 | i32 | sample_rate = 44063 // entry_offset + 0x08 = 0x238
0x23C | i32 | codex = 12 // entry_offset + 0x0C = 0x23C;-1=dummy;12=MSADPCM
0x240 | i32 | loop_start = 0 // entry_offset + 0x10 = 0x240
0x244 | i32 | loop_end = 0 // entry_offset + 0x14 = 0x244
0x248 | i32 | extradata_sieze = 50 // entry_offset + 0x18 = 0x248
0x24C | i32 | aux_chunk_count = 16777216 // entry_offset + 0x1C = 0x24C
0x250 | i32 | extradata_offset = 65538 //entry_offset+0x20;if 1296126539 = skip
0x25C | i16 | frame_size = 70 // extradata_offset + 0x0C = 0x25C
0x264 | u16 | waveformatex = 7 // extradata_offset + 0x14 = 0x264
0x266 | i16 | coef0 = 256 // extradata_offset + 0x16 = 0x266
0x268 | i16 | coef1 = 0 // extradata_offset + 0x18 = 0x268
0x26A | i16 | coef2 = 512 // extradata_offset + 0x1A = 0x26A
0x26C | i16 | coef3 = -256 // extradata_offset + 0x1C = 0x26C
0x26E | i16 | coef4 = 0 // extradata_offset + 0x1E = 0x26E
0x270 | i16 | coef5 = 0 // extradata_offset + 0x20 = 0x270
0x272 | i16 | coef6 = 192 // extradata_offset + 0x22 = 0x272
0x274 | i16 | coef7 = 64 // extradata_offset + 0x24 = 0x274
0x276 | i16 | coef8 = 240 // extradata_offset + 0x26 = 0x276
0x278 | i16 | coef9 = 0 // extradata_offset + 0x28 = 0x278
0x27A | i16 | coef10 = 460 // extradata_offset + 0x2A = 0x27A
0x27C | i16 | coef11 = -208 // extradata_offset + 0x2C = 0x27C
0x27E | i16 | coef12 = 392 // extradata_offset + 0x2E = 0x27E
0x280 | i16 | coef13 = -232 // extradata_offset + 0x30 = 0x280


sample_num = 69376 //msadpcm_bytes_to_samples
total = 1.5744729137825386 //sample_num / sample_rate
time_total_sec = 1.5744729137825386 // total - 60 * ((int)total / 60)
samples_per_frame = 128 // (frame_size - 0x07 * channels)*2 / channels + 2;


```

```c
long msadpcm_bytes_to_samples(long stream_size, int frame_size, int channels) {
    if (frame_size <= 0 || channels <= 0) return 0;
    return (stream_size / frame_size) * (frame_size - (7-1)*channels) * 2 / channels + ((stream_size % frame_size) ? ((stream_size % frame_size) - (7-1)*channels) * 2 / channels : 0);
}
```