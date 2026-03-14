use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IpcMessage {
    category: String,
    name: String,
    data: serde_json::Value,
}
impl IpcMessage{
    pub fn new(category: &str, name: &str, data: serde_json::Value)-> Self{
        Self{
            category: category.to_string(),
            name: name.to_string(),
            data
        }
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
