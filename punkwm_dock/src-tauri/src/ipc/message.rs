use serde::{Serialize, Deserialize};
pub const IPC_NAME: &str = "PUNK_DOCK";
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IpcMessage {
    sender: String,
    pub category: String,
    pub name: String,
    pub data: serde_json::Value,
}

impl IpcMessage{
     pub fn new(
        category: impl Into<String>,
        name: impl Into<String>,
        data: serde_json::Value,
    ) -> Self {
        Self {
            sender: IPC_NAME.to_string(),
            category: category.into(),
            name: name.into(),
            data,
        }
    }
    
    // ✅ solo lectura
    pub fn sender(&self) -> &str {
        &self.sender
    }
    
}


