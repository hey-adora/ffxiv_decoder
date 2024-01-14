pub mod file_name;
pub mod platform;
pub mod category;
pub mod repository;
pub mod index_path;
pub mod chunk;

use std::path::PathBuf;
use regex::bytes::Regex;
use crate::ffxiv::parser::ffxiv_data::metadata::category::Category;
use crate::ffxiv::parser::ffxiv_data::metadata::chunk::Chunk;
use crate::ffxiv::parser::ffxiv_data::metadata::platform::Platform;
use crate::ffxiv::parser::ffxiv_data::metadata::repository::Repository;



