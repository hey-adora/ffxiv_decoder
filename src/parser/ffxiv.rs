use regex::Regex;

pub mod dat;
pub mod index;
pub mod index_path;

// PLATFORM

#[derive(Debug)]
pub struct Platform {
    name: String,
    id: u32
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

// #[derive(Debug)]
// pub enum Platform {
//     Win32 = 0,
//     PS3 = 1,
//     PS4 = 2,
// }
//
// impl Platform {
//     pub fn from(n: u8) -> Platform {
//         match n {
//             1 => Platform::PS3,
//             2 => Platform::PS4,
//             _ => Platform::Win32
//         }
//     }
// }

// REPOSITORY

#[derive(Debug)]
pub struct Repository {
    name: String,
    id: u32
}

impl Repository {
    pub fn from_str(repo: &str) -> Repository {
        let regex = Regex::new(r"^ex\d+$").unwrap();
        let captured = regex.captures(repo);
        if let Some(r) = captured{
            let expansion = &repo[2..];
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
}

// CATEGORY

#[derive(Debug)]
pub struct Category {
    name: String,
    id: u32
}

impl Category {
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
}

// pub enum Category {
//     Common = 0x00,
//     BGCommon = 0x01,
//     BG = 0x02,
//     Cut = 0x03,
//     Chara = 0x04,
//     Shader = 0x05,
//     UI = 0x06,
//     Sound = 0x07,
//     VFX = 0x08,
//     UIScript = 0x09,
//     EXD = 0x0A,
//     GameScript = 0x0B,
//     Music = 0x0C,
//     SQPackTest = 0x12,
//     Debug = 0x13,
// }
//
// impl Category {
//     pub fn from_number(n: u8) -> Result<Category, String> {
//         Ok(
//             match n {
//                 0x00 => Category::Common,
//                 0x01 => Category::BGCommon,
//                 0x02 => Category::BG,
//                 0x03 => Category::Cut,
//                 0x04 => Category::Chara,
//                 0x05 => Category::Shader,
//                 0x06 => Category::UI,
//                 0x07 => Category::Sound,
//                 0x08 => Category::VFX,
//                 0x09 => Category::UIScript,
//                 0x0A => Category::EXD,
//                 0x0B => Category::GameScript,
//                 0x0C => Category::Music,
//                 0x12 => Category::SQPackTest,
//                 0x13 => Category::Debug,
//                 _ => {
//                     return Err(format!("Category out of range 0:13, got: {}", { n }));
//                 }
//             }
//         )
//     }
//     pub fn from_str(cat: &str) -> Result<Category, String> {
//         Ok(
//             match cat {
//                 "common" => Category::Common,
//                 "bgcommon" => Category::BGCommon,
//                 "bg" => Category::BG,
//                 "cut" => Category::Cut,
//                 "chara" => Category::Chara,
//                 "shader" => Category::Shader,
//                 "ui" => Category::UI,
//                 "sound" => Category::Sound,
//                 "vfx" => Category::VFX,
//                 "ui_script" => Category::UIScript,
//                 "exd" => Category::EXD,
//                 "game_script" => Category::GameScript,
//                 "music" => Category::Music,
//                 "sqpack_test" => Category::SQPackTest,
//                 "debug" => Category::Debug,
//                 _ => {
//                     return Err(format!("Category '{}' not found", cat));
//                 }
//             }
//         )
//     }
// }