use crate::apphandle::get_api_ipc;
use tauri::{Manager};
use crate::ipc::socket::socket_send;
use crate::IpcMessage;
use serde_json::json;
use crate::ipc_front::message::IpcFrontMessage;

pub fn ipc_front_handler_system(msg: IpcFrontMessage) {
     match msg.name.as_str() {
        "Poweroff" =>{
            let command = IpcMessage::new("System", "Poweroff", json!({}));
            let _ = socket_send(&command);
        },

        "Reboot" =>{
            let command = IpcMessage::new("System", "Reboot", json!({}));
            let _ = socket_send(&command);
        },

        "Log Out" =>{
            let command = IpcMessage::new("System", "Log Out", json!({}));
            let _ = socket_send(&command);
        },
        "Lock" =>{
            let command = IpcMessage::new("System", "Lock", json!({}));
            let _ = socket_send(&command);
        },
        "Start Dock" => {
            let command = IpcMessage::new("System", "Start Dock", json!({}));
            let _ = socket_send(&command);
        }
        "Open Panel" => {
            let api = get_api_ipc();
            if let Some(window) = api.app_handle.get_webview_window("main") {
                let _ = window.set_resizable(true);
            } 
            let command = IpcMessage::new("System", "Open Panel", json!({}));
            let _ = socket_send(&command);

        }
        "Close Panel" => {
            let api = get_api_ipc();
            if let Some(window) = api.app_handle.get_webview_window("main") {
                let _ = window.set_resizable(true);
            }
            let command = IpcMessage::new("System", "Close Panel", json!({}));
            let _ = socket_send(&command);
        }
        _ => {
            println!(
                "Nombre desconocido: [{}:{}]",
                msg.category,
                msg.name
            );
        }
     }
    
}