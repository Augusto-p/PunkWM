use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IpcFrontMessage {
    category: String,
    name: String,
    data: serde_json::Value,
}


impl IpcFrontMessage{

     pub fn new<C: Into<String>, N: Into<String>>(
        category: C,
        name: N,
        data: serde_json::Value,
    ) -> Self {
        Self {
            category: category.into(),
            name: name.into(),
            data,
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