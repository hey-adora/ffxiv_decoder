

#[derive(Debug, Clone)]
pub struct AssetFileChunk {
    pub hex: String,
    pub id: u32,
}

impl AssetFileChunk {
    pub fn from_hex_str(chunk_hex_str: &str) -> Result<AssetFileChunk, String> {
        let chunk_number: u32 = u32::from_str_radix(chunk_hex_str, 16).or(Err(format!("Failed to parse chunk '{}' to a number.", chunk_hex_str)))?;
        if chunk_number > 255 {
            return Err(format!("Chunk '{}' is out of range 0:255", chunk_number));
        }
        Ok(AssetFileChunk {
            hex: String::from(chunk_hex_str),
            id: chunk_number,
        })
    }

    pub fn from_u32(chunk_number: u32) -> Result<AssetFileChunk, String> {
        if chunk_number > 255 {
            return Err(format!("Chunk '{}' is out of range 0:255", chunk_number));
        }
        let chunk_name: String = format!("{:02x}", chunk_number);
        Ok(AssetFileChunk {
            hex: chunk_name,
            id: chunk_number,
        })
    }

}