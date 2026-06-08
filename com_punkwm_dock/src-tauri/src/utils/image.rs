use base64::{engine::general_purpose, Engine as _};
use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::path::Path;

pub struct Image;

impl Image {
    pub fn save_from_base64(name: String, content: String) -> String {
        // 1. Quitar comillas dobles y espacios en blanco que puedan venir del JSON
        let sanitized = content.trim_matches(|c| c == '"' || c == ' ' || c == '\n' || c == '\r');

        // 2. Extraer la extensión y limpiar el prefijo data:image/...
        let extension = if sanitized.contains("image/png") {
            "png"
        } else if sanitized.contains("image/jpeg") || sanitized.contains("image/jpg") {
            "jpg"
        } else {
            "bin"
        };

        let clean_content = sanitized.split(',').last().unwrap_or(sanitized);

        // 3. Decodificar
        let bytes = match general_purpose::STANDARD.decode(clean_content) {
            Ok(b) => b,
            Err(_) => return "".to_string(),
        };

        let file_name = format!("{}.{}", name, extension);

        let mut file = match File::create(&file_name) {
            Ok(f) => f,
            Err(_) => return "".to_string(),
        };

        match file.write_all(&bytes) {
            Ok(_) => file_name,
            Err(_) => "".to_string(),
        }
    }

    pub fn get_image_base64(path: String) -> String {
        let mut file = match File::open(&path) {
            Ok(f) => f,
            Err(_) => return String::from(""),
        };

        let mut buffer = Vec::new();
        if file.read_to_end(&mut buffer).is_err() {
            return String::from("");
        }

        let b64_encoded = general_purpose::STANDARD.encode(buffer);

        let extension = Path::new(&path)
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("png"); // Valor por defecto si no tiene extensión

        format!("data:image/{};base64,{}", extension, b64_encoded)
    }
}
