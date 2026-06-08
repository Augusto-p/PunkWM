use crate::config::config::GLOBAL_CFG;

use reqwest::Client;
use serde::{Deserialize, Serialize};

use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, BufWriter},
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TokenResponse {
    access_token: String,
    expires_in: u64,
    refresh_token: Option<String>,
    scope: String,
    token_type: String,

    #[serde(skip)]
    created_at: u64,
}

impl TokenResponse {
    pub fn access_token(&self) -> &str {
        &self.access_token
    }

    pub fn is_expired(&self) -> bool {
        let now = unix_now();

        // margen de seguridad de 30 segundos
        now >= (self.created_at + self.expires_in.saturating_sub(30))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Credentials {
    client_id: String,
    client_secret: String,
    redirect_uris: Vec<String>,
}

impl Credentials {
    fn from_json(json: &str) -> Option<Self> {
        let value: serde_json::Value = serde_json::from_str(json).ok()?;

        serde_json::from_value(value["installed"].clone()).ok()
    }

    pub fn client_id(&self) -> &str {
        &self.client_id
    }

    pub fn client_secret(&self) -> &str {
        &self.client_secret
    }

    pub fn redirect_uri(&self) -> Option<&str> {
        self.redirect_uris.first().map(|s| s.as_str())
    }
}

#[derive(Debug)]
pub struct Google {
    pub(crate) client: Client,
    credentials: Credentials,
    scopes: Vec<String>,
    token: Option<TokenResponse>,
}

impl Google {
    pub fn new() -> Option<Self> {
        let cfg = GLOBAL_CFG.read().ok()?;

        let credentials =
            Credentials::from_json(&cfg.google().get_credenciales())?;

        let scopes = cfg.google().get_scopes();

        drop(cfg);

        Some(Self {
            client: Client::new(),
            credentials,
            scopes,
            token: Self::read_access_token(),
        })
    }

    pub fn is_authenticated(&self) -> bool {
        self.token.is_some()
    }

    pub fn access_token(&self) -> Option<&str> {
        self.token.as_ref().map(|t| t.access_token())
    }
    pub fn auth_url(&self) -> String {
        let scopes = self.scopes.join("%20");

        format!(
            "https://accounts.google.com/o/oauth2/v2/auth\
            ?client_id={}\
            &redirect_uri={}\
            &response_type=code\
            &scope={}",
            self.credentials.client_id(),
            self.credentials.redirect_uri().unwrap_or_default(),
            scopes
        )
    }

    pub async fn ensure_access_token(&mut self) -> Option<()> {
        let token = self.token.clone()?;

        if !token.is_expired() {
            return Some(());
        }

        let refresh = token.refresh_token?;

        self.refresh_token(&refresh).await
    }

    pub async fn exchange_code_for_token(
        &mut self,
        code: &str,
    ) -> Result<(), reqwest::Error> {
        let mut params = HashMap::new();

        params.insert("code", code.to_string());
        params.insert(
            "client_id",
            self.credentials.client_id().to_string(),
        );
        params.insert(
            "client_secret",
            self.credentials.client_secret().to_string(),
        );
        params.insert(
            "redirect_uri",
            self.credentials.redirect_uri().unwrap_or_default().to_string(),
        );
        params.insert(
            "grant_type",
            "authorization_code".to_string(),
        );

        let response = self
            .client
            .post("https://oauth2.googleapis.com/token")
            .form(&params)
            .send()
            .await?;

        let mut token: TokenResponse = response.json().await?;

        token.created_at = unix_now();

        self.token = Some(token);

        self.save_access_token();

        Ok(())
    }

    async fn refresh_token(
        &mut self,
        refresh_token: &str,
    ) -> Option<()> {
        let mut params = HashMap::new();

        params.insert(
            "client_id",
            self.credentials.client_id().to_string(),
        );

        params.insert(
            "client_secret",
            self.credentials.client_secret().to_string(),
        );

        params.insert(
            "refresh_token",
            refresh_token.to_string(),
        );

        params.insert(
            "grant_type",
            "refresh_token".to_string(),
        );

        let response = self
            .client
            .post("https://oauth2.googleapis.com/token")
            .form(&params)
            .send()
            .await
            .ok()?;

        let mut token: TokenResponse = response.json().await.ok()?;

        token.refresh_token = Some(refresh_token.to_string());
        token.created_at = unix_now();

        self.token = Some(token);

        self.save_access_token();

        Some(())
    }

    fn token_path() -> Option<PathBuf> {
        let cfg = GLOBAL_CFG.read().ok()?;

        Some(
            PathBuf::from(cfg.folder())
                .join("Google_Access_Token.json"),
        )
    }

    fn save_access_token(&self) {
        let Some(path) = Self::token_path() else {
            return;
        };

        let Some(token) = &self.token else {
            return;
        };

        let file = match File::create(path) {
            Ok(file) => file,
            Err(_) => return,
        };

        let writer = BufWriter::new(file);

        let _ = serde_json::to_writer_pretty(writer, token);
    }

    fn read_access_token() -> Option<TokenResponse> {
        let path = Self::token_path()?;

        let file = File::open(path).ok()?;

        let reader = BufReader::new(file);

        serde_json::from_reader(reader).ok()
    }
}

fn unix_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}