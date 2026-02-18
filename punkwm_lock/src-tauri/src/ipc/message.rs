use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IpcMessage {
    pub category: String,
    pub name: String,
    pub data: serde_json::Value,
}
impl IpcMessage{
    pub fn new(category: &str, name: &str, data: serde_json::Value)-> Self{
        Self{
            category: category.to_string(),
            name: name.to_string(),
            data
        }
    }
}
