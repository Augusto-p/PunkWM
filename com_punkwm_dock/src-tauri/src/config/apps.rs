use std::{collections::HashMap, fs};
use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct ConfigApps{
    #[serde(skip)]
    path: String,
    apps:  HashMap<String, String>
}

impl ConfigApps {

    pub fn get_apps(&self)->&HashMap<String, String>{
        &self.apps
    }

    pub fn set_app(&mut self, key: String, value: String) -> bool {
        self.apps.insert(key.replace("\"", ""), value.replace("\"", ""));
        self.update() 
    }

    pub fn pop_app(&mut self, key: String) -> bool {
        self.apps.remove(&key.replace("\"", ""));
        self.update() 
    }


    pub fn load(folder: String) -> Self {
        let path = format!("{}/apps.toml", folder);
        let result = (|| -> Result<Self, Box<dyn std::error::Error>> {
            let data = fs::read_to_string(&path)?;
            let mut cfg: ConfigApps = toml::from_str(&data)?;
            cfg.path = path.clone();
            Ok(cfg)
        })();
        result.unwrap_or(Self {
            path: path,
            apps: HashMap::new()
        })
    }

    pub fn update(&self) -> bool {
        let toml_string = match toml::to_string(self) {
            Ok(s) => s,
            Err(_) => return false, // Si falla la serialización, salimos con false
        };
        fs::write(&self.path, toml_string).is_ok()
    }


    
}
