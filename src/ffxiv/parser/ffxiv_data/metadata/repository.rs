use regex::Regex;

#[derive(Debug)]
pub struct Repository {
    pub name: String,
    pub id: u32
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