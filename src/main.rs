use std::fs;
use game_data_resolver::ffxiv::asset::exd::EXD;
use game_data_resolver::ffxiv::asset::exh::{EXH, EXHLang};
use game_data_resolver::ffxiv::buffer::Buffer;
use game_data_resolver::ffxiv::FFXIV;
use game_data_resolver::ffxiv::path::DatPath;

fn main() {
    let ffxiv = FFXIV::new("/mnt/hdd2/.local/share/Steam/steamapps/common/FINAL FANTASY XIV Online");

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

    let exh_path = DatPath::new("exd/custom/000/regseaarmguild_00056.exh").unwrap();
    let exh = ffxiv.get_asset_by_dat_path(&exh_path).unwrap().decompress().unwrap();
    let exd = ffxiv.get_asset("exd/custom/000/regseaarmguild_00056_0_en.exd").unwrap().decompress().unwrap();
    fs::write("./text88.exd", exd).unwrap();
    let mut exh = Buffer::from_vec(exh);
    let exh = EXH::new(& mut exh);
    let page_1 = ffxiv.get_csv_page(&exh, &exh_path, exh.find_lang(EXHLang::English).unwrap(), exh.rows[0].start_id).unwrap();
    fs::write("./text88.txt", page_1).unwrap();

    println!("test");
}