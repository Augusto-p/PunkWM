use serde::{Deserialize, Deserializer, Serialize,Serializer};
use num_bigint::BigUint;

use crate::ipc::wm::socket::PunkIPC;

pub const IPC_PUNK_DOCK_NAME: &str = "PUNK_DOCK";
pub const IPC_PUNK_NAME: &str = "PUNK";

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum IpcMode {Normal,Bridge,}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PunkIpcMessage{
    pub sender: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub data:  serde_json::Value,
}

impl PunkIpcMessage{
    pub fn new_punk_ipc_connection(data: DHClientDataMessage)->Self{
        let json = serde_json::to_value(data).unwrap();
        Self{
            sender:IPC_PUNK_DOCK_NAME.to_string(),
            type_: String::from("DOCK CONNECTION D/H"),
            data: json
        }
    }

    pub fn new_punk_ipc_message(data: serde_json::Value)->Self{
        Self{
            sender:IPC_PUNK_DOCK_NAME.to_string(),
            type_: String::from("DOCK MESSAGE ENCODE"),
            data: data
        }
    }

}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IpcMessageEncrypted{
    pub encrypted: Vec<u8>,
    pub nonce: [u8; 12]
}


impl IpcMessageEncrypted{
    pub fn new(encrypted: Vec<u8>,nonce: [u8; 12])->Self{
        Self{
            encrypted,
            nonce
        }
    }

}



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DHClientDataMessage{
    pub p: BigUint,
    pub g: BigUint,
    pub pub_key: BigUint,
}

impl DHClientDataMessage{
    pub fn from_punk_ipc(connect: &PunkIPC)-> Self{
        Self{
            p:connect.p(),
            g:connect.g(),
            pub_key:connect.my_pub_key()
        }
        
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DHServerDataMessage{
    pub pub_key: BigUint,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IpcMessage {
    #[serde(rename = "Bridge", deserialize_with = "deserialize_bridge",serialize_with = "serialize_bridge", default)]
    mode: IpcMode,
    category: String,
    name: String,
    data: serde_json::Value,
}

impl IpcMessage {
    pub fn new<C, N>(
        mode: Option<IpcMode>, 
        category: C, 
        name: N, 
        data: serde_json::Value
    ) -> Self
    where
        C: Into<String>,
        N: Into<String>,
    {
        Self {
            // Si es Some(m) usa m, si es None usa IpcMode::default() (Normal)
            mode: mode.unwrap_or_default(), 
            category: category.into(),
            name: name.into(),
            data,
        }
    }

    pub fn category(&self) -> String {
        self.category.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn data(&self) -> serde_json::Value {
        self.data.clone()
    }
    pub fn mode(&self)->IpcMode{
        self.mode.clone()
    }
    pub fn bridge(&self)->bool{
        self.mode == IpcMode::Bridge
    }
}








fn deserialize_bridge<'de, D>(deserializer: D) -> Result<IpcMode, D::Error> where
    D: Deserializer<'de>,
{
    let is_bridge = Option::<bool>::deserialize(deserializer)?.unwrap_or(false);
    Ok(if is_bridge { IpcMode::Bridge } else { IpcMode::Normal })
}

fn serialize_bridge<S>(mode: &IpcMode, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // Si el modo es Bridge, serializa como true, de lo contrario false
    serializer.serialize_bool(*mode == IpcMode::Bridge)
}
impl Default for IpcMode {
    fn default() -> Self {
        IpcMode::Normal
    }
}
