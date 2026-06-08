use tauri::AppHandle;
use once_cell::sync::OnceCell;
use crate::ipc::message::IpcMessage;
use serde::Serialize;
use serde::Deserialize;
use tauri::Emitter;
use crate::config::config::Config;
use std::sync::{Arc, Mutex};
use crate::PunkIPC;

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
            category: msg.category(),
            name: msg.name(),
            data: msg.data(),
        }
    }
}




#[derive(Clone)]
pub struct APP {
    app_handle: AppHandle,
    cfg: Arc<Mutex<Config>>,
    ipc: Arc<Mutex<PunkIPC>>,
}

impl APP {
    pub fn new(handle: AppHandle, cfg: Config, ipc: PunkIPC) -> Self {
        Self {
            app_handle: handle,
            cfg: Arc::new(Mutex::new(cfg)),
            ipc: Arc::new(Mutex::new(ipc))
        }
    }
    pub fn emit<T: EmitArg>(&self, msg: T) -> tauri::Result<()> {
            msg.emit(&self.app_handle)
        }
    pub fn emit_lock<T: EmitArg>(&self, msg: T) -> tauri::Result<()> {
            msg.emit_lock(&self.app_handle)
        }
    pub fn emit_settings<T: EmitArg>(&self, msg: T) -> tauri::Result<()> {
            msg.emit_settings(&self.app_handle)
        }    
    pub fn get_handle(&self)->AppHandle{
        self.app_handle.clone()
    }

    pub fn ipc(&self) -> Arc<Mutex<PunkIPC>> {
        self.ipc.clone()
    }
   pub fn with_config<F, R>(&self, f: F) -> R 
    where 
        F: FnOnce(&mut Config) -> R 
    {
        let mut cfg = self.cfg.lock().expect("Failed to lock config");
        f(&mut cfg)
    }

    // Para obtener el config, ahora necesitas bloquear el Mutex
    pub fn with_ipc<F, R>(&self, f: F) -> R 
    where 
        F: FnOnce(&mut PunkIPC) -> R 
    {
        let mut ipc = self.ipc.lock().expect("Failed to lock config");
        f(&mut ipc)
    }
}
pub trait EmitArg {
    fn emit(self, app: &AppHandle) -> tauri::Result<()>;
    fn emit_lock(self, app: &AppHandle) -> tauri::Result<()>;
    fn emit_settings(self, app: &AppHandle) -> tauri::Result<()>;
}
impl EmitArg for EmitPayload {
    fn emit(self, app: &AppHandle) -> tauri::Result<()> {
        app.emit("ipc", self)
    }

    fn emit_lock(self, app: &AppHandle) -> tauri::Result<()> {
        app.emit("ipc-lock", self)
    }
    fn emit_settings(self, app: &AppHandle) -> tauri::Result<()> {
        let payload = EmitPayload::from(self);
        app.emit("IPC_Settings", payload)
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
    fn emit_settings(self, app: &AppHandle) -> tauri::Result<()> {
        let payload = EmitPayload::from(self);
        app.emit("IPC_Settings", payload)
    }
}

pub fn set_app_handle(handle: AppHandle, cfg: Config, ipc: PunkIPC) {
    let app = APP::new(handle, cfg, ipc);
    if let Err(_) = API_IPC.set(app) {
        println!("API_IPC ya estaba inicializado");
    }
}

pub fn get_api_ipc() -> &'static APP {
    API_IPC.get().expect("AppHandle no inicializado")
}