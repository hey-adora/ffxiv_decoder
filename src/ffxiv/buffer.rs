use std::mem;
use std::path::Path;
use positioned_io::{RandomAccessFile, ReadAt};

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

pub trait BufferReader {
    fn read_to_buffer_at(&mut self, buffer: &mut [u8], at: u64);
}

pub struct Buffer<T: BufferReader> {
    pub reader: T,
    pub offset: u64
}

impl<T: BufferReader> Buffer<T> {
    read_impl!(u8, from_be_bytes, u8);
    read_impl!(i8, from_be_bytes, i8);

    read_impl!(be_u16, from_be_bytes, u16);
    read_impl!(be_i16, from_be_bytes, i16);
    read_impl!(be_u32, from_be_bytes, u32);
    read_impl!(be_i32, from_be_bytes, i32);
    read_impl!(be_u64, from_be_bytes, u64);
    read_impl!(be_i64, from_be_bytes, i64);
    read_impl!(be_u128, from_be_bytes, u128);
    read_impl!(be_i128, from_be_bytes, i128);

    read_impl!(be_f32, from_be_bytes, f32);
    read_impl!(be_f64, from_be_bytes, f64);

    read_impl!(le_u16, from_le_bytes, u16);
    read_impl!(le_i16, from_le_bytes, i16);
    read_impl!(le_u32, from_le_bytes, u32);
    read_impl!(le_i32, from_le_bytes, i32);
    read_impl!(le_u64, from_le_bytes, u64);
    read_impl!(le_i64, from_le_bytes, i64);
    read_impl!(le_u128, from_le_bytes, u128);
    read_impl!(le_i128, from_le_bytes, i128);

    read_impl!(le_f32, from_le_bytes, f32);
    read_impl!(le_f64, from_le_bytes, f64);

    read_at_impl!(u8_at, from_be_bytes, u8);
    read_at_impl!(i8_at, from_be_bytes, i8);

    read_at_impl!(be_u16_at, from_be_bytes, u16);
    read_at_impl!(be_i16_at, from_be_bytes, i16);
    read_at_impl!(be_u32_at, from_be_bytes, u32);
    read_at_impl!(be_i32_at, from_be_bytes, i32);
    read_at_impl!(be_u64_at, from_be_bytes, u64);
    read_at_impl!(be_i64_at, from_be_bytes, i64);
    read_at_impl!(be_u128_at, from_be_bytes, u128);
    read_at_impl!(be_i128_at, from_be_bytes, i128);

    read_at_impl!(be_f32_at, from_be_bytes, f32);
    read_at_impl!(be_f64_at, from_be_bytes, f64);

    read_at_impl!(le_u16_at, from_le_bytes, u16);
    read_at_impl!(le_i16_at, from_le_bytes, i16);
    read_at_impl!(le_u32_at, from_le_bytes, u32);
    read_at_impl!(le_i32_at, from_le_bytes, i32);
    read_at_impl!(le_u64_at, from_le_bytes, u64);
    read_at_impl!(le_i64_at, from_le_bytes, i64);
    read_at_impl!(le_u128_at, from_le_bytes, u128);
    read_at_impl!(le_i128_at, from_le_bytes, i128);

    read_at_impl!(le_f32_at, from_le_bytes, f32);
    read_at_impl!(le_f64_at, from_le_bytes, f64);

    read_be_or_le_impl!(be_le_u16, u16);
    read_be_or_le_impl!(be_le_i16, i16);
    read_be_or_le_impl!(be_le_u32, u32);
    read_be_or_le_impl!(be_le_i32, i32);
    read_be_or_le_impl!(be_le_u64, u64);
    read_be_or_le_impl!(be_le_i64, i64);
    read_be_or_le_impl!(be_le_i128, u128);
    read_be_or_le_impl!(be_le_u128, i128);

    read_be_or_le_impl!(be_le_f32, f32);
    read_be_or_le_impl!(be_le_f64, f64);

    read_be_or_le_at_impl!(be_le_u16_at, u16);
    read_be_or_le_at_impl!(be_le_i16_at, i16);
    read_be_or_le_at_impl!(be_le_u32_at, u32);
    read_be_or_le_at_impl!(be_le_i32_at, i32);
    read_be_or_le_at_impl!(be_le_u64_at, u64);
    read_be_or_le_at_impl!(be_le_i64_at, i64);
    read_be_or_le_at_impl!(be_le_i128_at, u128);
    read_be_or_le_at_impl!(be_le_u128_at, i128);

    read_be_or_le_at_impl!(be_le_f32_at, f32);
    read_be_or_le_at_impl!(be_le_f64_at, f64);


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

    pub fn read(&mut self, buffer: &mut [u8]) {
        self.reader.read_to_buffer_at(buffer, self.offset);
        self.offset += buffer.len() as u64;
    }

    fn read_at(&mut self, buffer: &mut [u8], at: u64) {
        self.reader.read_to_buffer_at(buffer, at);
    }

    pub fn offset_add(&mut self, offset: u64) -> u64 {
        self.offset += offset;
        self.offset
    }

    pub fn offset_skip(&mut self, offset: u64) {
        self.offset += offset;
    }

    pub fn offset_set(&mut self, offset: u64) -> u64 {
        self.offset = offset;
        self.offset
    }

}
//==================================================================================================

pub struct BufferVec {
    pub buffer_vec: Vec<u8>
}

impl BufferVec {
    pub fn new_vec(vec: Vec<u8>) -> BufferVec {
        BufferVec {
            buffer_vec: vec
        }
    }
}

impl BufferReader for BufferVec {
    fn read_to_buffer_at(&mut self, buffer: &mut [u8], at: u64) {
        buffer.copy_from_slice(&self.buffer_vec[at as usize..at as usize+buffer.len()]);
    }
}

impl Buffer<BufferVec> {
    pub fn from_vec(vec: Vec<u8>) -> Buffer<BufferVec> {
        Buffer {
            reader: BufferVec::new_vec(vec),
            offset: 0
        }
    }
}

//==================================================================================================

pub struct BufferFile {
    pub file_handle: RandomAccessFile
}

impl BufferFile {
    pub fn new_file_from_path<P: AsRef<Path>>(path: P) -> BufferFile {
        let file_handle = RandomAccessFile::open(path).unwrap();
        BufferFile {
            file_handle
        }
    }
}

impl BufferReader for BufferFile {
    fn read_to_buffer_at(&mut self, buffer: &mut [u8], at: u64) {
        self.file_handle.read_at(at, buffer).unwrap();
    }
}

impl Buffer<BufferFile> {
    pub fn from_file_path<P: AsRef<Path>>(path: P) -> Buffer<BufferFile> {
        Buffer {
            reader: BufferFile::new_file_from_path(path),
            offset: 0
        }
    }
}

//==================================================================================================

#[cfg(test)]
mod BufferTests {
    use crate::ffxiv::buffer::Buffer;

    #[test]
    fn reading_u8() {
        let buffer = vec![116, 101, 115, 116, 32, 121, 111];
        let mut buffer_reader = Buffer::from_vec(buffer);
        let n = buffer_reader.u8();
        assert_eq!(n, 116);
    }

    #[test]
    fn reading_u16() {
        let buffer = vec![116, 101, 115, 116, 32, 121, 111];
        let mut buffer_reader = Buffer::from_vec(buffer);
        let n = buffer_reader.be_u16();
        assert_eq!(n, 29797);
    }

    #[test]
    fn reading_string() {
        let buffer = vec![116, 101, 115, 116, 32, 121, 111];
        let mut buffer_reader = Buffer::from_vec(buffer);
        let n = buffer_reader.string(4);
        assert_eq!(n, "test");
    }
}