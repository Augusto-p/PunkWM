// use crate::ipc::socket::socket_send;

use serde_json::json;
use crate::ipc::message::IpcMessage;
use tauri::{Manager};
use crate::apphandle::get_api_ipc;
use tauri::WebviewUrl;
use tauri::WebviewWindowBuilder;

pub fn handler_home_panel(msg: IpcMessage) {
     match msg.name().as_str() {
        "Google:Oauth:Login" =>{
            let url = msg.data()["URL"].as_str().unwrap().to_string();
            let api = get_api_ipc();
            let _ = WebviewWindowBuilder::new(
                &api.get_handle(),
                "google_oauth_login",
                WebviewUrl::External(url.parse().unwrap())
            )
            .resizable(true)
            .title("__")
            .on_navigation(|url| {
                if let Some(code) = url
                    .query_pairs()
                    .find(|(k, _)| k == "code")
                    .map(|(_, v)| v.to_string())
                {
                    let command = IpcMessage::new(None,"Panel:Home","Google:Oauth:Code",json!({ "code": code }));
                    // let _ = socket_send(&command);
                    if let Some(w) = api.get_handle().get_webview_window("google_oauth_login") {
                        let _ = w.close();
                    }
                    return false; // cancela navegación
                }

                true
            })
            .build();
            
        },
        _ => {
            println!(
                "Nombre desconocido: [{}:{}]",
                msg.category(),
                msg.name()
            );
        }
     }
    
}



