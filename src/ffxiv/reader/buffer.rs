pub mod visualizer;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};

#[derive(Clone)]
pub struct Buffer {
    pub bytes: Vec<u8>,
    pub read_history: HashMap<usize, usize>,
}

impl Buffer {
    pub fn new(bytes: Vec<u8>) -> Buffer {
        Buffer {
            bytes,
            read_history: HashMap::new(),
        }
    }

    pub fn from_file(file_path: &str) -> Buffer {
        let file = File::open(file_path).expect("Failed to open file.");
        let mut reader = BufReader::new(file);
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).expect("Failed to read file.");
        Buffer {
            bytes,
            read_history: HashMap::new(),
        }
    }


    pub fn string(&mut self, start: usize, size: usize) -> String {
        String::from_utf8(self.vec(start, size).to_owned()).unwrap()
    }

    pub fn u8(&mut self, start: usize) -> u8 {
        u8::from_ne_bytes(self.vec(start, 1).try_into().unwrap())
    }

    pub fn i8(&mut self, start: usize) -> i8 {
        i8::from_ne_bytes(self.vec(start, 1).try_into().unwrap())
    }

    pub fn u16(&mut self, start: usize) -> u16 {
        u16::from_ne_bytes(self.vec(start, 2).try_into().unwrap())
    }

    pub fn i16(&mut self, start: usize) -> i16 {
        i16::from_ne_bytes(self.vec(start, 2).try_into().unwrap())
    }

    pub fn i32(&mut self, start: usize) -> i32 {
        i32::from_ne_bytes(self.vec(start, 4).try_into().unwrap())
    }

    pub fn u32(&mut self, start: usize) -> u32 {
        u32::from_ne_bytes(self.vec(start, 4).try_into().unwrap())
    }

    pub fn u64(&mut self, start: usize) -> u64 {
        u64::from_ne_bytes(self.vec(start, 8).try_into().unwrap())
    }

    pub fn i64(&mut self, start: usize) -> i64 {
        i64::from_ne_bytes(self.vec(start, 8).try_into().unwrap())
    }

    pub fn u128(&mut self, start: usize) -> u128 {
        u128::from_ne_bytes(self.vec(start, 16).try_into().unwrap())
    }

    pub fn i128(&mut self, start: usize) -> i128 {
        i128::from_ne_bytes(self.vec(start, 16).try_into().unwrap())
    }

    pub fn vec(&mut self, start: usize, size: usize) -> &[u8] {
        self.read_history.insert(start, size);
        &self.bytes[start..start + size]
    }
}