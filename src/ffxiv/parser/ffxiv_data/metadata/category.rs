#[derive(Debug)]
pub struct Category {
    pub name: String,
    pub id: u32
}

impl Category {
    pub fn from_hex_str(cat_hex_str: &str) -> Result<Category, String> {
        let expansion_id: u32 = u32::from_str_radix(cat_hex_str, 16).or(Err(format!("Failed to convert '{}' to number.", cat_hex_str)))?;
        Category::from_number(expansion_id)
    }

    pub fn from_str(cat: &str) -> Result<Category, String> {
        let id = match cat {
            "common" => 0x00,
            "bgcommon" => 0x01,
            "bg" => 0x02,
            "cut" => 0x03,
            "chara" => 0x04,
            "shader" => 0x05,
            "ui" => 0x06,
            "sound" => 0x07,
            "vfx" => 0x08,
            "ui_script" => 0x09,
            "exd" => 0x0A,
            "game_script" => 0x0B,
            "music" => 0x0C,
            "sqpack_test" => 0x12,
            "debug" => 0x13,
            _ => {
                return Err(format!("Category '{}' not found", cat));
            }
        };

        Ok(
            Category {
                name: String::from(cat),
                id
            }
        )


    }

    pub fn from_number(cat: u32) -> Result<Category, String> {
        let name = match cat {
            0x00 => "common",
            0x01 => "bgcommon",
            0x02 => "bg",
            0x03 => "cut",
            0x04 => "chara",
            0x05 => "shader",
            0x06 => "ui",
            0x07 => "sound",
            0x08 => "vfx",
            0x09 => "ui_script",
            0x0A => "exd",
            0x0B => "game_script",
            0x0C => "music",
            0x12 => "sqpack_test",
            0x13 => "debug",
            _ => {
                return Err(format!("Category '{}' not found", cat));
            }
        };

        Ok(
            Category {
                name: String::from(name),
                id: cat
            }
        )


    }
}