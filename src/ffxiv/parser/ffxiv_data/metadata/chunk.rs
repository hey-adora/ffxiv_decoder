

#[derive(Debug)]
pub struct Chunk {
    pub name: String,
    pub id: u32,
}

impl Chunk {
    pub fn from_str(chunk_str: &str) -> Result<Chunk, String> {
        let chunk_number: u32 = chunk_str.parse().or(Err(format!("Failed to parse chunk '{}' to a number.", chunk_str)))?;
        if chunk_number > 255 {
            return Err(format!("Chunk '{}' is out of range 0:255", chunk_number));
        }
        let chunk_name: String = format!("{:02x}", 2);
        Ok(Chunk {
            name: chunk_name,
            id: chunk_number,
        })
    }

    pub fn from_u32(chunk_number: u32) -> Result<Chunk, String> {
        if chunk_number > 255 {
            return Err(format!("Chunk '{}' is out of range 0:255", chunk_number));
        }
        let chunk_name: String = format!("{:02x}", 2);
        Ok(Chunk {
            name: chunk_name,
            id: chunk_number,
        })
    }

}