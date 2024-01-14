use std::arch::x86_64::_mm_crc32_u32;
use game_data_resolver::visualizer;
use game_data_resolver::reader;
use game_data_resolver::parser;
use game_data_resolver::decoder;
use std::f32::consts::PI;
use std::i16;
use std::path::Path;
use hound;
use crc32fast::Hasher;
use crc::*;

static FILE_PATH: &str = "./2.scd";


fn main() {
    // let mut buffer = reader::Buffer::from_file(FILE_PATH);
    // let metadata = parser::sqex_scd::Metadata::new(&mut buffer);
    // if metadata.entry_channels > 2 || metadata.entry_codex != 12 || metadata.entry_wave_format_ex != 7 {
    //     panic!("Unsupported format");
    // }
    // let decoded = decoder::sqex_scd::decode(&metadata, &mut buffer);
    //
    // let spec = hound::WavSpec {
    //     channels: metadata.entry_channels as u16,
    //     sample_rate: metadata.entry_sample_rate as u32,
    //     bits_per_sample: 16,
    //     sample_format: hound::SampleFormat::Int,
    // };
    // let mut writer = hound::WavWriter::create("sine.wav", spec).unwrap();
    // for index in (0..decoded.len()).step_by(2) {
    //     writer.write_sample(i16::from_le_bytes([decoded[index], decoded[index + 1]])).unwrap();
    // }
    parse_path("exd/root.exl");
}


fn parse_path(path: &str) {
    //path.split('/')
    let path = Path::new(path);
    let full_path = path.as_os_str().to_str().unwrap();
    let full_file_name = path.file_name().unwrap().to_str().unwrap();
    let file_name = path.file_stem().unwrap().to_str().unwrap();
    let file_extension = path.extension().unwrap().to_str().unwrap();
    let components: Vec<&str> = path.components().map(|c| c.as_os_str().to_str().unwrap()).collect();
    let category = components[0];

    // let mut hasher = Hasher::new();
    // hasher.update(full_path.as_bytes());
    // hasher.update(full_file_name.as_bytes());
    // let checksum = hasher.finalize();

    let folder_hash: u64 = crc32fast::hash(full_path.as_bytes()) as u64;
    println!("{} {}", full_path, folder_hash);

    let category_hash: u64 = crc32fast::hash(category.as_bytes()) as u64;
    println!("{} {}", category, category_hash);

    let filename_hash: u64 = crc32fast::hash(full_file_name.as_bytes()) as u64;
    println!("{} {}", full_file_name, filename_hash);

    let hash = (folder_hash << 32) | filename_hash;
    println!("{}", hash);

    let CrcInitialSeed: usize = 0;
    let poly: usize = 3988292384;
    let mut crcTable = [0; 16 * 256];

    for i in 0..256 {
        let mut res = i;
        for t in 0..16 {
            for k in 0..8 {
                res = if (res & 1) == 1 { poly ^ (res >> 1) } else { res >> 1 };
            }
            crcTable[(t * 256) + i] = res;
        }
    }

    let crc: usize = CrcInitialSeed;
    let table = crcTable;
    let mut crc_local: usize = (u32::MAX as usize) ^ crc;
    let path_ = String::from(full_path);
    let buffer: Vec<usize> = path_.as_bytes().iter().map(|b| *b as usize).collect();
    let mut size = buffer.len();
    let mut start = 0;

    while size >= 16 {
        let a = table[(3 * 256) + buffer[start * 12]] ^
            table[(2 * 256) + buffer[start * 13]] ^
            table[(1 * 256) + buffer[start * 14]] ^
            table[(0 * 256) + buffer[start * 15]];

        let b = table[(7 * 256) + buffer[start * 8]] ^
            table[(6 * 256) + buffer[start * 9]] ^
            table[(5 * 256) + buffer[start * 10]] ^
            table[(4 * 256) + buffer[start * 11]];

        let c = table[(11 * 256) + buffer[start * 4]] ^
            table[(10 * 256) + buffer[start * 5]] ^
            table[(9 * 256) + buffer[start * 6]] ^
            table[(8 * 256) + buffer[start * 7]];

        let d = table[(15 * 256) + (u64_to_u8_as_usize(crc_local ^ buffer[start]))] ^
            table[(14 * 256) + (u64_to_u8_as_usize((crc_local >> 8) ^ buffer[start + 1]))] ^
            table[(13 * 256) + (u64_to_u8_as_usize((crc_local >> 16) ^ buffer[start + 2]))] ^
            table[(12 * 256) + ((crc_local >> 24) ^ buffer[start + 3])];

        crc_local = d ^ c ^ b ^ a;

        start += 16;
        size -= 16;
    }

    size -= 1;
    while size > 1 {
        let index = u64_to_u8_as_usize(crc_local ^ buffer[start]);
        let table_item = table[index];
        crc_local = table_item ^ (crc_local >> 8);
        size -= 1;
        start += 1;
    }


    let crc32 = crc_local ^ (u32::MAX as usize);

    println!("{}", crc32);

    // exd/root.exl

    const CUSTOM_ALG: Algorithm<u32> = Algorithm {
        width: 32,
        poly: 3988292384,
        init: 0xffff,
        refin: false,
        refout: false,
        xorout: 0x0000,
        check: 0xaee7,
        residue: 0x0000,
    };

    let algos = [&CRC_17_CAN_FD, &CRC_21_CAN_FD, &CRC_24_BLE, &CRC_24_FLEXRAY_A, &CRC_24_FLEXRAY_B, &CRC_24_INTERLAKEN, &CRC_24_LTE_A, &CRC_24_LTE_B, &CRC_24_OPENPGP, &CRC_24_OS_9, &CRC_30_CDMA, &CRC_31_PHILIPS, &CRC_32_AIXM, &CRC_32_AUTOSAR, &CRC_32_BASE91_D, &CRC_32_BZIP2, &CRC_32_CD_ROM_EDC, &CRC_32_CKSUM, &CRC_32_ISCSI, &CRC_32_ISO_HDLC, &CRC_32_JAMCRC, &CRC_32_MEF, &CRC_32_MPEG_2, &CRC_32_XFER];
    for algo in algos {
        testt(algo);
    }

    testt(&CRC_32_JAMCRC);

    // for com in category {
    //     println!("{}", com);
    // }
    // for component in path.components() {
    //     println!("{}", component.as_os_str().to_str().unwrap());
    // }
}

fn u64_to_u8_as_usize(n: usize) -> usize {
    // let usize_size = std::mem::size_of::<usize>() * 8;
    // let bytes = usize_size - 8;
    let output = n >> 24;
    println!("{}", output);
    output
}

fn testt(algo: &'static Algorithm<u32>){
    let crc = Crc::<u32>::new(algo);
    let mut digest = crc.digest();
    digest.update(b"exd/root.exl");
    println!("{}", digest.finalize());
}