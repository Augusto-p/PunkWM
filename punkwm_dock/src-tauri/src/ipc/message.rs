use serde::{Serialize, Deserialize};
pub const IPC_NAME: &str = "PUNK_DOCK";
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IpcMessage {
    sender: String,
    category: String,
    name: String,
    data: serde_json::Value,
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
    pub fn sender(&self) -> String {
        self.sender.clone()
    }

    pub fn get_category(&self)->String{
        self.category.clone()
    }

    pub fn get_name(&self)->String{
        self.name.clone()
    }

    pub fn get_data(&self)->serde_json::Value{
        self.data.clone()
    }


    
}


