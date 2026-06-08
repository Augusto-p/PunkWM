use std::{fs};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct ConfigWeather{
    #[serde(skip)]
    path: String,
    city: String,
    state: String,
    country: String,
    units: String,
}

impl ConfigWeather {
    pub fn get_city(&self)->String{self.city.clone()}

    pub fn get_state(&self)->String{self.state.clone()}

    pub fn get_country(&self)->String{self.country.clone()}

    pub fn get_units(&self)->String{self.units.clone()}

    pub fn set_city(&mut self, city:String)->bool{
        self.city = city.replace("\"", "");
        self.update()
    }

    pub fn set_state(&mut self, state:String)->bool{
        self.state = state.replace("\"", "");
        self.update()
    }

    pub fn set_country(&mut self, country:String)->bool{
        self.country = country.replace("\"", "");
        self.update()
    }

    pub fn set_units(&mut self, units:String)->bool{
        self.units = units.replace("\"", "");
        self.update()
    }

    pub fn load(folder: String) -> Self {
        let path = format!("{}/weather.toml", folder);

        // Intentamos ejecutar el bloque de lectura y parseo
        let result = (|| -> Result<Self, Box<dyn std::error::Error>> {
            let data = fs::read_to_string(&path)?;
            let data_limpia = data.lines()
                    .filter(|line| line.trim() != "[weather]")
                    .collect::<Vec<_>>()
                    .join("\n");
            let mut cfg: ConfigWeather = toml::from_str(&data_limpia)?;
            cfg.path = path.clone();
            Ok(cfg)
        })();

        // Si el resultado es Ok, lo devuelve. Si es Err, devuelve los valores por defecto.
        result.unwrap_or(Self {
            path: path,
            city: "Francorchamps".to_string(),
            state: "Liege".to_string(),
            country: "BE".to_string(),
            units: "metric".to_string(),
        })
    }

    pub fn update(&self) -> bool {
        let toml_string = match toml::to_string(self) {
            Ok(s) => s,
            Err(_) => return false, // Si falla la serialización, salimos con false
        };
        let final_toml = format!("[weather]\n{}", toml_string);
        fs::write(&self.path, final_toml).is_ok()
    }

}

