use std::path::PathBuf;
//use crate::ffxiv::asset::exd::EXD;
use crate::ffxiv::asset::exh::{EXH, EXHColumnKind};
use crate::ffxiv::FFXIV;
use crate::ffxiv::path::DatPath;

pub struct CSV {
    pub path: DatPath,
    pub columns: Vec<EXHColumnKind>,
    pub rows: Vec<Vec<u8>>
}

impl CSV {
    // pub fn new(ffxiv: &FFXIV, path: &str) {
    //     ffxiv.get()
    // }
}

// impl CSV {
//     pub fn new(exh: EXH, exd: EXD) {
//
//
//         for row in exd.rows {
//             let file_path_str = format!("./csvs/{}_{}_en.csv", name, &row.start_id);
//             let file_path_buf = PathBuf::from(&file_path_str);
//             let file_path_dir = file_path_buf.parent().unwrap();
//         }
//     }
// }