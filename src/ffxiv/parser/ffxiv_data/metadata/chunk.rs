

#[derive(Debug)]
pub struct Chunk {
    pub hex: String,
    pub id: u32,
}

impl Chunk {
    pub fn from_hex_str(chunk_hex_str: &str) -> Result<Chunk, String> {
        let chunk_number: u32 = u32::from_str_radix(chunk_hex_str, 16).or(Err(format!("Failed to parse chunk '{}' to a number.", chunk_hex_str)))?;
        Chunk::from_u32(chunk_number)
    }

    pub fn from_u32(chunk_number: u32) -> Result<Chunk, String> {
        if chunk_number > 255 {
            return Err(format!("Chunk '{}' is out of range 0:255", chunk_number));
        }
        let chunk_name: String = format!("{:02x}", 2);
        Ok(Chunk {
            hex: chunk_name,
            id: chunk_number,
        })
    }

}