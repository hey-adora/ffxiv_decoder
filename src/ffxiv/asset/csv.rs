use std::path::PathBuf;
//use crate::ffxiv::asset::exd::EXD;
use crate::ffxiv::asset::exh::{EXH, EXHColumnKind};
use crate::ffxiv::FFXIV;
use crate::ffxiv::path::DatPath;

// pub struct CSV {
//     pub path: DatPath,
//     pub columns: Vec<EXHColumnKind>,
//     pub rows: Vec<Vec<u8>>
// }
//
// impl CSV {
//     pub fn new(ffxiv: &FFXIV, path: &str) {
//         let exh_lang = exh
//             .data
//             .find_lang(EXHLang::English)
//             .unwrap_or(&EXHLang::None);
//         let pages = exh.get_pages(exh_lang)?;
//
//         for (path, csv) in pages {
//             let export_dir = export_path_buf.join(path.path_dir);
//             let export_file = export_dir.join(format!("{}.csv", path.path_stem));
//             if !export_file.exists() {
//                 create_dir_all(&export_dir).or_else(|e| {
//                     Err(CSVExportError::CreatingDir(format!(
//                         "'{}' for '{}'",
//                         e.to_string(),
//                         path.path_str
//                     )))
//                 })?;
//                 fs::write(export_file, csv).or_else(|e| {
//                     Err(CSVExportError::WritingFile(format!("{}", e.to_string())))
//                 })?;
//             }
//         }
//     }
// }

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