use std::{env, fs};
use std::fs::{create_dir_all};
use std::path::{Path, PathBuf};
use ffxiv_decoder::ffxiv::FFXIV;
use ffxiv_decoder::ffxiv::path::DatPath;

static HASH_NAMES: &str = "./resources/hash_names.txt";
static EXPORT_DIR: &str = "./exports/";

fn main() {
    let game_path = read_input();

    let ffxiv = FFXIV::new(&game_path).unwrap();

    let data_path = DatPath::new("exd/custom/000/regseaarmguild_00056.exh").unwrap();
    let exh = ffxiv.get_asset(&data_path).unwrap().decompress().unwrap();
    write_file(&data_path.path_name, exh);

    let data_path = DatPath::new("exd/custom/000/regseaarmguild_00056_0_en.exd").unwrap();
    let exd = ffxiv.get_asset(&data_path).unwrap().decompress().unwrap();
    write_file(&data_path.path_name, exd);

    let data_path = DatPath::new("music/ex1/bgm_ex1_endcredit01.scd").unwrap();
    let scd = ffxiv.get_asset(&data_path).unwrap().decompress().unwrap();
    write_file(&data_path.path_name, scd);
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

fn write_file(file_name: &str, content: Vec<u8>) {
    let export_path = &format!("{}{}", EXPORT_DIR, file_name);
    let export_path_buf = PathBuf::from(export_path);
    let dir_path = export_path_buf.parent().unwrap();

    if !dir_path.exists() {
        create_dir_all(dir_path).unwrap();
    }

    fs::write(export_path, content).unwrap();
}