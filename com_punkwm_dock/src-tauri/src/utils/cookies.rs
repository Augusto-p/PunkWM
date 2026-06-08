use serde::{Deserialize};

#[derive(Debug, Deserialize, Clone)]
pub struct Cookie{
    domain: String,
    name: String,
    value: String,
}

impl Cookie{
    pub fn new(domain: String, name: String, value: String)->Self{
        Self{domain, name,value}
    }

    pub fn _get_domain(&self)->String{
        self.domain.clone()
    }
    pub fn _get_name(&self)->String{
        self.name.clone()
    }
    pub fn _get_value(&self)->String{
        self.value.clone()
    }

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

        let name = parts[5].to_string();
        let value = parts[6].to_string();

        Some(Cookie::new(domain,name,value,))
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
    fn _search(&self, domain: &str, name: &str) -> Option<&Cookie>;
    fn search_many<'a>(&'a self,domain: &str,names: Vec<&str>,) -> Vec<&'a Cookie>;

}

impl CookieSearch for Vec<Cookie> {
    fn _search(&self, domain: &str, name: &str) -> Option<&Cookie> {
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