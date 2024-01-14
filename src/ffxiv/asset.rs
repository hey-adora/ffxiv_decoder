use thiserror::Error;
use crate::ffxiv::asset::exh::{EXH, EXHLang};
use crate::ffxiv::{FFXIV, FileType, FileTypeError};
use crate::ffxiv::asset::exd::EXD;
use crate::ffxiv::asset::exl::EXL;
use crate::ffxiv::buffer::Buffer;
use crate::ffxiv::path::DatPath;

pub mod index;
pub mod dat;
pub mod exd;
pub mod exh;
pub mod scd;
pub mod exl;
pub mod csv;


pub struct Asset<'a, T> {
    pub path: DatPath,
    pub data: T,
    pub game: &'a FFXIV
}

impl <'a> Asset<'a, EXH> {
    pub fn new_exh(ffxiv: &'a FFXIV, path: &str) -> Asset<'a, EXH> {
        let path = DatPath::new(path).unwrap();
        let asset = ffxiv.get_asset_by_dat_path(&path).unwrap();
        let data = match asset {
            FileType::Standard(exh) => exh,
            _ => panic!("wrong file type")
        };
        let data = data.decompress().unwrap();
        let mut data = Buffer::from_vec(data);
        let data = EXH::new(&mut data);
        Asset {
            path,
            data,
            game: ffxiv
        }
    }

    pub fn get_page(&self, lang: &EXHLang, page: u32) -> Result<String, CSVExportError> {
        let exd_asset_path = Asset::make_exd_path(&self.path.path_dir, &self.path.path_stem, lang, page);
        let exd =  self.game.get_asset_by_dat_path(&exd_asset_path).ok_or(CSVExportError::EXDNotFound(exd_asset_path.path_str))?;
        let exd = EXD::from_vec(exd.decompress()?, &self.data);

        let mut csv: String = self.data.to_string();
        csv.push_str("\n\n");
        csv.push_str(&exd.to_string());

        Ok(csv)
    }

    pub fn export_all(&self, path: &str) {
        let exl = self.game.get_asset("exd/root.exl");

    }

    pub fn make_exd_path(dir: &str, name: &str, lang: &EXHLang, page: u32) -> DatPath {
        let exd_asset_path = format!("{}/{}_{}_{}.exd", dir , name, page, lang);
        DatPath::new(&exd_asset_path).unwrap()
    }
}

impl <'a> Asset<'a, EXL> {
    pub fn new_exl(ffxiv: &'a FFXIV, path: &str) -> Asset<'a, EXL> {
        let path = DatPath::new(path).unwrap();
        let asset = ffxiv.get_asset_by_dat_path(&path).unwrap();
        let data = match asset {
            FileType::Standard(exh) => exh,
            _ => panic!("wrong file type")
        };
        let data = data.decompress().unwrap();
        let data = EXL::from_vec(data);
        Asset {
            path,
            data,
            game: ffxiv
        }
    }
}


#[derive(Error, Debug)]
pub enum CSVExportError {
    #[error("'exd/root.exl' not found at")]
    RootNotFound,

    #[error("EXH not found: {0}")]
    EXHNotFound(String),

    #[error("EXD not found: {0}")]
    EXDNotFound(String),

    #[error("'exd/root.exl' failed to decompress: {0}")]
    RootDecompressionFail(#[from] FileTypeError),
}
