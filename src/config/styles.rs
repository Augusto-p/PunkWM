use std::fs;

use serde::{Deserialize, Serialize};

use crate::config::config::Config;

#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct ConfigStyles{
    #[serde(skip)]
    path: String,
    bg: String,
    lock_bg: String,
    dock_width: u16,
    pannel_width: u16,
    lang: String,
    keymap: String,
    music_folders: Vec<String>,
    dock: String
}

impl ConfigStyles {
    
     pub fn get_bg(&self)->&String{&self.bg}
     pub fn get_lock_bg(&self)->&String{&self.lock_bg}
     pub fn get_lang(&self)->&String{&self.lang}
     pub fn get_keymap(&self)->&String{&self.keymap}
     pub fn get_dock_width(&self)->u16{self.dock_width.clone()}
     pub fn get_pannel_width(&self)->u16{self.pannel_width.clone()}
     pub fn get_music_folders(&self)->&Vec<String>{&self.music_folders}
     pub fn get_dock(&self)->&String{&self.dock}

    pub fn set_bg(&mut self, bg:String)->bool{
        self.bg = bg;
        self.update()
    }

    pub fn set_lock_bg(&mut self, lock_bg:String)->bool{
        self.lock_bg = lock_bg;
        self.update()
    }

    pub fn set_lang(&mut self, lang:String)->bool{
        self.lang = lang.replace("\"","");
        self.update()
    }

    pub fn set_keymap(&mut self, keymap:String)->bool{
        self.keymap = keymap.replace("\"","");
        self.update()
    }

    pub fn set_dock_width(&mut self, dock_width:u16)->bool{
        self.dock_width = dock_width;
        self.update()
    }

    pub fn set_pannel_width(&mut self, pannel_width:u16)->bool{
        self.pannel_width = pannel_width;
        self.update()
    }

    pub fn set_dock(&mut self, dock:String)->bool{
        self.dock = dock;
        self.update()
    }



    pub fn add_music_folders(&mut self, folder:String)->bool{
        self.music_folders.push(Config::expand_home(&folder.replace("\"","")));
        self.update()
    }

    pub fn pop_music_folders(&mut self, folder:String)->bool{
        let longitud_inicial = self.music_folders.len();
        let absolute_path = Config::expand_home(&folder.replace("\"",""));
        self.music_folders.retain(|s| s != &folder.replace("\"","") && s != &absolute_path);
        if self.music_folders.len() < longitud_inicial {
            self.update()
        } else {
            true
        }
    }



    pub fn load(folder: String) -> Self {
        let path = format!("{}/styles.toml", folder);
        let result = (|| -> Result<Self, Box<dyn std::error::Error>> {
            let data = fs::read_to_string(&path)?;
            let data_limpia = data.lines()
                    .filter(|line| line.trim() != "[styles]")
                    .collect::<Vec<_>>()
                    .join("\n");
            let mut cfg: ConfigStyles = toml::from_str(&data_limpia)?;
            cfg.path = path.clone();
            cfg.bg = Config::expand_home(&cfg.bg);
            cfg.lock_bg = Config::expand_home(&cfg.lock_bg);
            cfg.dock = Config::expand_home(&cfg.dock);
            let mut music_folders = Vec::new();
            for folder in cfg.music_folders {
                music_folders.push(Config::expand_home(&folder));
            }
            cfg.music_folders = music_folders;
            Ok(cfg)
        })();
        result.unwrap_or(Self {
            path: path,
            bg: "".to_string(),
            music_folders: Vec::new(),
            lock_bg: "".to_string(),
            dock_width: 80,
            pannel_width: 300,
            lang: "en".to_string(),
            keymap: "us".to_string(),
            dock: "".to_string(),
            
        })
    }

    pub fn update(&self) -> bool {
        let toml_string = match toml::to_string(self) {
            Ok(s) => s,
            Err(_) => return false, // Si falla la serialización, salimos con false
        };
        let final_toml = format!("[styles]\n{}", toml_string);
        fs::write(&self.path, final_toml).is_ok()
    }
    
}