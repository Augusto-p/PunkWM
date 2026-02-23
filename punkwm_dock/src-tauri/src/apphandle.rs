use tauri::AppHandle;
use once_cell::sync::OnceCell;
use crate::IpcMessage;
use crate::IpcFrontMessage;
use serde::Serialize;
use serde::Deserialize;
use tauri::Emitter;

static API_IPC: OnceCell<APP> = OnceCell::new();

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmitPayload {
    pub category: String,
    pub name: String,
    pub data: serde_json::Value,
}
impl From<IpcMessage> for EmitPayload {
    fn from(msg: IpcMessage) -> Self {
        EmitPayload {
            category: msg.category,
            name: msg.name,
            data: msg.data,
        }
    }
}

impl From<IpcFrontMessage> for EmitPayload {
    fn from(msg: IpcFrontMessage) -> Self {
        EmitPayload {
            category: msg.category,
            name: msg.name,
            data: msg.data,
        }
    }
}
pub struct APP {
    pub app_handle: AppHandle,
}

impl APP {
    pub fn new(handle: AppHandle) -> Self {
        Self {
            app_handle: handle,
        }
    }
    pub fn emit<T: EmitArg>(&self, msg: T) -> tauri::Result<()> {
            msg.emit(&self.app_handle)
        }
    
    
}
pub trait EmitArg {
    fn emit(self, app: &AppHandle) -> tauri::Result<()>;
}
impl EmitArg for EmitPayload {
    fn emit(self, app: &AppHandle) -> tauri::Result<()> {
        app.emit("ipc", self)
    }
}

impl EmitArg for IpcMessage {
    fn emit(self, app: &AppHandle) -> tauri::Result<()> {
        let payload = EmitPayload::from(self);
        app.emit("ipc", payload)
    }
}
impl EmitArg for IpcFrontMessage {
    fn emit(self, app: &AppHandle) -> tauri::Result<()> {
        let payload = EmitPayload::from(self);
        app.emit("ipc", payload)
    }
}



pub fn set_app_handle(handle: AppHandle) {
    let app = APP::new(handle);
    let _ = API_IPC.set(app);
}

pub fn get_api_ipc()-> &'static APP{
    API_IPC.get().expect("AppHandle no inicializado")
}


