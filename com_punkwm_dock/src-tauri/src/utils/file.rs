use std::fs::File;
use std::io::Write;
use std::fs::read_to_string;
pub struct LocalFile;

impl LocalFile {
    pub fn save_text_file(name: String,ext: String, content: String) -> String {
        // 1. Limpiamos posibles comillas extras (por el error anterior)
        let clean_content =
            content.trim_matches(|c| c == '"' || c == ' ' || c == '\n' || c == '\r');

        // 2. Definimos el nombre del archivo
        let file_name = format!("{}.{}", name, ext);

        // 3. Intentamos crear el archivo
        let mut file = match File::create(&file_name) {
            Ok(f) => f,
            Err(e) => return format!("Error al crear el archivo: {}", e),
        };

        // 4. Escribimos el contenido directamente como texto
        match file.write_all(clean_content.as_bytes()) {
            Ok(_) => file_name,
            Err(e) => format!("Error al escribir texto: {}", e),
        }
    }
    pub fn save_json_text_file(name: String, content: String) -> String {
        return Self::save_text_file(name, "json".to_string(), content);
    }

    pub fn read_text_file(path: String) -> String{
        match read_to_string(path) {
            Ok(f) => f,
            Err(_) => "".to_string(),
        }    
    }

}
