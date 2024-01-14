use positioned_io::{RandomAccessFile, ReadAt, ReadBytesAtExt};
use std::mem;

macro_rules! read_impl {
    ($name: ident, $from_fn: ident, $t: ty) => {
        pub fn $name(&mut self) -> $t {
            let mut buffer: [u8; mem::size_of::<$t>()] = [0; mem::size_of::<$t>()];
            self.read(&mut buffer);
            <$t>::$from_fn(buffer)
        }
    }
}

macro_rules! read_at_impl {
    ($name: ident, $from_fn: ident, $t: ty) => {
        pub fn $name(&mut self, at: u64) -> $t {
            let mut buffer: [u8; mem::size_of::<$t>()] = [0; mem::size_of::<$t>()];
            self.read_at(&mut buffer, at);
            <$t>::$from_fn(buffer)
        }
    }
}

macro_rules! read_be_or_le_impl {
    ($name: ident, $t: ty) => {
        pub fn $name(&mut self, is_be: bool) -> $t {
            let mut buffer: [u8; mem::size_of::<$t>()] = [0; mem::size_of::<$t>()];
            self.read(&mut buffer);
            if is_be {
                <$t>::from_be_bytes(buffer)
            } else {
                <$t>::from_le_bytes(buffer)
            }
        }
    }
}

macro_rules! read_be_or_le_at_impl {
    ($name: ident, $t: ty) => {
        pub fn $name(&mut self, at: u64, is_be: bool) -> $t {
            let mut buffer: [u8; mem::size_of::<$t>()] = [0; mem::size_of::<$t>()];
            self.read_at(&mut buffer, at);
            if is_be {
                <$t>::from_be_bytes(buffer)
            } else {
                <$t>::from_le_bytes(buffer)
            }
        }
    }
}

pub struct BufferFile {
    file_handle: RandomAccessFile,
    offset: u64,
}

impl BufferFile {
    pub fn from_file_handle(file_handle: RandomAccessFile) -> BufferFile {
        BufferFile
        {
            file_handle,
            offset: 0,
        }
    }

    pub fn from_file_path(file_path: &str) -> BufferFile {
        let file_handle = RandomAccessFile::open(&file_path).unwrap();
        BufferFile {
            file_handle,
            offset: 0,
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
    read_be_or_le_impl!(be_le_u128, u128);
    read_be_or_le_impl!(be_le_i128, i128);

    read_be_or_le_at_impl!(be_le_u16_at, u16);
    read_be_or_le_at_impl!(be_le_i16_at, i16);
    read_be_or_le_at_impl!(be_le_u32_at, u32);
    read_be_or_le_at_impl!(be_le_i32_at, i32);
    read_be_or_le_at_impl!(be_le_u64_at, u64);
    read_be_or_le_at_impl!(be_le_i64_at, i64);
    read_be_or_le_at_impl!(be_le_u128_at, u128);
    read_be_or_le_at_impl!(be_le_i128_at, i128);

    pub fn string(&mut self, size: usize) -> String {
        let mut buffer: Vec<u8> = vec![0; size];
        self.read(&mut buffer);
        String::from_utf8(buffer).unwrap()
    }

    pub fn string_at(&mut self, at: u64, size: usize) -> String {
        let mut buffer: Vec<u8> = vec![0; size];
        self.read_at(&mut buffer, at);
        String::from_utf8(buffer).unwrap()
    }

    pub fn vec(&mut self, size: usize) -> Vec<u8> {
        let mut buffer: Vec<u8> = vec![0; size];
        self.read(&mut buffer);
        buffer
    }

    pub fn vec_at(&mut self, at: u64, size: usize) -> Vec<u8> {
        let mut buffer: Vec<u8> = vec![0; size];
        self.read_at(&mut buffer, at);
        buffer
    }

    pub fn offset_set(&mut self, offset: u64) -> u64 {
        self.offset = offset;
        self.offset
    }

    pub fn offset_add(&mut self, offset: u64) -> u64 {
        self.offset += offset;
        self.offset
    }

    pub fn u8(&mut self) -> u8 {
        let byte = self.file_handle.read_u8_at(self.offset).unwrap();
        self.offset += 1;
        byte
    }

    pub fn u8_at(&mut self, at: u64) -> u8 {
        self.file_handle.read_u8_at(at).unwrap()
    }

    pub fn i8(&mut self) -> i8 {
        let byte = self.file_handle.read_i8_at(self.offset).unwrap();
        self.offset += 1;
        byte
    }

    pub fn i8_at(&mut self, at: u64) -> i8 {
        self.file_handle.read_i8_at(at).unwrap()
    }

    pub fn read(&mut self, buffer: &mut [u8]) {
        self.file_handle.read_at(self.offset, buffer).unwrap();
        self.offset += buffer.len() as u64;
    }

    pub fn read_at(&mut self, buffer: &mut [u8], at: u64) {
        self.file_handle.read_at(at, buffer).unwrap();
    }
}