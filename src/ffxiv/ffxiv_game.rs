use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use crate::ffxiv::ffxiv_asset::{FFXIVAssetParserDatHeaderType, FFXIVAssetParserIndex, FFXIVAssetParserIndex1Data1Item, FFXIVAssetPathDat, FFXIVAssetPathFile, FileType, StandardFile, TextureFile};
use crate::ffxiv::ffxiv_buffer::FFXIVBuffer;

// pub struct FFXIVGame {
//     game_path: String,
//     asset_files: Vec<FFXIVAssetFiles>
// }
//
// impl FFXIVGame {
//     pub fn new(game_path: &str) -> FFXIVGame {
//         let asset_files: Vec<FFXIVAssetFiles> = FFXIVAssetFiles::new(&game_path).unwrap();
//         FFXIVGame {
//             game_path: String::from(game_path),
//             asset_files
//         }
//     }
//
//     pub fn get_asset_from_dat(&self, dat_path: &str) -> Option<FFXIVAssetParserDat> {
//         let path_dat = FFXIVAssetPathDat::from_str(dat_path).unwrap();
//         let possible_asset_files = self.find_asset_files_by_asset_path(&path_dat);
//
//         for possible_asset_file in possible_asset_files {
//             let index_asset = FFXIVAssetParserIndex::from_index1_file(&possible_asset_file.index_file.path);
//             if let Some(item) = index_asset.find(path_dat.index1_hash) {
//                 let dat_file = FFXIVAssetParserDat::from_dat_files(&possible_asset_file.dat_files, item.data_file_id, item.data_file_offset);
//                 return Some(dat_file);
//             }
//         };
//
//         None
//     }
//
//     fn find_asset_files_by_asset_path(&self, asset_path: &FFXIVAssetPathDat) -> Vec<&FFXIVAssetFiles>{
//         let mut possible_asset_files: Vec<&FFXIVAssetFiles> = Vec::new();
//
//         for asset_file in &self.asset_files {
//             if asset_file.index_file.data_category.id == asset_path.data_category.id &&
//                 asset_file.index_file.data_repository.id == asset_path.data_repo.id {
//                 possible_asset_files.push(asset_file.clone());
//             }
//         }
//
//         possible_asset_files
//     }
// }
//


pub struct FFXIVAssetFiles {
    pub asset_files: Vec<FFXIVAssetFileGroup>
}

impl FFXIVAssetFiles {
    pub fn new(game_path: &str) -> FFXIVAssetFiles {
        let asset_files: Vec<FFXIVAssetFileGroup> = FFXIVAssetFileGroup::new(&game_path).unwrap();
        FFXIVAssetFiles {
            asset_files
        }
    }

    pub fn get_asset(&self, dat_path: &str) -> Option<FileType> {
        let (dat, index) = self.find_asset(dat_path).unwrap();
        let mut buffer = FFXIVBuffer::from_file_path(&dat.path);
        let header_type = FFXIVAssetParserDatHeaderType::check_at(&mut buffer, index.data_file_offset).ok()?;
        match header_type {
            FFXIVAssetParserDatHeaderType::Texture => Some(FileType::Texture(TextureFile::new(&mut buffer, index.data_file_offset))),
            FFXIVAssetParserDatHeaderType::Standard => Some(FileType::Standard(StandardFile::new(&mut buffer, index.data_file_offset))),
            FFXIVAssetParserDatHeaderType::Model => None,
            FFXIVAssetParserDatHeaderType::Empty => None
        }
    }

    pub fn find_asset(&self, dat_path: &str) -> Option<(FFXIVAssetPathFile, FFXIVAssetParserIndex1Data1Item)> {
        let path_dat = FFXIVAssetPathDat::from_str(dat_path).unwrap();
        let possible_asset_files = self.find_possible_files_from_dot_path(&path_dat);

        for possible_asset_file in possible_asset_files {
            let index_asset = FFXIVAssetParserIndex::from_index1_file(&possible_asset_file.index1_file.path);

            if let Some(item) = index_asset.find(path_dat.index1_hash) {
                let find_this_dat =  format!("dat{}", item.data_file_id);
                let dat_file = possible_asset_file.dat_files.iter().find(|d| d.path_extension == find_this_dat);
                if let Some(dat_file) = dat_file {
                    return Some((dat_file.clone(), item.clone()));
                } else {
                    return None;
                }
            }
        };

        None
    }

    pub fn get_assets_path_from_file(&self, paths_file: &str) -> HashMap<u64, FFXIVAssetPathDat> {
        let mut path_hashes: HashMap<u64, FFXIVAssetPathDat> = HashMap::new();

        let mut thread_handles = vec![];
        let mut thread_count = std::thread::available_parallelism().unwrap().get();
        if thread_count < 2 {
            thread_count = 2;
        }


        let paths_file = fs::read_to_string(paths_file).unwrap();
        let paths: Vec<&str> = paths_file.split("\n").collect();
        let line_count = paths.len();
        if thread_count > line_count {
            thread_count = line_count;
        }

        let path_chunks: Vec<&[&str]> = paths.chunks(line_count / (thread_count - 1)).collect();

        let path_hashes_arc_mutex = Mutex::new(path_hashes);
        let path_hashes_arc = Arc::new(path_hashes_arc_mutex);
        for (thread_index, chunk) in path_chunks.iter().enumerate() {
            let paths_block: Vec<String> = chunk.to_vec().iter().map(|p| (*p).to_owned()).collect();
            let line_count = paths_block.len();
            let path_hashes_clone = Arc::clone(&path_hashes_arc);

            let handle = std::thread::spawn(move || {
                let max_index: f32 = line_count as f32;
                let check_every: f32 = (max_index / 100.0).floor();
                let mut path_hashes: HashMap<u64, FFXIVAssetPathDat> = HashMap::new();

                for (index, path) in paths_block.iter().enumerate() {
                    let parsed_path = FFXIVAssetPathDat::from_str(&path);
                    if let Ok(parsed_path) = parsed_path {
                        path_hashes.insert(parsed_path.index1_hash, parsed_path);

                        let index = index as f32;
                        if index % check_every == 0.0 {
                            let done = (index / max_index) * 100.0;
                            println!("Thread {} Reading path: {}%.\n", thread_index, done);
                        }
                    }
                }
                path_hashes_clone.lock().unwrap().extend(path_hashes);
            });
            thread_handles.push(handle);
        }

        for thread_handle in thread_handles {
            thread_handle.join().unwrap();
        }


        let mut gg = Arc::try_unwrap(path_hashes_arc).unwrap().into_inner().unwrap();

        gg
    }

    pub fn get_index1_assets_locations(&self) -> HashMap<u64, (String, FFXIVAssetParserIndex1Data1Item)> {
        let mut map: HashMap<u64, (String, FFXIVAssetParserIndex1Data1Item)> = HashMap::new();

        for group in &self.asset_files {
            let index1 = FFXIVAssetParserIndex::from_index1_file(&group.index1_file.path);
            for item in index1.data1 {
                let find_this_dat =  format!("dat{}", item.data_file_id);
                let dat_file = group.dat_files.iter().find(|d| d.path_extension == find_this_dat);
                if let Some(dat_file_path) = dat_file {
                    map.insert(item.hash,(dat_file_path.path_str.clone(), item));
                }
            }
        }

        map

    }

    pub fn get_index1_dat_items(&self) -> HashMap<String, Vec<FFXIVAssetParserIndex1Data1Item>> {
        let mut map: HashMap<String, Vec<FFXIVAssetParserIndex1Data1Item>> = HashMap::new();


        for group in &self.asset_files {
            let index1 = FFXIVAssetParserIndex::from_index1_file(&group.index1_file.path);
            for item in index1.data1 {
                let find_this_dat =  format!("dat{}", item.data_file_id);
                let dat_file = group.dat_files.iter().find(|d| d.path_extension == find_this_dat);
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

    pub fn export_all_text(&self, export_path: &str, path_names: &str) {

        let dats = self.get_index1_dat_items();
        let names = self.get_assets_path_from_file(path_names);



        for (dat, items) in dats {
            let max_index: f32 = items.len() as f32;
            let check_every: f32 = (max_index / 100.0).floor();

            let mut buffer = FFXIVBuffer::from_file_path(&dat);
            for (index, item) in items.iter().enumerate() {
                let org_name = names.get(&item.hash);
                let item_name: String;
                if let Some(org_name) = org_name {
                    if org_name.path_extension == "scd" || org_name.path_extension == "tex" || org_name.path_extension == "mdl" {
                        continue;
                    }
                    item_name = org_name.path_name.clone();
                } else {
                    item_name = item.hash.to_string();
                }
                let new_item_path = PathBuf::from(format!("{}/{}", export_path, item_name));

                if !new_item_path.exists() {
                    let header_type = FFXIVAssetParserDatHeaderType::check_at(&mut buffer, item.data_file_offset).unwrap();
                    if let FFXIVAssetParserDatHeaderType::Standard = header_type {
                        let file = StandardFile::new(&mut buffer, item.data_file_offset);
                        let data = file.decompress();
                        fs::write(new_item_path, data).unwrap();
                    }
                }

                let index = index as f32;
                if index % check_every == 0.0 {
                    let done = (index / max_index) * 100.0;
                    println!("Exporting {}: {}%.\n", &dat, done);
                }
            }
        }
    }

    // pub fn export_all_text(&self, paths_file: &str) {
    //     let hash_names = self.get_assets_path_from_file(paths_file);
    //     let hashes = self.get_index1_assets_locations();
    //
    //     let a = hash_names.len();
    //     let b = hashes.len();
    //
    //
    //     //let mut output: String = String::new();
    //
    //     //let mut error_log = File::open("error_log.txt").unwrap();
    //
    //     let max_index: f32 = hashes.len() as f32;
    //     let check_every: f32 = (max_index / 100.0).floor();
    //     for (index, (hash, (data_path, index1data1item))) in hashes.iter().enumerate() {
    //         let path = hash_names.get(&hash);
    //         if let Some(path) = path {
    //
    //             if path.file_extension == "scd" {
    //                 let mut scd_file_path = SaveFilePath::from_index_path(path);
    //
    //                 if scd_file_path.exists {
    //                     let wav_file_path = scd_file_path.as_new("wav");
    //
    //                     if !wav_file_path.exists {
    //                         scd_file_path.decode_to_wav();
    //                     }
    //
    //                     scd_file_path.remove();
    //                 } else {
    //                     let wav_file_path = scd_file_path.as_new("wav");
    //
    //                     if !wav_file_path.exists {
    //                         scd_file_path.decompress_and_write_blocks(data_path, index1data1item.data_file_offset);
    //                         scd_file_path.decode_to_wav();
    //                         scd_file_path.remove();
    //                     }
    //                 }
    //
    //             }
    //         }
    //
    //         let index = index as f32;
    //         if index % check_every == 0.0 {
    //             let done = (index / max_index) * 100.0;
    //             println!("Writing path: {}%.\n", done);
    //         }
    //     }
    // }



    fn find_possible_files_from_dot_path(&self, asset_path: &FFXIVAssetPathDat) -> Vec<&FFXIVAssetFileGroup>{
        let mut possible_asset_files: Vec<&FFXIVAssetFileGroup> = Vec::new();

        for asset_file in &self.asset_files {
            if asset_file.index1_file.data_category.id == asset_path.data_category.id &&
                asset_file.index1_file.data_repository.id == asset_path.data_repo.id {
                possible_asset_files.push(asset_file.clone());
            }
        }

        possible_asset_files
    }

}


pub struct FFXIVAssetFileGroup {
    pub dat_files: Vec<FFXIVAssetPathFile>,
    pub index1_file: FFXIVAssetPathFile,
    pub index2_file: FFXIVAssetPathFile,
}

impl FFXIVAssetFileGroup {

    pub fn new(game_path: &str) -> Result<Vec<FFXIVAssetFileGroup>, String> {
        let mut file_paths: Vec<PathBuf> = Vec::new();
        FFXIVAssetFileGroup::get_files(game_path, &mut file_paths);

        let mut dat_files: Vec<FFXIVAssetPathFile> = Vec::new();
        let mut index_files: Vec<FFXIVAssetPathFile> = Vec::new();
        let mut index2_files: Vec<FFXIVAssetPathFile> = Vec::new();

        for file_path in file_paths {
            let file_metadata = FFXIVAssetPathFile::new(file_path);
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


        let grouped_files = FFXIVAssetFileGroup::group_files(dat_files, index_files, index2_files);

        Ok(grouped_files)
    }

    pub fn group_files(dat_files: Vec<FFXIVAssetPathFile>, index_files: Vec<FFXIVAssetPathFile>, index2_files: Vec<FFXIVAssetPathFile>) -> Vec<FFXIVAssetFileGroup> {
        let mut file_groups: Vec<FFXIVAssetFileGroup> = Vec::new();

        for index_file in index_files {
            let index2_file = index2_files.iter().find(|f| **f == index_file);
            if let Some(index2_file) = index2_file {
                let dat_files: Vec<FFXIVAssetPathFile> = dat_files.iter().filter(|f| **f == index_file).map(|f| f.clone()).collect();
                if dat_files.len() == 0 {
                    continue;
                }


                file_groups.push(FFXIVAssetFileGroup {
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
                FFXIVAssetFileGroup::get_files(path.to_str().unwrap(), output);
            }
        }
    }
}
