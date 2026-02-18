use qrcode::{QrCode};
use qrcode::render::svg;
use base64::{engine::general_purpose, Engine as _};

pub fn get_qrcode_in_base64(data: &str) -> Option<String>{
    let code_result = QrCode::new(data.as_bytes()); 
    match code_result {
        Ok(code) => {
            let image = code.render()
                .min_dimensions(200, 200)
                .dark_color(svg::Color("#ffffff"))
                .light_color(svg::Color("#ffffff00"))
                .build();
                let b64_encoded = general_purpose::STANDARD.encode(image.as_bytes());
                let data_url = format!("data:image/svg+xml;base64,{}", b64_encoded);
                return Some(data_url);
        }
        Err(_) => return None,
    }
}