use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IpcMessage {
    pub category: String,
    pub name: String,
    pub data: serde_json::Value,
}
