use std::fs;
use serde::{Deserialize, Serialize};

use crate::config::config::Config;

#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct ConfigGoogle{
    #[serde(skip)]
    path: String,
    credentials_file: String,
    max_events_view: u16,
    scopes: Vec<String>,
}

impl ConfigGoogle {
    pub fn get_credentials(&self)->&String{&self.credentials_file}

    pub fn get_events(&self)->&u16{&self.max_events_view}

    pub fn get_scopes(&self)->&Vec<String>{&self.scopes}

    pub fn set_credentials(&mut self, credentials:String)->bool{
        self.credentials_file = credentials;
        self.update()
    }

    pub fn set_events(&mut self, events:u16)->bool{
        self.max_events_view = events;
        self.update()
    }

    pub fn add_scope(&mut self, scope:String)->bool{
        self.scopes.push(scope.replace("\"", ""));
        self.update()
    }

    pub fn pop_scope(&mut self, scope:String)->bool{
        let longitud_inicial = self.scopes.len();
        self.scopes.retain(|s| s != &scope.replace("\"", ""));
        if self.scopes.len() < longitud_inicial {
            self.update()
        } else {
            true
        }
    }




    pub fn load(folder: String) -> Self {
        let path = format!("{}/google.toml", folder);
        let result = (|| -> Result<Self, Box<dyn std::error::Error>> {
            let data = fs::read_to_string(&path)?;
            let data_limpia = data.lines()
                    .filter(|line| line.trim() != "[google]")
                    .collect::<Vec<_>>()
                    .join("\n");
            let mut cfg: ConfigGoogle = toml::from_str(&data_limpia)?;
            cfg.path = path.clone();
            cfg.credentials_file = Config::expand_home(&cfg.credentials_file);
            Ok(cfg)
        })();
        result.unwrap_or(Self {
            path: path,
            credentials_file: "".to_string(),
            scopes: Vec::new(),
            max_events_view: 5
            
        })
    }

    pub fn update(&self) -> bool {
        let toml_string = match toml::to_string(self) {
            Ok(s) => s,
            Err(_) => return false, // Si falla la serialización, salimos con false
        };
        let final_toml = format!("[google]\n{}", toml_string);
        fs::write(&self.path, final_toml).is_ok()
    }

}