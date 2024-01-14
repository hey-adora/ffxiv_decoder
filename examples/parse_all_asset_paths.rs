use std::{env, fs};
use std::collections::HashMap;
use std::fs::{create_dir_all, File};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use ffxiv_decoder::ffxiv::FFXIV;
use ffxiv_decoder::ffxiv::path::DatPath;

static HASH_NAMES: &str = "./resources/hash_names.txt";
static EXPORT_FILE: &str = "./exports/parsed_asset_paths.csv";

fn main() {
    let game_path = read_input();
    let mut writer = create_output_writer(EXPORT_FILE);

    let ffxiv = FFXIV::new(&game_path).unwrap();

    let hash_names = FFXIV::get_decoded_hash_names(HASH_NAMES).expect("Failed to parse asset paths");
    let asset_hashes = ffxiv.get_all_dat_index1item_hashmap();

    write!(writer, "Hash,HashName,DataFile,DataOffset\n").expect(&format!("Failed to write to {}", EXPORT_FILE));

    for (dat, items) in asset_hashes {
        for item in items {
            let hash_name = hash_names.get(&item.hash).and_then(|hash| Some(hash.path_str.clone())).unwrap_or(String::from("UNKNOWN"));
            let output_line = format!("\"{}\",\"{}\",\"{}\",\"{}\"", item.hash, hash_name, dat, item.data_file_offset);
            write!(writer, "{}\n", output_line).expect(&format!("Failed to write to {}", EXPORT_FILE));
        }
    }
}

fn read_input() -> String {
    let args: Vec<String> = env::args().collect();
    let err_msg = "Must specify game path as argument, example:\ncargo run --package ffxiv_data_resolver --example parse_all_asset_paths -- \"/path/to/game\"";

    if args.len() < 2 {
        panic!("{}", err_msg)
    }

    let game_path = &args.get(1).expect(err_msg);
    let verify = Path::new(game_path);
    if !verify.exists() {
        panic!("Game path \"{}\" doesn't exist.", game_path)
    }

    if !verify.is_dir() {
        panic!("Game path \"{}\" is not a directory.", game_path)
    }


    (*game_path).to_owned()
}

fn create_output_writer(export_path: &str) -> BufWriter<File> {
    let export_path_buf = PathBuf::from(export_path);

    if export_path_buf.exists() && export_path_buf.is_file() {
        fs::remove_file(export_path).unwrap();
    } else {
        let dir = export_path_buf.parent().unwrap();
        create_dir_all(dir).unwrap();
    }

    let file = File::create(export_path_buf).expect(&format!("Failed to create \"{}\" file.", export_path));

    let writer = BufWriter::new(file);

    writer
}