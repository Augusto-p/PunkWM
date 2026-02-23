use serde::{Deserialize};
use serde::{Serialize, Serializer};
use serde::ser::SerializeMap;

#[derive(Debug, Deserialize, Clone)]
pub struct Cookie{
    pub domain: String,
    pub _include_subdomains: bool,
    pub _path : String,
    pub _secure: bool,
    pub _expires: i64,
    pub name: String,
    pub value: String,
    pub _same_site: SameSite,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SameSite{
    None, 
    Lax,
    Strict
}


impl Serialize for Cookie {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry(&self.name, &self.value)?;
        map.end()
    }
}
impl SameSite {
    fn from_str(value: &str) -> SameSite {
        match value {
            "Lax" => SameSite::Lax,
            "Strict" => SameSite::Strict,
            _ => SameSite::None,
        }
    }
}

impl Cookie {
    pub fn from_netscape_line(line: &str) -> Option<Self> {
        let parts: Vec<&str> = line.split('\t').collect();

        if parts.len() < 7 {
            return None;
        }

        // Quitar #HttpOnly_ si existe
        let domain = parts[0]
            .strip_prefix("#HttpOnly_")
            .unwrap_or(parts[0])
            .to_string();

        let include_subdomains = parts[1] == "TRUE";
        let path = parts[2].to_string();
        let secure = parts[3] == "TRUE";
        let expires = parts[4].parse::<i64>().unwrap_or(0);
        let name = parts[5].to_string();
        let value = parts[6].to_string();

        // SameSite opcional (puede no existir)
        let same_site = if parts.len() >= 8 {
            SameSite::from_str(parts[7])
        } else {
            SameSite::None
        };

        Some(Cookie {
            domain,
            _include_subdomains:include_subdomains,
            _path:path,
            _secure:secure,
            _expires: expires,
            name,
            value,
            _same_site:same_site,
        })
    }
}
pub fn parse_cookies(text: &str) -> Vec<Cookie> {
    text.lines()
        .filter(|line| {
            !line.trim().is_empty() && (!line.starts_with('#') || line.starts_with("#HttpOnly_"))
        })
        .filter_map(|line| Cookie::from_netscape_line(line))
        .collect()
}
pub trait CookieSearch {
    fn search(&self, domain: &str, name: &str) -> Option<&Cookie>;
    fn search_many<'a>(&'a self,domain: &str,names: Vec<&str>,) -> Vec<&'a Cookie>;

}

impl CookieSearch for Vec<Cookie> {
    fn search(&self, domain: &str, name: &str) -> Option<&Cookie> {
        self.iter().find(|c| {
            c.domain == domain && c.name == name
        })
    }
    fn search_many<'a>( &'a self,domain: &str,names: Vec<&str>,) -> Vec<&'a Cookie> {
        self.iter()
            .filter(|c| {
                c.domain == domain &&
                names.contains(&c.name.as_str())
            })
            .collect()
    }


}
pub fn serialize_cookies(cookies: &[Cookie]) -> serde_json::Value {
    let mut map = serde_json::Map::new();
    for c in cookies {
        map.insert(c.name.clone(), serde_json::Value::String(c.value.clone()));
    }
    serde_json::Value::Object(map)
}