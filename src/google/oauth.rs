use reqwest::Client;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::sync::{Arc, Mutex};
use tokio::sync::oneshot;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::google::credentials::Credentials;


#[derive(Serialize, Deserialize, Debug)]
pub struct TokenResponse {
    pub access_token: String,
    expires_in: u64,
    pub refresh_token: Option<String>,
    scope: String,
    token_type: String,
    now: Option<u64>
}

pub fn get_auth_url(credentials: &Credentials, scope: Vec<String> ) -> String {
    format!(
        "https://accounts.google.com/o/oauth2/v2/auth?client_id={}&redirect_uri={}&response_type=code&scope={}",
        credentials.installed.client_id,
        credentials.installed.redirect_uris[0],
        scope.join("%20")
    )
}

fn extract_code_from_url(url: &str) -> Option<String> {
    url.split("?code=").nth(1).map(|s| s.to_string())
}

pub async fn exchange_code_for_token(received_code: String, credentials: &Credentials) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let mut params = HashMap::new();
    params.insert("code", received_code);
    params.insert("client_id", credentials.installed.client_id.clone());
    params.insert("client_secret", credentials.installed.client_secret.clone());
    params.insert("redirect_uri", credentials.installed.redirect_uris[0].clone());
    params.insert("grant_type", "authorization_code".to_string());

    
    let res = client
        .post("https://oauth2.googleapis.com/token")
         .form(&params)
        .send()
        .await?;

    let mut json = res.json().await?;
    
    let _ = save_access_token(&mut json);
    

    Ok(())
}

pub fn save_access_token(token: &mut TokenResponse) -> Result<(), Box<dyn Error>> {
    // Obtenemos la hora actual en Unix timestamp
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
     token.now = Some(now);

    // Creamos/escribimos el archivo
    let file = File::create("Access_Token.json")?;
    serde_json::to_writer_pretty(file, token)?;
    Ok(())
}

pub fn read_access_token() -> Option<TokenResponse> {
    let reader = BufReader::new(File::open("Access_Token.json").ok()?);
    serde_json::from_reader(reader).ok()
}


pub async fn wait_for_code(redirect_url: String) -> String {
    // Variable compartida para almacenar el código
    let received_code = Arc::new(Mutex::new(None::<String>));
    let received_code_for_closure = Arc::clone(&received_code);

    // Canal para notificar cuando se recibe el código
    let (tx, rx) = oneshot::channel::<()>();
    let tx = Arc::new(Mutex::new(Some(tx)));

    let make_svc = make_service_fn(move |_| {
        let code_clone = Arc::clone(&received_code_for_closure);
        let tx = Arc::clone(&tx);

        async move {
            Ok::<_, Infallible>(service_fn(move |req: Request<Body>| {
                let code_clone = Arc::clone(&code_clone);
                let tx = Arc::clone(&tx);

                async move {
                    if let Some(query) = req.uri().query() {
                        for pair in query.split('&') {
                            let mut split = pair.split('=');
                            if let (Some(key), Some(value)) = (split.next(), split.next()) {
                                if key == "code" {
                                    // Guardamos el código
                                    *code_clone.lock().unwrap() = Some(value.to_string());

                                    // Notificamos que llegó el código
                                    if let Some(tx_inner) = tx.lock().unwrap().take() {
                                        let _ = tx_inner.send(());
                                    }
                                }
                            }
                        }
                    }

                    Ok::<_, Infallible>(Response::new(Body::from(
                        "Código recibido. Puedes cerrar esta ventana.",
                    )))
                }
            }))
        }
    });
    

    // Bind al redirect_url recibido como String
    let addr_str = extract_host_port(&redirect_url);
    let addr = addr_str.parse().expect("URL inválida");
    let server = Server::bind(&addr).serve(make_svc);

    let server_handle = tokio::spawn(server);

    // Esperamos a que llegue el código
    let _ = rx.await;

    // Apagamos el servidor
    server_handle.abort();

    // Retornamos el código
    let x = received_code.lock().unwrap().clone().unwrap();
    x
}
fn extract_host_port(url: &str) -> String {
    let url = url.trim_start_matches("http://")
                 .trim_start_matches("https://");

    // Reemplazamos "localhost" por "127.0.0.1"
    url.replace("localhost", "127.0.0.1")
}

pub async fn refresh_token(
    refresh_token: String,
    credentials: &Credentials,
) -> Option<TokenResponse> {
    let client = Client::new();

    let mut params = HashMap::new();
    params.insert("client_id", credentials.installed.client_id.clone());
    params.insert("client_secret", credentials.installed.client_secret.clone());
    params.insert("refresh_token", refresh_token.clone());
    params.insert("grant_type", "refresh_token".to_string());

    let res = client
        .post("https://oauth2.googleapis.com/token")
        .form(&params)
        .send()
        .await
        .ok()?; // 👈 convertimos error a None

    let mut token: TokenResponse = res.json().await.ok()?;

    token.refresh_token = Some(refresh_token);
    // Guardamos el token con `now` actualizado
    save_access_token(&mut token).ok()?;

    Some(token)
}



pub async fn get_access_token(credentials: &Credentials) -> Option<TokenResponse> {
    // 1. Leer el token guardado
    let token = read_access_token()?; // si falla → None
    // 2. Verificar que tenga `now`
    let now_saved = token.now?;
    // 3. Hora actual en unix
    let now_current = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()?
        .as_secs();
    // 4. Calculamos expiración
    let expires_at = now_saved + token.expires_in;

    // 5. Validación
    if now_current < expires_at {
        Some(token)
    } else {
        if let Some(refresh) = token.refresh_token.clone() {
            refresh_token(refresh, credentials).await
        }else{
            None
        }
    }
}
