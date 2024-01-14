use crate::ffxiv::asset_files::FFXIVAssetFiles;
use crate::ffxiv::asset_index_file::{AssetIndexFile, Index1Data1Item};
use crate::ffxiv::asset_path::AssetPath;

pub struct Asset {
    path: AssetPath,
    index1: AssetIndexFile<Index1Data1Item>
}