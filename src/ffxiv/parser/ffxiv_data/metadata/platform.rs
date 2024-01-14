#[derive(Debug)]
pub struct Platform {
    pub name: String,
    pub id: u32
}

impl Platform {
    pub fn from_u32(n: u32) -> Result<Platform, String> {
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

    pub fn from_hex_str(platform_hex_str: &str) -> Result<Platform, String> {
        let expansion_id: u32 = u32::from_str_radix(platform_hex_str, 16).or(Err(format!("Failed to convert '{}' to number.", platform_hex_str)))?;
        Platform::from_u32(expansion_id)
    }

    pub fn from_str_contains(string: &str) -> Result<Platform, String> {
        let mut name: String = String::new();
        let mut id = 0;
        if string.contains("win32") {
            name = String::from("win32");
        } else if string.contains("ps3") {
            name = String::from("ps3");
            id = 3;
        } else if string.contains("ps4") {
            name = String::from("ps4");
            id = 4;
        } else {
            return Err(String::from("String doesn't contain win32, ps3 or ps4"));
        }

        Ok(
            Platform {
                name,
                id
            }
        )
    }
}
