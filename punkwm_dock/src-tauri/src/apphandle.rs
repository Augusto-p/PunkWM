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
    category: String,
    name: String,
    data: serde_json::Value,
}
impl From<IpcMessage> for EmitPayload {
    fn from(msg: IpcMessage) -> Self {
        EmitPayload {
            category: msg.get_category(),
            name: msg.get_name(),
            data: msg.get_data(),
        }
    }
}

impl From<IpcFrontMessage> for EmitPayload {
    fn from(msg: IpcFrontMessage) -> Self {
        EmitPayload {
            category: msg.get_category(),
            name: msg.get_name(),
            data: msg.get_data(),
        }
    }
}
pub struct APP {
    app_handle: AppHandle,
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
    pub fn emit_lock<T: EmitArg>(&self, msg: T) -> tauri::Result<()> {
            msg.emit_lock(&self.app_handle)
        }
    pub fn get_handle(&self)->AppHandle{
        self.app_handle.clone()
    }
}
pub trait EmitArg {
    fn emit(self, app: &AppHandle) -> tauri::Result<()>;
    fn emit_lock(self, app: &AppHandle) -> tauri::Result<()>;
}
impl EmitArg for EmitPayload {
    fn emit(self, app: &AppHandle) -> tauri::Result<()> {
        app.emit("ipc", self)
    }

    fn emit_lock(self, app: &AppHandle) -> tauri::Result<()> {
        app.emit("ipc-lock", self)
    }
}

impl EmitArg for IpcMessage {
    fn emit(self, app: &AppHandle) -> tauri::Result<()> {
        let payload = EmitPayload::from(self);
        app.emit("ipc", payload)
    }

    fn emit_lock(self, app: &AppHandle) -> tauri::Result<()> {
        let payload = EmitPayload::from(self);
        app.emit("ipc-lock", payload)
    }
}
impl EmitArg for IpcFrontMessage {
    fn emit(self, app: &AppHandle) -> tauri::Result<()> {
        let payload = EmitPayload::from(self);
        app.emit("ipc", payload)
    }
    fn emit_lock(self, app: &AppHandle) -> tauri::Result<()> {
        let payload = EmitPayload::from(self);
        app.emit("ipc-lock", payload)
    }
}



pub fn set_app_handle(handle: AppHandle) {
    let app = APP::new(handle);
    let _ = API_IPC.set(app);
}

pub fn get_api_ipc()-> &'static APP{
    API_IPC.get().expect("AppHandle no inicializado")
}


