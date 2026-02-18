use tauri::AppHandle;
use once_cell::sync::OnceCell;
use crate::ipc::message::IpcMessage;
use tauri::Emitter;

static API_IPC: OnceCell<APP> = OnceCell::new();

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
impl EmitArg for IpcMessage {
    fn emit(self, app: &AppHandle) -> tauri::Result<()> {
        app.emit("ipc", self)
    }
}



pub fn set_app_handle(handle: AppHandle) {
    let app = APP::new(handle);
    let _ = API_IPC.set(app);
}

pub fn get_api_ipc()-> &'static APP{
    API_IPC.get().expect("AppHandle no inicializado")
}


