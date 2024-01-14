use std::fmt::{Display, Formatter};
use thiserror::Error;
use regex::Regex as Reg;

#[derive(Debug, Clone)]
pub struct Category {
    pub name: String,
    pub id: u32
}

impl Category {
    pub fn from_hex_str(cat_hex_str: &str) -> Result<Category, CategoryError> {
        let expansion_id: u32 = u32::from_str_radix(cat_hex_str, 16).or(Err(CategoryError::Invalid(format!("Failed to convert '{}' to number.", cat_hex_str))))?;
        Category::from_number(expansion_id)
    }

    pub fn from_str(cat: &str) -> Result<Category, CategoryError> {
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
                return Err(CategoryError::Invalid(format!("Category '{}' not found", cat)));
            }
        };

        Ok(
            Category {
                name: String::from(cat),
                id
            }
        )


    }

    pub fn from_number(cat: u32) -> Result<Category, CategoryError> {
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
                return Err(CategoryError::Invalid(format!("Category '{}' not found", cat)));
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

// #[derive(Error, Debug)]
// pub enum CategoryError {
//
//     #[error("Asset Category: `{0}`")]
//     Invalid(#[from] String),
// }

#[derive(Error, Debug)]
pub enum CategoryError {

    #[error("Asset Category: `{0}`")]
    Invalid(String),
}

// impl Display for CategoryError {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "Category: {}", match self {
//             CategoryError::Invalid(msg) => msg
//         })
//     }
// }

//==================================================================================================

#[derive(Debug, Clone)]
pub struct Chunk {
    pub hex: String,
    pub id: u32,
}

impl Chunk {
    pub fn from_hex_str(chunk_hex_str: &str) -> Result<Chunk, String> {
        let chunk_number: u32 = u32::from_str_radix(chunk_hex_str, 16).or(Err(format!("Failed to parse chunk '{}' to a number.", chunk_hex_str)))?;
        if chunk_number > 255 {
            return Err(format!("Chunk '{}' is out of range 0:255", chunk_number));
        }
        Ok(Chunk {
            hex: String::from(chunk_hex_str),
            id: chunk_number,
        })
    }

    pub fn from_u32(chunk_number: u32) -> Result<Chunk, String> {
        if chunk_number > 255 {
            return Err(format!("Chunk '{}' is out of range 0:255", chunk_number));
        }
        let chunk_name: String = format!("{:02x}", chunk_number);
        Ok(Chunk {
            hex: chunk_name,
            id: chunk_number,
        })
    }

}

//==================================================================================================

#[derive(Debug, Clone)]
pub struct Repository {
    pub name: String,
    pub id: u32
}

impl Repository {
    pub fn from_hex_str(repo_hex_str: &str) -> Result<Repository, String> {
        let expansion_id: u32 = u32::from_str_radix(repo_hex_str, 16).or(Err(format!("Failed to convert '{}' to number.", repo_hex_str)))?;
        Ok(Repository::from_u32(expansion_id))
    }

    pub fn from_str(repo: &str) -> Repository {
        let regex = Reg::new(r"^ex\d+$").unwrap();
        let captured = regex.captures(repo);
        if let Some(r) = captured{
            let expansion: &str = &repo[2..];
            let expansion: Result<u32, _> = expansion.parse();
            if let Ok(id) = expansion{
                return Repository {
                    name: String::from(repo),
                    id
                };
            }
        }
        Repository {
            name: String::from("ffxiv"),
            id: 0
        }
    }

    pub fn from_u32(number: u32) -> Repository {
        let mut expansion = String::new();
        if number > 0 {
            expansion = format!("ex{}", number);
        } else {
            expansion = String::from("ffxiv")
        }
        Repository {
            id: number,
            name: expansion
        }
    }
}

//==================================================================================================

#[derive(Debug, Clone)]
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
