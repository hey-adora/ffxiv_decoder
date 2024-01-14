
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::mem;

macro_rules! read_impl {
    ($name: tt, $from_fn: tt, $t: ty) => {
        pub fn $name(&mut self) -> $t {
            let output = <$t>::$from_fn(self.slice_at(self.offset, mem::size_of::<$t>()).try_into().unwrap());
            self.offset += mem::size_of::<$t>();
            output
        }
    }
}

macro_rules! read_at_impl {
    ($name: tt, $from_fn: tt, $t: ty) => {
        pub fn $name(&mut self, at: usize) -> $t {
            <$t>::$from_fn(self.slice_at(at, mem::size_of::<$t>()).try_into().unwrap())
        }
    }
}

macro_rules! read_be_or_le_impl {
    ($name: tt, $t: ty) => {
        pub fn $name(&mut self, is_be: bool) -> $t {
            let output;
            if is_be {
                output = <$t>::from_be_bytes(self.slice_at(self.offset, mem::size_of::<$t>()).try_into().unwrap());
            } else {
                output = <$t>::from_le_bytes(self.slice_at(self.offset, mem::size_of::<$t>()).try_into().unwrap());
            }
            self.offset += mem::size_of::<$t>();
            output
        }
    }
}

macro_rules! read_be_or_le_at_impl {
    ($name: tt, $t: ty) => {
        pub fn $name(&mut self, at: usize, is_be: bool) -> $t {
            if is_be {
                <$t>::from_be_bytes(self.slice_at(at, mem::size_of::<$t>()).try_into().unwrap())
            } else {
                <$t>::from_le_bytes(self.slice_at(at, mem::size_of::<$t>()).try_into().unwrap())
            }
        }
    }
}


#[derive(Clone)]
pub struct BufferVec {
    pub bytes: Vec<u8>,
    pub offset: usize
}

impl BufferVec {
    pub fn new(bytes: Vec<u8>) -> BufferVec {
        BufferVec {
            bytes,
            offset: 0
        }
    }

    pub fn from_file(file_path: &str) -> BufferVec {
        let file = File::open(file_path).expect("Failed to open file.");
        let mut reader = BufReader::new(file);
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).expect("Failed to read file.");
        BufferVec {
            bytes,
            offset: 0
        }
    }


    read_impl!(be_u16, from_be_bytes, u16);
    read_impl!(be_i16, from_be_bytes, i16);
    read_impl!(be_u32, from_be_bytes, u32);
    read_impl!(be_i32, from_be_bytes, i32);
    read_impl!(be_u64, from_be_bytes, u64);
    read_impl!(be_i64, from_be_bytes, i64);
    read_impl!(be_u128, from_be_bytes, u128);
    read_impl!(be_i128, from_be_bytes, i128);

    read_impl!(le_u16, from_le_bytes, u16);
    read_impl!(le_i16, from_le_bytes, i16);
    read_impl!(le_u32, from_le_bytes, u32);
    read_impl!(le_i32, from_le_bytes, i32);
    read_impl!(le_u64, from_le_bytes, u64);
    read_impl!(le_i64, from_le_bytes, i64);
    read_impl!(le_u128, from_le_bytes, u128);
    read_impl!(le_i128, from_le_bytes, i128);

    read_at_impl!(be_u16_at, from_be_bytes, u16);
    read_at_impl!(be_i16_at, from_be_bytes, i16);
    read_at_impl!(be_u32_at, from_be_bytes, u32);
    read_at_impl!(be_i32_at, from_be_bytes, i32);
    read_at_impl!(be_u64_at, from_be_bytes, u64);
    read_at_impl!(be_i64_at, from_be_bytes, i64);
    read_at_impl!(be_u128_at, from_be_bytes, u128);
    read_at_impl!(be_i128_at, from_be_bytes, i128);

    read_at_impl!(le_u16_at, from_le_bytes, u16);
    read_at_impl!(le_i16_at, from_le_bytes, i16);
    read_at_impl!(le_u32_at, from_le_bytes, u32);
    read_at_impl!(le_i32_at, from_le_bytes, i32);
    read_at_impl!(le_u64_at, from_le_bytes, u64);
    read_at_impl!(le_i64_at, from_le_bytes, i64);
    read_at_impl!(le_u128_at, from_le_bytes, u128);
    read_at_impl!(le_i128_at, from_le_bytes, i128);


    read_be_or_le_impl!(be_le_u16, u16);
    read_be_or_le_impl!(be_le_i16, i16);
    read_be_or_le_impl!(be_le_u32, u32);
    read_be_or_le_impl!(be_le_i32, i32);
    read_be_or_le_impl!(be_le_u64, u64);
    read_be_or_le_impl!(be_le_i64, i64);
    read_be_or_le_impl!(be_le_i128, u128);
    read_be_or_le_impl!(be_le_u128, i128);

    read_be_or_le_at_impl!(be_le_u16_at, u16);
    read_be_or_le_at_impl!(be_le_i16_at, i16);
    read_be_or_le_at_impl!(be_le_u32_at, u32);
    read_be_or_le_at_impl!(be_le_i32_at, i32);
    read_be_or_le_at_impl!(be_le_u64_at, u64);
    read_be_or_le_at_impl!(be_le_i64_at, i64);
    read_be_or_le_at_impl!(be_le_i128_at, u128);
    read_be_or_le_at_impl!(be_le_u128_at, i128);

    pub fn string(&mut self, size: usize) -> String {
        let output = String::from_utf8(self.slice_at(self.offset, size).to_owned()).unwrap();
        self.offset += size;
        output
    }

    pub fn string_at(&mut self, start: usize, size: usize) -> String {
        String::from_utf8(self.slice_at(start, size).to_owned()).unwrap()
    }

    pub fn u8(&mut self) -> u8 {
        let output = self.bytes[self.offset];
        self.offset += 1;
        output
    }

    pub fn u8_at(&mut self, at: usize) -> u8 {
        self.bytes[at]
    }

    pub fn i8(&mut self) -> i8 {
        let output = self.bytes[self.offset] as i8;
        self.offset += 1;
        output
    }

    pub fn i8_at(&mut self, at: usize) -> i8 {
        self.bytes[at] as i8
    }

    pub fn slice_at(&mut self, start: usize, size: usize) -> &[u8] {
        &self.bytes[start..start + size]
    }

    pub fn offset_set(&mut self, offset: usize) -> usize {
        self.offset = offset;
        self.offset
    }

    pub fn offset_add(&mut self, offset: usize) -> usize {
        self.offset += offset;
        self.offset
    }

    pub fn offset_skip(&mut self, offset: usize) {
        self.offset += offset;
    }

}