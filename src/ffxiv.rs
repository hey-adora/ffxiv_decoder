use crate::ffxiv::asset::dat::{
    DatHeaderType, DatHeaderTypeError, DecompressError, StandardFile, TextureFile,
};
use crate::ffxiv::asset::exd::EXD;
use crate::ffxiv::asset::exh::{EXHColumnKind, EXHLang, EXH};
use crate::ffxiv::asset::exl::EXL;
use crate::ffxiv::asset::index::{Index, Index1Data1Item};
use crate::ffxiv::asset::{Asset, AssetEXHGetPageError, AssetNewError};
use crate::ffxiv::buffer::Buffer;
use crate::ffxiv::path::{DatPath, FilePath, PathError};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs::create_dir_all;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::ops::Add;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use thiserror::Error;

use self::asset::scd::AssetSCDFile;
use self::buffer::{BufferFile, BufferReader};

pub mod asset;
pub mod buffer;
pub mod decode;
pub mod metadata;
pub mod path;

pub struct FFXIV {
    pub asset_files: Vec<FFXIVFileGroup>,
}

impl FFXIV {
    pub fn new(game_path: &str) -> FFXIV {
        let asset_files: Vec<FFXIVFileGroup> = FFXIVFileGroup::new(&game_path);
        FFXIV { asset_files }
    }

    pub fn get_asset(&self, dat_path: &DatPath) -> Result<FileType, AssetFindError> {
        let (dat, index) = self
            .find_asset_by_dat_path(dat_path)
            .ok_or(AssetFindError::NotFound(format!("'{}'", dat_path.path_str)))?;
        self.read_asset_from_file(dat, &index)
    }

    // pub fn get_asset(&self, dat_path: &str) -> Result<FileType, AssetFindError> {
    //     let (dat, index) = self.find_asset(dat_path)?;
    //     self.read_asset(dat, index)
    // }

    // pub fn get_asset_standard(&self, path_str: &str) -> Result<StandardFile, AssetFindError> {
    //     let path = DatPath::new(path_str)?;
    //     let asset = self.get_asset_by_dat_path(&path)?;
    //     match asset {
    //         FileType::Standard(exh) => Ok(exh),
    //         _ => Err(AssetFindError::NotFound(format!("Not standard file '{}'", path_str)))
    //     }
    // }

    pub fn get_asset_standard(&self, path: &DatPath) -> Result<StandardFile, AssetFindError> {
        let asset = self.get_asset(&path)?;
        match asset {
            FileType::Standard(exh) => Ok(exh),
            _ => Err(AssetFindError::NotFound(format!(
                "Not standard file '{}'",
                path.path_str
            ))),
        }
    }

    pub fn read_asset_from_file(
        &self,
        dat: FilePath,
        index: &Index1Data1Item,
    ) -> Result<FileType, AssetFindError> {
        let mut buffer = Buffer::from_file_path(&dat.path);
        self.read_asset_from_buffer(&mut buffer, index)
            .or_else(|e| match e {
                AssetFindError::NotSupported(e) => Err(AssetFindError::NotSupported(format!(
                    "Model type: {}",
                    dat.path_str
                ))),
                AssetFindError::Empty(e) => Err(AssetFindError::Empty(format!("{}", dat.path_str))),
                _ => Err(e),
            })
    }

    pub fn read_asset_from_buffer<R: BufferReader>(
        &self,
        buffer: &mut Buffer<R>,
        index: &Index1Data1Item,
    ) -> Result<FileType, AssetFindError> {
        let header_type = DatHeaderType::check_at(buffer, index.data_file_offset)?;
        match header_type {
            DatHeaderType::Texture => Ok(FileType::Texture(TextureFile::new_at(
                buffer,
                index.data_file_offset,
            ))),
            DatHeaderType::Standard => Ok(FileType::Standard(StandardFile::new_at(
                buffer,
                index.data_file_offset,
            ))),
            DatHeaderType::Model => Err(AssetFindError::NotSupported(format!("Model type"))),
            DatHeaderType::Empty => Err(AssetFindError::Empty(String::new())),
        }
    }

    pub fn find_asset(
        &self,
        dat_path: &str,
    ) -> Result<(FilePath, Index1Data1Item), AssetFindError> {
        let path_dat = DatPath::new(dat_path)?;
        self.find_asset_by_dat_path(&path_dat)
            .ok_or(AssetFindError::NotFound(format!("'{}'", dat_path)))
    }

    pub fn find_asset_by_dat_path(
        &self,
        dat_path: &DatPath,
    ) -> Option<(FilePath, Index1Data1Item)> {
        let possible_asset_files = self.find_possible_files_from_dot_path(dat_path)?;

        for possible_asset_file in possible_asset_files {
            let index_asset = Index::from_index1_file(&possible_asset_file.index1_file.path);

            if let Some(item) = index_asset.find(dat_path.index1_hash) {
                let find_this_dat: String = format!("dat{}", item.data_file_id);
                let dat_file = possible_asset_file
                    .dat_files
                    .iter()
                    .find(|d| d.path_extension == find_this_dat);
                if let Some(dat_file) = dat_file {
                    return Some((dat_file.clone(), item.clone()));
                } else {
                    return None;
                }
            }
        }

        None
    }

    pub fn get_exh(&self, path: DatPath) -> Result<Asset<EXH>, AssetNewError> {
        Asset::new_exh(self, path)
    }

    pub fn get_exd(&self, path: DatPath, exh: &EXH) -> Result<Asset<EXD>, AssetNewError> {
        Asset::new_exd(self, path, exh)
    }

    pub fn get_exl(&self, path: DatPath) -> Result<Asset<EXL>, AssetNewError> {
        Asset::new_exl(self, path)
    }

    pub fn get_scd(&self, path: DatPath) -> Result<Asset<AssetSCDFile>, AssetNewError> {
        Asset::new_scd(self, path)
    }

    pub fn get_paths(paths_file: &str) -> Result<HashMap<u64, DatPath>, AssetPathsError> {
        let paths_file =
            fs::read_to_string(paths_file).or_else(|e| Err(AssetPathsError::IO(e.to_string())))?;
        let paths: Vec<&str> = paths_file.split("\n").collect();
        let len = paths.len();
        let mut path_hashes: HashMap<u64, DatPath> = HashMap::with_capacity(len);
        let path_hashes_mutex: Mutex<HashMap<u64, DatPath>> = Mutex::new(path_hashes);
        //let path_hashes_mutex_arc: Arc<Mutex<HashMap<u64, DatPath>>> = Arc::new(path_hashes_mutex);
        let bar = MultiProgress::new();
        let chunk_size =
            FFXIV::chunk_size(len).or_else(|e| Err(AssetPathsError::ThreadCount(e.to_string())))?;
        let chunks: Vec<&[&str]> = paths.chunks(chunk_size).collect();
        let paths = chunks.par_iter().enumerate().map(
            |(thread_i, paths): (usize, &&[&str])| -> Result<HashMap<u64, DatPath>, AssetPathsError> {
                let len = paths.len();
                let mut path_hashes: HashMap<u64, DatPath> = HashMap::with_capacity(len);
                let bar = bar.add(ProgressBar::new(len as u64));
                // let style = ("Parsing dat paths:", "█  ", "white");
                bar.set_style(
                    ProgressStyle::with_template(&format!(
                        "{{prefix:.bold}}▕{{bar:.{}}}▏{{pos}}",
                        "white"
                    ))
                    .unwrap()
                    .progress_chars("█  "),
                );
                bar.set_prefix("Parsing path");

                for (i, path) in (*paths).iter().enumerate() {
                    //bar.set_message(format!("{}/{} - {}", i + 1, len, path.to_owned()));
                    if path.len() > 3 {
                        let parsed_path = DatPath::new(&path)?;
                        path_hashes
                            .insert(parsed_path.index1_hash, parsed_path);
                    }
                    bar.inc(1);
                }

                Ok(path_hashes)
            },
        ).try_reduce(|| HashMap::<u64, DatPath>::with_capacity(len), | mut a: HashMap<u64, DatPath>, b: HashMap<u64, DatPath> | -> Result<HashMap<u64, DatPath>, AssetPathsError> {
                a.extend(b);

                Ok(a)
            })?;
        // let mut path_hasmap_vec_cleaned: HashMap<u64, DatPath> = HashMap::with_capacity(len);
        // for path_hasmap_dirty in path_hasmap_dirty_vec {
        //     let path_hasmap_clean: HashMap<u64, DatPath> = path_hasmap_dirty?;
        //     path_hasmap_vec_cleaned.extend(path_hasmap_clean);
        // }
        Ok(paths)
        //Ok(path_hashes_mutex.into_inner().unwrap())

        // let mut thread_handles = vec![];
        // let mut thread_count = std::thread::available_parallelism()
        //     .or_else(|e| Err(AssetPathsError::ThreadCount(e.to_string())))?
        //     .get();
        // if thread_count < 2 {
        //     thread_count = 2;
        // }
        //
        // let paths_file =
        //     fs::read_to_string(paths_file).or_else(|e| Err(AssetPathsError::IO(e.to_string())))?;
        // let paths: Vec<&str> = paths_file.split("\n").collect();
        // let line_count = paths.len();
        // if thread_count > line_count {
        //     thread_count = line_count;
        // }
        //
        // let path_chunks: Vec<&[&str]> = paths.chunks(line_count / (thread_count - 1)).collect();
        //
        // let path_hashes_arc_mutex = Mutex::new(path_hashes);
        // let path_hashes_arc = Arc::new(path_hashes_arc_mutex);
        // for (thread_index, chunk) in path_chunks.iter().enumerate() {
        //     let paths_block: Vec<String> = chunk.to_vec().iter().map(|p| (*p).to_owned()).collect();
        //     let line_count = paths_block.len();
        //     let path_hashes_clone = Arc::clone(&path_hashes_arc);
        //
        //     let handle = std::thread::spawn(move || -> Result<(), AssetPathsThreadError> {
        //         let max_index: f32 = line_count as f32;
        //         let check_every: f32 = (max_index / 100.0).floor();
        //         let mut path_hashes: HashMap<u64, DatPath> = HashMap::new();
        //
        //         for (index, path) in paths_block.iter().enumerate() {
        //             let parsed_path = DatPath::new(&path);
        //             if let Ok(parsed_path) = parsed_path {
        //                 path_hashes.insert(parsed_path.index1_hash, parsed_path);
        //
        //                 let index = index as f32;
        //                 if index % check_every == 0.0 {
        //                     let done = (index / max_index) * 100.0;
        //                     println!("Thread {} Reading path: {}%.\n", thread_index, done);
        //                 }
        //             }
        //         }
        //         path_hashes_clone
        //             .lock()
        //             .or_else(|e| Err(AssetPathsThreadError::ThreadLock(e.to_string())))?
        //             .extend(path_hashes);
        //         Ok(())
        //     });
        //     thread_handles.push(handle);
        // }
        //
        // for thread_handle in thread_handles {
        //     thread_handle.join().unwrap()?;
        // }
        //
        // let gg = Arc::try_unwrap(path_hashes_arc)
        //     .unwrap()
        //     .into_inner()
        //     .unwrap();
        //
        // Ok(gg)
    }

    pub fn chunk_size(len: usize) -> anyhow::Result<usize> {
        let mut thread_count = std::thread::available_parallelism()?.get();
        if thread_count < 2 {
            thread_count = 2;
        }

        if thread_count > len {
            thread_count = len;
        }

        let count = len / (thread_count - 1);
        anyhow::Ok(count)
    }

    pub fn get_all_hash_dat_index1item(&self) -> HashMap<u64, (String, Index1Data1Item)> {
        let mut map: HashMap<u64, (String, Index1Data1Item)> = HashMap::new();

        for group in &self.asset_files {
            let index1 = Index::from_index1_file(&group.index1_file.path);
            for item in index1.data1 {
                let find_this_dat = format!("dat{}", item.data_file_id);
                let dat_file = group
                    .dat_files
                    .iter()
                    .find(|d| d.path_extension == find_this_dat);
                if let Some(dat_file_path) = dat_file {
                    map.insert(item.hash, (dat_file_path.path_str.clone(), item));
                }
            }
        }

        map
    }

    pub fn get_all_dat_index1item_hashmap(&self) -> HashMap<String, Vec<Index1Data1Item>> {
        let mut map: HashMap<String, Vec<Index1Data1Item>> = HashMap::new();

        for group in &self.asset_files {
            let index1 = Index::from_index1_file(&group.index1_file.path);
            for item in index1.data1 {
                let find_this_dat = format!("dat{}", item.data_file_id);
                let dat_file = group
                    .dat_files
                    .iter()
                    .find(|d| d.path_extension == find_this_dat);
                if let Some(dat_file) = dat_file {
                    let dat_items = map.get_mut(&dat_file.path_str);
                    if let Some(dat_items) = dat_items {
                        dat_items.push(item.clone());
                    } else {
                        map.insert(dat_file.path_str.clone(), vec![item.clone()]);
                    }
                }
            }
        }

        map
    }

    pub fn get_all_dat_index1item_vec(&self) -> Vec<(String, Vec<Index1Data1Item>)> {
        let mut vec: Vec<(String, Vec<Index1Data1Item>)> = Vec::new();

        for group in &self.asset_files {
            let index1 = Index::from_index1_file(&group.index1_file.path);
            for item in index1.data1 {
                let dat_file = group.dat_files.iter().find(|d| {
                    d.path_extension == FFXIVFileGroup::gen_dat_id_str(item.data_file_id)
                });
                if let Some(dat_file) = dat_file {
                    let dat_items_pos = vec
                        .iter()
                        .position(|(path, items)| *path == dat_file.path_str);
                    if let Some(dat_items_pos) = dat_items_pos {
                        let (dat, items) = vec.get_mut(dat_items_pos).unwrap();
                        items.push(item.clone());
                    } else {
                        vec.push((dat_file.path_str.clone(), vec![item.clone()]));
                    }
                }
            }
        }

        vec
    }

    // impl rayon::iter::FromParallelIterator<std::result::Result<std::collections::HashMap<u64, ffxiv::path::DatPath>, ffxiv::AssetPathsError>> for std::result::Result<std::collections::HashMap<u64, ffxiv::path::DatPath>, ffxiv::AssetPathsError> {
    //
    //    }
    // pub fn export_audio_details(&self, path_names: &str) -> anyhow::Result<String> {
    //     //let dats_hash = self.get_all_dat_index1item();
    //     //let names = FFXIV::get_paths(path_names)?;
    //     let lines: RefCell<String> = RefCell::new(String::new());
    //     self.export_iter(
    //         path_names,
    //         |dat_buffer: &mut Buffer<BufferFile>,
    //          item: &Index1Data1Item,
    //          item_name: Option<&DatPath>|
    //          -> anyhow::Result<()> {
    //             if let Some(item_name) = item_name {
    //                 if item_name.path_extension == "scd" {
    //                     let standart_asset =
    //                         StandardFile::new_at(dat_buffer, item.data_file_offset);
    //                     let asset_vec = standart_asset.decompress()?;
    //                     let asset = AssetSCDFile::from_vec(asset_vec);
    //                     lines
    //                         .borrow_mut()
    //                         .push_str(&format!("channels: {}\n", asset.entry_channels));
    //                 }
    //             }
    //             anyhow::Ok(())
    //         },
    //     )?;
    //     anyhow::Ok(lines.into_inner())
    // }

    // pub fn export_iter<
    //     T,
    //     F: Fn(&mut Buffer<BufferFile>, &Index1Data1Item, Option<&DatPath>) -> anyhow::Result<T>,
    // >(
    //     &self,
    //     path_names: &str,
    //     callback: F,
    // ) -> anyhow::Result<()> {
    //     let dats_hash = self.get_all_dat_index1item();
    //     let names = FFXIV::get_paths(path_names)?;
    //
    //     let i_max: usize = dats_hash.iter().fold(0, |a, b| a + b.1.len());
    //     let bar = MultiProgress::new();
    //
    //     dats_hash
    //         .iter()
    //         .map(|(dat, items)| -> anyhow::Result<T> {
    //             let bar = bar.add(ProgressBar::new(items.len() as u64));
    //             let style = (dat.clone(), "█  ", "white");
    //             bar.set_style(
    //                 ProgressStyle::with_template(&format!(
    //                     "{{prefix:.bold}}▕{{bar:.{}}}▏{{msg}}",
    //                     style.2
    //                 ))
    //                 .unwrap()
    //                 .progress_chars(style.1),
    //             );
    //             bar.set_prefix(style.0);
    //             let mut buffer = Buffer::from_file_path(dat);
    //             for (i, item) in items.iter().enumerate() {
    //                 bar.set_message(format!("{}/{}", i + 1, i_max));
    //                 let org_name = names.get(&item.hash);
    //                 callback(&mut buffer, item, org_name)?;
    //                 bar.inc(1);
    //             }
    //
    //             bar.set_message(format!("{}/{}", i_max, i_max));
    //             Ok(())
    //         });
    //     Ok(())
    // }
    pub fn export_scd_details(
        &self,
        path_names: &str,
        export_path: &str,
    ) -> Result<(), AssetExportError> {
        let dats_items = self.get_all_dat_index1item_hashmap();
        let paths = FFXIV::get_paths(path_names)?;

        let i_max: usize = dats_items.iter().fold(0, |a, b| a + b.1.len());
        let bar = ProgressBar::new(i_max as u64);
        //let lines: String = String::with_capacity(i_max * 20);
        let export_path_buf = PathBuf::from(export_path);
        if !export_path_buf.is_file() {
            return Err(AssetExportError::ExportPath(format!(
                "Export path must be a file: {}",
                export_path
            )));
        }
        let file = if export_path_buf.exists() {
            File::open(export_path_buf)
                .or_else(|e| Err(AssetExportError::ExportPath(e.to_string())))?
        } else {
            File::create(export_path_buf)
                .or_else(|e| Err(AssetExportError::ExportPath(e.to_string())))?
        };
        let mut writer = BufWriter::new(file);
        for (dat, items) in dats_items {
            for item in items {
                write!(writer, "channels: {}", "asset.entry_channels");
            }
        }

        Ok(())
        //let bar = MultiProgress::new();
        // let lines: Mutex<String> = Mutex::new(lines);
        //
        // dats_hash
        //     .par_iter()
        //     .try_for_each(|(dat, items)| -> Result<(), AssetExportError> {
        //         let items_len = items.len();
        //         //let mut lines = String::with_capacity(items_len * 500);
        //         let bar = bar.add(ProgressBar::new(items_len as u64));
        //         bar.set_style(
        //             ProgressStyle::with_template(&format!(
        //                 "{{prefix:.bold}}▕{{bar:.{}}}▏{{pos}}",
        //                 "white"
        //             ))
        //             .unwrap()
        //             .progress_chars("█  "),
        //         );
        //         bar.set_prefix(dat.clone());
        //         let mut buffer = Buffer::from_file_path(dat);
        //         for (i, item) in items.iter().enumerate() {
        //             let org_name = names.get(&item.hash);
        //             if let Some(item_name) = org_name {
        //                 if item_name.path_extension == "scd" {
        //                     let standart_asset =
        //                         StandardFile::new_at(&mut buffer, item.data_file_offset);
        //                     let asset_vec = standart_asset.decompress()?;
        //                     let asset = AssetSCDFile::from_vec(asset_vec);
        //                     lines
        //                         .lock()
        //                         .unwrap()
        //                         .borrow_mut()
        //                         .push_str(&format!("channels: {}\n", asset.entry_channels));
        //                 }
        //             }
        //             bar.inc(1);
        //         }
        //         bar.finish();
        //         Ok(())
        //     });
        //
        // Ok(lines.into_inner().unwrap())
    }

    pub fn export_all(&self, export_path: &str, path_names: &str) -> Result<(), AssetExportError> {
        let dats_hash = self.get_all_dat_index1item_hashmap();
        let names = FFXIV::get_paths(path_names)?;

        let i_max: usize = dats_hash.iter().fold(0, |a, b| a + b.1.len());
        //let bar = ProgressBar::new(i_max as u64);
        // let style = ("Parsing dat paths:", "█  ", "white");
        // bar.set_style(
        //     ProgressStyle::with_template(&format!("{{prefix:.bold}}▕{{bar:.{}}}▏{{msg}}", style.2))
        //         .unwrap()
        //         .progress_chars(style.1),
        // );
        // bar.set_prefix(style.0);
        let bar = MultiProgress::new();

        dats_hash.par_iter().for_each(|(dat, items)| {
            let bar = bar.add(ProgressBar::new(items.len() as u64));
            let style = (dat.clone(), "█  ", "white");
            bar.set_style(
                ProgressStyle::with_template(&format!(
                    "{{prefix:.bold}}▕{{bar:.{}}}▏{{msg}}",
                    style.2
                ))
                .unwrap()
                .progress_chars(style.1),
            );
            bar.set_prefix(style.0);
            let mut buffer = Buffer::from_file_path(dat);
            for (i, item) in items.iter().enumerate() {
                bar.set_message(format!("{}/{}", i + 1, i_max));
                let org_name = names.get(&item.hash);
                let item_name: String;
                if let Some(org_name) = org_name {
                    item_name = format!("{}_{}", item.hash.to_string(), org_name.path_name.clone());
                } else {
                    item_name = item.hash.to_string();
                }
                let new_item_path = PathBuf::from(format!("{}/{}", export_path, item_name));

                if !new_item_path.exists() {
                    let header_type =
                        DatHeaderType::check_at(&mut buffer, item.data_file_offset).unwrap();
                    if let DatHeaderType::Standard = header_type {
                        let file = StandardFile::new_at(&mut buffer, item.data_file_offset);
                        let data = file.decompress().unwrap();
                        fs::write(new_item_path, data).unwrap();
                    }
                }
                bar.inc(1);
            }

            bar.set_message(format!("{}/{}", i_max, i_max));
        });

        // let dat_chunks: Vec<(usize, (&String, &Vec<Index1Data1Item>))> =
        //     dats_hash.iter().enumerate().collect();
        // std::thread::scope(|scope| {
        //     for (thread_index, (dat, items)) in dat_chunks.iter() {
        //         scope.spawn(|| {
        //             let max_index: f32 = items.len() as f32;
        //             let check_every: f32 = (max_index / 100.0).floor();
        //
        //             let mut buffer = Buffer::from_file_path(*dat);
        //             for (index, item) in items.iter().enumerate() {
        //                 let org_name = names.get(&item.hash);
        //                 let item_name: String;
        //                 if let Some(org_name) = org_name {
        //                     item_name =
        //                         format!("{}_{}", item.hash.to_string(), org_name.path_name.clone());
        //                 } else {
        //                     item_name = item.hash.to_string();
        //                 }
        //                 let new_item_path = PathBuf::from(format!("{}/{}", export_path, item_name));
        //
        //                 if !new_item_path.exists() {
        //                     let header_type =
        //                         DatHeaderType::check_at(&mut buffer, item.data_file_offset)
        //                             .unwrap();
        //                     if let DatHeaderType::Standard = header_type {
        //                         let file = StandardFile::new_at(&mut buffer, item.data_file_offset);
        //                         let data = file.decompress().unwrap();
        //                         fs::write(new_item_path, data).unwrap();
        //                     }
        //                 }
        //
        //                 let index = index as f32;
        //                 if index % check_every == 0.0 {
        //                     let done = (index / max_index) * 100.0;
        //                     println!("THREAD {}: Exporting {}: {}%.\n", *thread_index, *dat, done);
        //                 }
        //             }
        //         });
        //     }
        // });

        Ok(())
    }

    pub fn export_all_csv(&self, export_path: &str) -> Result<(), CSVExportError> {
        let export_path_buf = PathBuf::from(export_path);
        let exl_path = DatPath::new("exd/root.exl")?;
        let exl = self.get_exl(exl_path)?;

        let i_max = exl.data.lines.len() as u64;
        let bar = ProgressBar::new(i_max);
        let style = ("Rough bar:", "█  ", "white");
        bar.set_style(
            ProgressStyle::with_template(&format!("{{prefix:.bold}}▕{{bar:.{}}}▏{{msg}}", style.2))
                .unwrap()
                .progress_chars(style.1),
        );
        bar.set_prefix(style.0);
        for (i, (name, uwnk)) in exl.data.lines.iter().enumerate() {
            let exh_name = &format!("exd/{}.exh", name);
            bar.set_message(format!("{}/{} - {}", i + 1, i_max, exh_name.clone()));
            let exh_path = DatPath::new(exh_name)?;
            let exh = self.get_exh(exh_path);
            if let Ok(exh) = exh {
                let exh_lang = exh
                    .data
                    .find_lang(EXHLang::English)
                    .unwrap_or(&EXHLang::None);
                let pages = exh.get_pages(exh_lang)?;

                for (path, csv) in pages {
                    let export_dir = export_path_buf.join(path.path_dir);
                    let export_file = export_dir.join(format!("{}.csv", path.path_stem));
                    if !export_file.exists() {
                        create_dir_all(&export_dir).or_else(|e| {
                            Err(CSVExportError::CreatingDir(format!(
                                "'{}' for '{}'",
                                e.to_string(),
                                path.path_str
                            )))
                        })?;
                        fs::write(export_file, csv).or_else(|e| {
                            Err(CSVExportError::WritingFile(format!("{}", e.to_string())))
                        })?;
                    }
                }
            }
            bar.inc(1);
        }
        bar.finish();

        Ok(())
    }

    // pub fn save_all_cvs(&self) -> Result<(), CSVExportError> {
    //     let
    //     // let exl =  self.get_asset("exd/root.exl")?.decompress()?;
    //     // let exl = EXL::from_vec(exl);
    //     // for (name, ukwn) in exl.lines {
    //     //     let name = name.to_lowercase();
    //     //     let asset_path = &format!("exd/{}.exh", name);
    //     //     let exh =  self.get_asset(asset_path)?;
    //     //
    //     //     if let Ok(exh) = exh {
    //     //         let exh = EXH::from_vec(exh.decompress()?);
    //     //
    //     //         let exd_lang_prefix = match &exh.languages[0] {
    //     //             EXHLang::None => String::from(".exd"),
    //     //             _ => String::from("_en.exd")
    //     //         };
    //     //
    //     //         for row in &exh.rows{
    //     //             let file_path_str = format!("./csvs/{}_{}_en.csv", name, &row.start_id);
    //     //             let file_path_buf = PathBuf::from(&file_path_str);
    //     //             let file_path_dir = file_path_buf.parent().unwrap();
    //     //
    //     //             if !file_path_buf.exists() {
    //     //                 let mut rows: String = exh.to_string();
    //     //                 rows.push_str("\n\n");
    //     //
    //     //                 let exd_asset_path = &format!("exd/{}_{}{}", name, &row.start_id, exd_lang_prefix);
    //     //                 let exd =  self.get_asset(exd_asset_path).ok_or(CSVExportError::EXDNotFound(asset_path.to_owned()))?;
    //     //                 let exd = EXD::from_vec(exd.decompress()?, &exh);
    //     //
    //     //                 rows.push_str(&exd.to_string());
    //     //
    //     //                 create_dir_all(file_path_dir).unwrap();
    //     //                 fs::write(file_path_buf, rows).unwrap();
    //     //                 println!("Saved {}", file_path_str);
    //     //             } else {
    //     //                 println!("Skipped: {}", asset_path)
    //     //             }
    //     //         }
    //     //     } else {
    //     //         println!("Not found: {}", asset_path)
    //     //     }
    //     // }
    //
    //     Ok(())
    // }

    fn find_possible_files_from_dot_path(
        &self,
        asset_path: &DatPath,
    ) -> Option<Vec<&FFXIVFileGroup>> {
        let mut possible_asset_files: Vec<&FFXIVFileGroup> = Vec::new();

        for asset_file in &self.asset_files {
            if asset_file.index1_file.data_category.id == asset_path.data_category.id
                && asset_file.index1_file.data_repository.id == asset_path.data_repo.id
            {
                possible_asset_files.push(asset_file);
            }
        }

        if possible_asset_files.len() > 0 {
            Some(possible_asset_files)
        } else {
            None
        }
    }
}

//==================================================================================================

pub struct FFXIVFileGroup {
    pub dat_files: Vec<FilePath>,
    pub index1_file: FilePath,
    pub index2_file: FilePath,
}

impl FFXIVFileGroup {
    pub fn new(game_path: &str) -> Vec<FFXIVFileGroup> {
        let mut file_paths: Vec<PathBuf> = Vec::new();
        FFXIVFileGroup::get_files(game_path, &mut file_paths);

        let mut dat_files: Vec<FilePath> = Vec::new();
        let mut index_files: Vec<FilePath> = Vec::new();
        let mut index2_files: Vec<FilePath> = Vec::new();

        for file_path in file_paths {
            let file_metadata = FilePath::new(file_path);
            if let Ok(file_metadata) = file_metadata {
                if file_metadata.path_extension == "index" {
                    index_files.push(file_metadata);
                } else if file_metadata.path_extension == "index2" {
                    index2_files.push(file_metadata);
                } else {
                    dat_files.push(file_metadata);
                }
            }
        }

        let grouped_files = FFXIVFileGroup::group_files(dat_files, index_files, index2_files);

        grouped_files
    }

    pub fn group_files(
        dat_files: Vec<FilePath>,
        index_files: Vec<FilePath>,
        index2_files: Vec<FilePath>,
    ) -> Vec<FFXIVFileGroup> {
        let mut file_groups: Vec<FFXIVFileGroup> = Vec::new();

        for index_file in index_files {
            let index2_file = index2_files.iter().find(|f| **f == index_file);
            if let Some(index2_file) = index2_file {
                let dat_files: Vec<FilePath> = dat_files
                    .iter()
                    .filter(|f| **f == index_file)
                    .map(|f| f.clone())
                    .collect();
                if dat_files.len() == 0 {
                    continue;
                }

                file_groups.push(FFXIVFileGroup {
                    index1_file: index_file,
                    index2_file: (*index2_file).clone(),
                    dat_files,
                })
            }
        }
        file_groups
    }

    pub fn get_files(input_path: &str, output: &mut Vec<PathBuf>) {
        let verify = Path::new(input_path);
        let flag = verify.is_dir();
        if !flag {
            panic!("Path is not a directory: {}", input_path)
        }
        let flag = verify.exists();
        if !flag {
            panic!("Path doesn't not exist: {}", input_path)
        }
        let paths = fs::read_dir(input_path).unwrap();
        for path in paths {
            let path = path.unwrap().path();
            if path.is_file() {
                output.push(path)
            } else {
                FFXIVFileGroup::get_files(path.to_str().unwrap(), output);
            }
        }
    }

    pub fn gen_dat_id_str(i: u32) -> String {
        match i {
            0 => String::from("dat0"),
            1 => String::from("dat1"),
            2 => String::from("dat2"),
            3 => String::from("dat3"),
            4 => String::from("dat4"),
            5 => String::from("dat5"),
            6 => String::from("dat6"),
            i => format!("dat{}", i),
        }
    }
}

//==================================================================================================

#[derive(Clone)]
pub enum FileType {
    Empty,
    Standard(StandardFile),
    Model,
    Texture(TextureFile),
}

impl Display for FileType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FileType::Empty => "Empty",
                FileType::Standard(_) => "Standard",
                FileType::Model => "Model",
                FileType::Texture(_) => "Texture",
            }
        )
    }
}

impl FileType {
    pub fn to_u32(&self) -> u32 {
        match self {
            FileType::Empty => 1,
            FileType::Standard(_) => 2,
            FileType::Model => 3,
            FileType::Texture(_) => 4,
        }
    }

    pub fn decompress(&self) -> Result<Vec<u8>, FileTypeError> {
        match self {
            FileType::Texture(t) => Ok(t.decompress()?),
            FileType::Standard(s) => Ok(s.decompress()?),
            _ => Err(FileTypeError::UnsupportedFileType(self.to_string())),
        }
    }
}

//==================================================================================================

#[derive(Error, Debug)]
pub enum FileTypeError {
    #[error("FileType: '{0}' not supported.")]
    UnsupportedFileType(String),

    #[error("Decompression error: {0}")]
    DecompressError(#[from] DecompressError),
}

#[derive(Error, Debug)]
pub enum AssetFindError {
    #[error("Not found: '{0}'.")]
    NotFound(String),

    #[error("Not supported: '{0}'.")]
    NotSupported(String),

    #[error("Empty file found: '{0}'.")]
    Empty(String),

    #[error("Path error: {0}")]
    PathError(#[from] PathError),

    #[error("Invalid header type: {0}")]
    InvalidFileType(#[from] DatHeaderTypeError),
}

#[derive(Error, Debug)]
pub enum AssetPathsError {
    #[error("Failed to get thread count '{0}'.")]
    ThreadCount(String),
    //
    // #[error("Failed to lock hashmap '{0}'.")]
    // Thread(#[from] AssetPathsThreadError),
    //
    #[error("Not found: '{0}'.")]
    IO(String),

    #[error("Failed to parse path: '{0}'.")]
    DatPathError(#[from] PathError),
}

#[derive(Error, Debug)]
pub enum AssetPathsThreadError {
    // #[error("Failed to parse dat path '{0}'.")]
    // DatPath(#[from] PathError),
    #[error("Failed to lock hashmap '{0}'.")]
    ThreadLock(String),
}

#[derive(Error, Debug)]
pub enum AssetExportError {
    #[error("Export path is invalid: '{0}'.")]
    ExportPath(String),

    #[error("Failed to get paths: '{0}'.")]
    AssetPath(#[from] AssetPathsError),

    #[error("Failed to get thread count '{0}'.")]
    ThreadCount(String),

    #[error("Decompression error: {0}")]
    DecompressError(#[from] DecompressError),
}

#[derive(Error, Debug)]
pub enum CSVExportError {
    #[error("Path parsing error: {0}")]
    Path(#[from] PathError),

    #[error("Creating asset error: {0}")]
    CreatingAsset(#[from] AssetNewError),

    #[error("Getting EXH page error: {0}")]
    EXHGetPage(#[from] AssetEXHGetPageError),

    #[error("Creating all directories error: {0}")]
    CreatingDir(String),

    #[error("Writing file error: {0}")]
    WritingFile(String),
}
