use ffxiv_decoder::ffxiv::asset::exh::{EXHLang, EXH};
use ffxiv_decoder::ffxiv::path::DatPath;
use ffxiv_decoder::ffxiv::FFXIV;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use std::{env, fs};

static HASH_NAMES: &str = "./resources/hash_names.txt";
static EXPORT_DIR: &str = "./exports/";

fn main() {
    let game_path = read_input();

    let ffxiv = FFXIV::new(&game_path).unwrap();

    let data_path = DatPath::new("exd/action.exh").unwrap();
    let exh = ffxiv.get_exh(data_path).unwrap();

    let exh_lang = exh
        .data
        .find_lang(EXHLang::English)
        .unwrap_or(&EXHLang::None);

    let pages = exh.get_pages(exh_lang).unwrap();
    for (path, csv) in pages {
        write_file(&format!("{}.csv", &path.path_name), csv);
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

fn write_file(file_name: &str, content: String) {
    let export_path = &format!("{}{}", EXPORT_DIR, file_name);
    let export_path_buf = PathBuf::from(export_path);
    let dir_path = export_path_buf.parent().unwrap();

    if !dir_path.exists() {
        create_dir_all(dir_path).unwrap();
    }

    fs::write(export_path, content).unwrap();
}
