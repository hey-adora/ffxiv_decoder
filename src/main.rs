use game_data_resolver::ffxiv::asset::exd::EXD;
use game_data_resolver::ffxiv::asset::exh::{EXHLang, EXH};
use game_data_resolver::ffxiv::buffer::Buffer;
use game_data_resolver::ffxiv::path::DatPath;
use game_data_resolver::ffxiv::FFXIV;
use std::fs;

fn main() {
    rayon::ThreadPoolBuilder::new()
        .num_threads(6)
        .build_global()
        .unwrap();
    let ffxiv = FFXIV::new("/home/night/Games/FINAL FANTASY XIV Online/FINAL FANTASY XIV Online");

    // let a = DatPath::new("bg/ex1/03_abr_a2/dun/a2d2/grass/008_001_017_l.ggd").unwrap();
    // println!("{}", a.index1_hash);
    let paths =
        FFXIV::get_paths("/home/night/Documents/GitHub/game_data_resolver/media/all_paths.txt")
            .unwrap();

    println!("done");
    // for (hash, path) in paths {
    //     println!("'{}' - '{}'", hash, path.path_str);
    // }

    // let path = paths.get(&18370806862700317260u64).unwrap();
    // println!("{}", path.path_str);

    // let wow: String = ffxiv
    //     .export_scd_details("/home/night/Documents/GitHub/game_data_resolver/media/all_paths.txt")
    //     .unwrap();
    // fs::write(
    //     "/home/night/Documents/GitHub/sqex_scd_file_parser/media/jojojo.txt",
    //     wow,
    // )
    // .unwrap();
    //println!("{}", wow);

    //let paths = FFXIV::get_paths("/home/night/Documents/GitHub/game_data_resolver/media/all_paths.txt");
    //ffxiv.export_all_csv("/home/night/Documents/GitHub/sqex_scd_file_parser/media/csv").unwrap();
    //FFXIV::get_paths("/home/night/Documents/GitHub/game_data_resolver/media/all_paths.txt")
    //   .unwrap();

    // ffxiv.export_all(
    //     "/home/night/Documents/aaaaaaaaaaa2",
    //     "/home/night/Documents/GitHub/game_data_resolver/media/all_paths.txt",
    // );

    // let exh = ffxiv.get_asset("exd/custom/000/regseaarmguild_00056.exh").unwrap().decompress().unwrap();
    // let exd = ffxiv.get_asset("exd/custom/000/regseaarmguild_00056_0_en.exd").unwrap().decompress().unwrap();
    //
    // //fs::write("./media/test69.exh", exh).unwrap();
    // //fs::write("./media/test69.exd", exd).unwrap();
    //
    // let exh = EXH::from_vec(exh);
    // let exd = EXD::from_vec(exd, &exh);

    // let root = ffxiv.get_asset("exd/root.exl").unwrap().decompress().unwrap();
    // fs::write("./media/blabla.txt", root).unwrap();

    // let exh_path = DatPath::new("exd/custom/000/regseaarmguild_00056.exh").unwrap();
    // let exh = ffxiv.get_asset_by_dat_path(&exh_path).unwrap().decompress().unwrap();
    // let exd = ffxiv.get_asset("exd/custom/000/regseaarmguild_00056_0_en.exd").unwrap().decompress().unwrap();
    // fs::write("./text88.exd", exd).unwrap();
    // let mut exh = Buffer::from_vec(exh);
    // let exh = EXH::new(& mut exh);
    // let page_1 = ffxiv.get_csv_page(&exh, &exh_path, exh.find_lang(EXHLang::English).unwrap(), exh.rows[0].start_id).unwrap();
    // fs::write("./text88.txt", page_1).unwrap();

    // let exh = ffxiv.get_exh("exd/custom/000/regseaarmguild_00056.exh");
    // let page_1 = exh.get_page(exh.data.find_lang(EXHLang::English).unwrap(), exh.data.rows[0].start_id).unwrap();
}
