use regex::Regex;

#[derive(Debug, Clone)]
pub struct AssetFileRepository {
    pub name: String,
    pub id: u32
}

impl AssetFileRepository {
    pub fn from_hex_str(repo_hex_str: &str) -> Result<AssetFileRepository, String> {
        let expansion_id: u32 = u32::from_str_radix(repo_hex_str, 16).or(Err(format!("Failed to convert '{}' to number.", repo_hex_str)))?;
        Ok(AssetFileRepository::from_u32(expansion_id))
    }

    pub fn from_str(repo: &str) -> AssetFileRepository {
        let regex = Regex::new(r"^ex\d+$").unwrap();
        let captured = regex.captures(repo);
        if let Some(r) = captured{
            let expansion: &str = &repo[2..];
            let expansion: Result<u32, _> = expansion.parse();
            if let Ok(id) = expansion{
                return AssetFileRepository {
                    name: String::from(repo),
                    id
                };
            }
        }
        AssetFileRepository {
            name: String::from("ffxiv"),
            id: 0
        }
    }

    pub fn from_u32(number: u32) -> AssetFileRepository {
        let mut expansion = String::new();
        if number > 0 {
            expansion = format!("ex{}", number);
        } else {
            expansion = String::from("ffxiv")
        }
        AssetFileRepository {
            id: number,
            name: expansion
        }
    }
}