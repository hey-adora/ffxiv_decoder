#[derive(Debug)]
pub struct Platform {
    pub name: String,
    pub id: u32
}

impl Platform {
    pub fn from_number(n: u32) -> Result<Platform, String> {
        let name = match n {
            0 => "win32",
            1 => "ps3",
            2 => "ps4",
            _ => {
                return Err(format!("Platform id out of range 0:2, got: {}", n));
            }
        };

        Ok(Platform {
            name: String::from(name),
            id: n,
        })
    }

    pub fn from_str(platform: &str) -> Result<Platform, String> {
        let id = match platform {
            "win32" => 0u32,
            "ps3" => 1u32,
            "ps4" => 2u32,
            _ => {
                return Err(format!("Platform '{}' not found.", platform));
            }
        };

        Ok(Platform {
            name: String::from(platform),
            id,
        })
    }
}
