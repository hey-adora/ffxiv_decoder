use positioned_io::{RandomAccessFile, ReadAt};

pub struct BufferFile {
    file_handle: RandomAccessFile,
    offset: usize,
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


    pub fn be_u8(&mut self) -> u8 {
        let mut buffer: [u8; 1] = [0; 1];
        self.read(&mut buffer);
        u8::from_be_bytes(buffer)
    }

    pub fn be_i8(&mut self) -> i8 {
        let mut buffer: [u8; 1] = [0; 1];
        self.read(&mut buffer);
        i8::from_be_bytes(buffer)
    }

    pub fn be_u16(&mut self) -> u16 {
        let mut buffer: [u8; 2] = [0; 2];
        self.read(&mut buffer);
        u16::from_be_bytes(buffer)
    }

    pub fn be_i16(&mut self) -> i16 {
        let mut buffer: [u8; 2] = [0; 2];
        self.read(&mut buffer);
        i16::from_be_bytes(buffer)
    }

    pub fn be_i32(&mut self) -> i32 {
        let mut buffer: [u8; 4] = [0; 4];
        self.read(&mut buffer);
        i32::from_be_bytes(buffer)
    }

    pub fn be_u32(&mut self) -> u32 {
        let mut buffer: [u8; 4] = [0; 4];
        self.read(&mut buffer);
        u32::from_be_bytes(buffer)
    }

    pub fn be_u64(&mut self) -> u64 {
        let mut buffer: [u8; 8] = [0; 8];
        self.read(&mut buffer);
        u64::from_be_bytes(buffer)
    }

    pub fn be_i64(&mut self) -> i64 {
        let mut buffer: [u8; 8] = [0; 8];
        self.read(&mut buffer);
        i64::from_be_bytes(buffer)
    }

    pub fn be_u128(&mut self) -> u128 {
        let mut buffer: [u8; 16] = [0; 16];
        self.read(&mut buffer);
        u128::from_be_bytes(buffer)
    }

    pub fn be_i128(&mut self) -> i128 {
        let mut buffer: [u8; 16] = [0; 16];
        self.read(&mut buffer);
        i128::from_be_bytes(buffer)
    }

    pub fn be_u8_at(&mut self, at: usize) -> u8 {
        let mut buffer: [u8; 1] = [0; 1];
        self.read_at(at, &mut buffer);
        u8::from_be_bytes(buffer)
    }

    pub fn be_i8_at(&mut self, at: usize) -> i8 {
        let mut buffer: [u8; 1] = [0; 1];
        self.read_at(at, &mut buffer);
        i8::from_be_bytes(buffer)
    }

    pub fn be_u16_at(&mut self, at: usize) -> u16 {
        let mut buffer: [u8; 2] = [0; 2];
        self.read_at(at, &mut buffer);
        u16::from_be_bytes(buffer)
    }

    pub fn be_i16_at(&mut self, at: usize) -> i16 {
        let mut buffer: [u8; 2] = [0; 2];
        self.read_at(at, &mut buffer);
        i16::from_be_bytes(buffer)
    }

    pub fn be_i32_at(&mut self, at: usize) -> i32 {
        let mut buffer: [u8; 4] = [0; 4];
        self.read_at(at, &mut buffer);
        i32::from_be_bytes(buffer)
    }

    pub fn be_u32_at(&mut self, at: usize) -> u32 {
        let mut buffer: [u8; 4] = [0; 4];
        self.read_at(at, &mut buffer);
        u32::from_be_bytes(buffer)
    }

    pub fn be_u64_at(&mut self, at: usize) -> u64 {
        let mut buffer: [u8; 8] = [0; 8];
        self.read_at(at, &mut buffer);
        u64::from_be_bytes(buffer)
    }

    pub fn be_i64_at(&mut self, at: usize) -> i64 {
        let mut buffer: [u8; 8] = [0; 8];
        self.read_at(at, &mut buffer);
        i64::from_be_bytes(buffer)
    }

    pub fn be_u128_at(&mut self, at: usize) -> u128 {
        let mut buffer: [u8; 16] = [0; 16];
        self.read_at(at, &mut buffer);
        u128::from_be_bytes(buffer)
    }

    pub fn be_i128_at(&mut self, at: usize) -> i128 {
        let mut buffer: [u8; 16] = [0; 16];
        self.read_at(at, &mut buffer);
        i128::from_be_bytes(buffer)
    }

    pub fn le_u8(&mut self) -> u8 {
        let mut buffer: [u8; 1] = [0; 1];
        self.read(&mut buffer);
        u8::from_le_bytes(buffer)
    }

    pub fn le_i8(&mut self) -> i8 {
        let mut buffer: [u8; 1] = [0; 1];
        self.read(&mut buffer);
        i8::from_le_bytes(buffer)
    }

    pub fn le_u16(&mut self) -> u16 {
        let mut buffer: [u8; 2] = [0; 2];
        self.read(&mut buffer);
        u16::from_le_bytes(buffer)
    }

    pub fn le_i16(&mut self) -> i16 {
        let mut buffer: [u8; 2] = [0; 2];
        self.read(&mut buffer);
        i16::from_le_bytes(buffer)
    }

    pub fn le_i32(&mut self) -> i32 {
        let mut buffer: [u8; 4] = [0; 4];
        self.read(&mut buffer);
        i32::from_le_bytes(buffer)
    }

    pub fn le_u32(&mut self) -> u32 {
        let mut buffer: [u8; 4] = [0; 4];
        self.read(&mut buffer);
        u32::from_le_bytes(buffer)
    }

    pub fn le_u64(&mut self) -> u64 {
        let mut buffer: [u8; 8] = [0; 8];
        self.read(&mut buffer);
        u64::from_le_bytes(buffer)
    }

    pub fn le_i64(&mut self) -> i64 {
        let mut buffer: [u8; 8] = [0; 8];
        self.read(&mut buffer);
        i64::from_le_bytes(buffer)
    }

    pub fn le_u128(&mut self) -> u128 {
        let mut buffer: [u8; 16] = [0; 16];
        self.read(&mut buffer);
        u128::from_le_bytes(buffer)
    }

    pub fn le_i128(&mut self) -> i128 {
        let mut buffer: [u8; 16] = [0; 16];
        self.read(&mut buffer);
        i128::from_le_bytes(buffer)
    }

    pub fn le_u8_at(&mut self, at: usize) -> u8 {
        let mut buffer: [u8; 1] = [0; 1];
        self.read_at(at, &mut buffer);
        u8::from_le_bytes(buffer)
    }

    pub fn le_i8_at(&mut self, at: usize) -> i8 {
        let mut buffer: [u8; 1] = [0; 1];
        self.read_at(at, &mut buffer);
        i8::from_le_bytes(buffer)
    }

    pub fn le_u16_at(&mut self, at: usize) -> u16 {
        let mut buffer: [u8; 2] = [0; 2];
        self.read_at(at, &mut buffer);
        u16::from_le_bytes(buffer)
    }

    pub fn le_i16_at(&mut self, at: usize) -> i16 {
        let mut buffer: [u8; 2] = [0; 2];
        self.read_at(at, &mut buffer);
        i16::from_le_bytes(buffer)
    }

    pub fn le_i32_at(&mut self, at: usize) -> i32 {
        let mut buffer: [u8; 4] = [0; 4];
        self.read_at(at, &mut buffer);
        i32::from_le_bytes(buffer)
    }

    pub fn le_u32_at(&mut self, at: usize) -> u32 {
        let mut buffer: [u8; 4] = [0; 4];
        self.read_at(at, &mut buffer);
        u32::from_le_bytes(buffer)
    }

    pub fn le_u64_at(&mut self, at: usize) -> u64 {
        let mut buffer: [u8; 8] = [0; 8];
        self.read_at(at, &mut buffer);
        u64::from_le_bytes(buffer)
    }

    pub fn le_i64_at(&mut self, at: usize) -> i64 {
        let mut buffer: [u8; 8] = [0; 8];
        self.read_at(at, &mut buffer);
        i64::from_le_bytes(buffer)
    }

    pub fn le_u128_at(&mut self, at: usize) -> u128 {
        let mut buffer: [u8; 16] = [0; 16];
        self.read_at(at, &mut buffer);
        u128::from_le_bytes(buffer)
    }

    pub fn le_i128_at(&mut self, at: usize) -> i128 {
        let mut buffer: [u8; 16] = [0; 16];
        self.read_at(at, &mut buffer);
        i128::from_le_bytes(buffer)
    }

    pub fn string(&mut self, size: usize) -> String {
        let mut buffer: Vec<u8> = vec![0; size];
        self.read(&mut buffer);
        String::from_utf8(buffer).unwrap()
    }

    pub fn string_at(&mut self, at: usize, size: usize) -> String {
        let mut buffer: Vec<u8> = vec![0; size];
        self.read_at(at, &mut buffer);
        String::from_utf8(buffer).unwrap()
    }

    pub fn vec(&mut self, size: usize) -> Vec<u8> {
        let mut buffer: Vec<u8> = vec![0; size];
        self.read(&mut buffer);
        buffer
    }

    pub fn vec_at(&mut self, at: usize, size: usize) -> Vec<u8> {
        let mut buffer: Vec<u8> = vec![0; size];
        self.read_at(at, &mut buffer);
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

    pub fn read_at(&mut self, at: usize, buffer: &mut [u8]) -> usize {
        let read_bytes = self.file_handle.read_at(at as u64, buffer).unwrap();
        read_bytes
    }
}