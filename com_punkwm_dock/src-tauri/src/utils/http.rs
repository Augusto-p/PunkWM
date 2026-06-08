use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT, ACCEPT, REFERER};
use reqwest::redirect::Policy;


pub struct HTTP;

impl HTTP {

pub async fn get(url: String) -> String {
    // 1. Configurar la política de redirección (Seguir hasta 10 saltos)
    let custom_policy = Policy::limited(10);

    // 2. Construir el cliente con la política
    let client = match reqwest::Client::builder()
        .redirect(custom_policy) // Esto asegura que el 302 se siga automáticamente
        .build() 
    {
        Ok(c) => c,
        Err(e) => return format!("Error al crear cliente: {}", e),
    };

    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:124.0) Gecko/20100101 Firefox/124.0"));
    headers.insert(ACCEPT, HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8"));
    headers.insert(REFERER, HeaderValue::from_static("https://www.accuweather.com/"));

    // 3. Enviar la petición
    let response = match client
        .get(&url)
        .headers(headers)
        .send()
        .await 
    {
        Ok(res) => res,
        Err(e) => return format!("Error de conexión o redirección: {}", e),
    };

    // Al usar redirección automática, 'response' será la del destino final (donde aterrizó el 302)
    if !response.status().is_success() {
        return format!("Error final: Estado {}", response.status());
    }

    match response.text().await {
        Ok(body) => body,
        Err(e) => format!("Error al leer cuerpo final: {}", e),
    }
}

}