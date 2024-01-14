
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use positioned_io::{RandomAccessFile, ReadAt};

pub struct BufferWithRandomAccess {
    file_handle: RandomAccessFile,
    offset: usize
}

impl BufferWithRandomAccess {
    pub fn new(file_handle: RandomAccessFile) -> BufferWithRandomAccess {
        BufferWithRandomAccess
        {
            file_handle,
            offset: 0,
        }
    }

    pub fn from_file(file_path: &str) -> BufferWithRandomAccess {
        let file_handle = RandomAccessFile::open(&file_path).unwrap();
        BufferWithRandomAccess {
            file_handle,
            offset: 0
        }
    }

    pub fn string(&mut self, size: usize) -> String {
        let mut buffer: Vec<u8> = vec![0; size];
        self.read(&mut buffer);
        String::from_utf8(buffer).unwrap()
    }

    pub fn u8(&mut self) -> u8 {
        let mut buffer: [u8; 1] = [0; 1];
        self.read(&mut buffer);
        u8::from_ne_bytes(buffer)
    }

    pub fn i8(&mut self) -> i8 {
        let mut buffer: [u8; 1] = [0; 1];
        self.read(&mut buffer);
        i8::from_ne_bytes(buffer)
    }

    pub fn u16(&mut self) -> u16 {
        let mut buffer: [u8; 2] = [0; 2];
        self.read(&mut buffer);
        u16::from_ne_bytes(buffer)
    }

    pub fn i16(&mut self) -> i16 {
        let mut buffer: [u8; 2] = [0; 2];
        self.read(&mut buffer);
        i16::from_ne_bytes(buffer)
    }

    pub fn i32(&mut self) -> i32 {
        let mut buffer: [u8; 4] = [0; 4];
        self.read(&mut buffer);
        i32::from_ne_bytes(buffer)
    }

    pub fn u32(&mut self) -> u32 {
        let mut buffer: [u8; 4] = [0; 4];
        self.read(&mut buffer);
        u32::from_ne_bytes(buffer)
    }

    pub fn u64(&mut self) -> u64 {
        let mut buffer: [u8; 8] = [0; 8];
        self.read(&mut buffer);
        u64::from_ne_bytes(buffer)
    }

    pub fn i64(&mut self) -> i64 {
        let mut buffer: [u8; 8] = [0; 8];
        self.read(&mut buffer);
        i64::from_ne_bytes(buffer)
    }

    pub fn u128(&mut self) -> u128 {
        let mut buffer: [u8; 16] = [0; 16];
        self.read(&mut buffer);
        u128::from_ne_bytes(buffer)
    }

    pub fn i128(&mut self) -> i128 {
        let mut buffer: [u8; 16] = [0; 16];
        self.read(&mut buffer);
        i128::from_ne_bytes(buffer)
    }

    pub fn string_at(&mut self, at: usize , size: usize) -> String {
        let mut buffer: Vec<u8> = vec![0; size];
        self.read_at(at, &mut buffer);
        String::from_utf8(buffer).unwrap()
    }

    pub fn u8_at(&mut self, at: usize) -> u8 {
        let mut buffer: [u8; 1] = [0; 1];
        self.read_at(at, &mut buffer);
        u8::from_ne_bytes(buffer)
    }

    pub fn i8_at(&mut self, at: usize) -> i8 {
        let mut buffer: [u8; 1] = [0; 1];
        self.read_at(at, &mut buffer);
        i8::from_ne_bytes(buffer)
    }

    pub fn u16_at(&mut self, at: usize) -> u16 {
        let mut buffer: [u8; 2] = [0; 2];
        self.read_at(at, &mut buffer);
        u16::from_ne_bytes(buffer)
    }

    pub fn i16_at(&mut self, at: usize) -> i16 {
        let mut buffer: [u8; 2] = [0; 2];
        self.read_at(at, &mut buffer);
        i16::from_ne_bytes(buffer)
    }

    pub fn i32_at(&mut self, at: usize) -> i32 {
        let mut buffer: [u8; 4] = [0; 4];
        self.read_at(at, &mut buffer);
        i32::from_ne_bytes(buffer)
    }

    pub fn u32_at(&mut self, at: usize) -> u32 {
        let mut buffer: [u8; 4] = [0; 4];
        self.read_at(at, &mut buffer);
        u32::from_ne_bytes(buffer)
    }

    pub fn u64_at(&mut self, at: usize) -> u64 {
        let mut buffer: [u8; 8] = [0; 8];
        self.read_at(at, &mut buffer);
        u64::from_ne_bytes(buffer)
    }

    pub fn i64_at(&mut self, at: usize) -> i64 {
        let mut buffer: [u8; 8] = [0; 8];
        self.read_at(at, &mut buffer);
        i64::from_ne_bytes(buffer)
    }

    pub fn u128_at(&mut self, at: usize) -> u128 {
        let mut buffer: [u8; 16] = [0; 16];
        self.read_at(at, &mut buffer);
        u128::from_ne_bytes(buffer)
    }

    pub fn i128_at(&mut self, at: usize) -> i128 {
        let mut buffer: [u8; 16] = [0; 16];
        self.read_at(at, &mut buffer);
        i128::from_ne_bytes(buffer)
    }
    
    pub fn vec(&mut self, size: usize) -> Vec<u8> {
        let mut buffer: Vec<u8> = vec![0; size];
        self.read( &mut buffer);
        buffer
    }

    pub fn vec_at(&mut self, at: usize, size: usize) -> Vec<u8> {
        let mut buffer: Vec<u8> = vec![0; size];
        self.read_at( at, &mut buffer);
        buffer
    }

    pub fn offset_set(&mut self, offset: usize) -> usize {
        self.offset = offset;
        self.offset
    }

    pub fn offset_add(&mut self, offset: usize) -> usize {
        self.offset += offset;
        self.offset
    }

    pub fn read(&mut self, buffer: &mut [u8]) -> usize {
        let read_bytes = self.file_handle.read_at(self.offset as u64, buffer).unwrap();
        self.offset += buffer.len();
        read_bytes
    }

    pub fn read_at(&mut self, at: usize ,buffer: &mut [u8]) -> usize {
        let read_bytes = self.file_handle.read_at(at as u64, buffer).unwrap();
        read_bytes
    }
}