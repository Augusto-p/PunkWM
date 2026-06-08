use crate::apphandle::get_api_ipc;
use tauri::{Manager};
use crate::utils::lockwindow::LockWin;
use serde_json::json;
use crate::ipc::message::IpcMessage;


pub fn handler_system(msg: IpcMessage) {
     match msg.name().as_str() {
        "Lock" =>{
            LockWin::open();
            
            
        },

        _ => {
            println!(
                "Nombre desconocido::: [{}:{}]",
                msg.category(),
                msg.name()
            );
        }
     }
    
}