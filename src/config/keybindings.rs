use std::{collections::HashMap, fs};
use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct ConfigKeybindings{
    #[serde(skip)]
    path: String,
    keybindings:  HashMap<String, String>
}

impl  ConfigKeybindings {

    pub fn get_keybindings(&self)->&HashMap<String, String>{
        &self.keybindings
    }

    pub fn set_keybinding(&mut self, key: String, value: String) -> bool {
        self.keybindings.insert(key, value);
        self.update() 
    }



    pub fn load(folder: String) -> Self {
        let path = format!("{}/keybindings.toml", folder);
        let result = (|| -> Result<Self, Box<dyn std::error::Error>> {
            let data = fs::read_to_string(&path)?;
            let mut cfg: ConfigKeybindings = toml::from_str(&data)?;
            cfg.path = path.clone();
            Ok(cfg)
        })();
        result.unwrap_or(Self {
            path: path,
            keybindings: HashMap::new()
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
