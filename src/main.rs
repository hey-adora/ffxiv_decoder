use game_data_resolver::ffxiv::FFXIV;

fn main() {
    let ffxiv = FFXIV::new("/mnt/hdd2/.local/share/Steam/steamapps/common/FINAL FANTASY XIV Online");
    let exh = ffxiv.get_asset_from_path("exd/action.exh").unwrap().to_exh();
    println!("test");
    // ffxiv.get_asset_from_path("exd/action.exh").unwrap().save_decompressed("./media/1.exh");
    // ffxiv.get_asset_from_path("exd/action_0_en.exd").unwrap().save_decompressed("./media/2.exh");
}