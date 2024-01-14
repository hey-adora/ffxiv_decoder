use thiserror::Error;
use crate::ffxiv::asset::exh::{EXH, EXHLang};
use crate::ffxiv::{AssetFindError, FFXIV, FileType, FileTypeError};
use crate::ffxiv::asset::dat::DecompressError;
use crate::ffxiv::asset::exd::EXD;
use crate::ffxiv::asset::exl::EXL;
use crate::ffxiv::buffer::Buffer;
use crate::ffxiv::path::{DatPath, PathError};

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
    pub fn new_exh(ffxiv: &'a FFXIV, exh_path: DatPath) -> Result<Asset<'a, EXH>, AssetNewError> {
        //let path = DatPath::new(path).unwrap();
        let asset = ffxiv.get_asset(&exh_path)?;
        let data = match asset {
            FileType::Standard(exh) => exh,
            _ => panic!("wrong file type")
        };
        let data = data.decompress().unwrap();
        let mut data = Buffer::from_vec(data);
        let data = EXH::new(&mut data);
        let asset = Asset {
            path: exh_path,
            data,
            game: ffxiv
        };
        Ok(asset)
    }

    pub fn get_page(&self, lang: &EXHLang, page: u32) -> Result<(DatPath, String), AssetEXHGetPageError> {
        let exd_asset_path = Asset::make_exd_path(&self.path.path_dir, &self.path.path_stem, lang, page);
        let exd = Asset::new_exd(self.game, exd_asset_path.clone(), &self.data)?;

        let mut csv: String = self.data.to_string();
        csv.push_str("\n\n");
        csv.push_str(&exd.data.to_string());

        Ok((exd_asset_path, csv))
    }

    pub fn get_pages(&self, lang: &EXHLang) -> Result<Vec<(DatPath, String)>, AssetEXHGetPageError> {
        let mut pages: Vec<(DatPath, String)> = Vec::new();

        for row in &self.data.rows {
            let page = self.get_page(lang, row.start_id)?;
            pages.push(page)
        }

        Ok(pages)
    }

    // pub fn export_all(&self, path: &str) -> Result<(), CSVExportError> {
    //     //let exl = Asset::new_exl(self.game)?;
    //     for (exh_name, unk) in exl.data.lines {
    //
    //     }
    //
    //
    //     0
    // }

    pub fn make_exd_path(dir: &str, name: &str, lang: &EXHLang, page: u32) -> DatPath {
        let exd_asset_path = if let EXHLang::None = lang {
            format!("{}/{}_{}.exd", dir , name, page)
        } else {
            format!("{}/{}_{}_{}.exd", dir , name, page, lang)
        };
        
        DatPath::new(&exd_asset_path).unwrap()
    }
}

impl <'a> Asset<'a, EXL> {
    pub fn new_exl(ffxiv: &'a FFXIV, exl_path: DatPath) -> Result<Asset<'a, EXL>, AssetNewError> {
        //let path = DatPath::new("exd/root.exl")?;
        let asset = ffxiv.get_asset_standard(&exl_path)?.decompress()?;
        let data = EXL::from_vec(asset);

        let asset = Asset {
            path: exl_path,
            data,
            game: ffxiv
        };
        Ok(asset)
    }
}

impl <'a> Asset<'a, EXD> {
    pub fn new_exd(ffxiv: &'a FFXIV, exd_path: DatPath, exh: &EXH) -> Result<Asset<'a, EXD>, AssetNewError> {
        let asset = ffxiv.get_asset_standard(&exd_path)?.decompress()?;
        let data = EXD::from_vec(asset, exh);

        let asset = Asset {
            path: exd_path,
            data,
            game: ffxiv
        };
        Ok(asset)
    }
}

#[derive(Error, Debug)]
pub enum AssetNewError {
    // #[error("Path error: {0}")]
    // PathError(#[from] PathError),

    #[error("'exd/root.exl' not found at")]
    AssetFindError(#[from] AssetFindError),

    #[error("Decompression error: {0}")]
    DecompressError(#[from] DecompressError)
}

#[derive(Error, Debug)]
pub enum AssetEXHGetPageError {
    #[error("NewEXD Error: {0}")]
    NewEXDError(#[from] AssetNewError),

    #[error("{0}")]
    AssetFindError(#[from] AssetFindError)
}

