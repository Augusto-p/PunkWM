use std::fs::File;
use std::io::BufReader;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Installed {
    pub client_id: String,
    pub project_id: String,
    pub auth_uri: String,
    pub token_uri: String,
    pub auth_provider_x509_cert_url: String,
    pub client_secret: String,
    pub redirect_uris: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Credentials {
    pub installed: Installed,
}

pub fn read_credentials(filepath: String) -> Option<Credentials> {
    let file = File::open(filepath).ok()?;        // Si falla, devuelve None
    let reader = BufReader::new(file);
    let credentials: Credentials = serde_json::from_reader(reader).ok()?; // Si falla, None
    Some(credentials)
}