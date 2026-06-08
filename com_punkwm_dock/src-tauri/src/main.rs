mod config;
mod utils;
mod apphandle;
mod ipc;
use std::env;
use tauri::webview::WebviewWindowBuilder;
use crate::apphandle::set_app_handle;
use crate::apphandle::get_api_ipc;
use crate::utils::settingspanel::SettingsPanel;
use crate::config::config::Config;
use crate::ipc::message::IpcMessage;
use crate::ipc::dock::handler::dock_handler;
use crate::ipc::settings::handler::settings_handler;
use crate::ipc::wm::socket::PunkIPC;
use tauri::{Listener,CursorIcon};
use tauri::{Manager};

fn main() {
    let cfg_folder = String::from("~/.config/PunkWM");
    let mut cfg = Config::load(cfg_folder);
    let mut punk_ipc = PunkIPC::new();

    let mut window_title = "Dock".to_string();
    if let Some(tiite) = env::args().collect::<Vec<String>>().iter().find_map(|s| s.strip_prefix("--title=")){
        window_title = tiite.to_string();
    }


    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_fs::init()) // <--- Añade esta línea
        .plugin(tauri_plugin_dialog::init())
        .setup(move |app| {
            set_app_handle(app.handle().clone(), cfg, punk_ipc);
            WebviewWindowBuilder::new(
                app,
                "dock_window",
                tauri::WebviewUrl::App("index.html".into()),
            )
            .title(&window_title)
            .decorations(false)
            .resizable(true)
            .transparent(true)
            .build()?;

            // Suponiendo que tienes la instancia de la ventana
            let window = app.get_webview_window("dock_window").unwrap();

            // Cambiar a cursor de redimensionamiento horizontal (Este-Oeste)
            window.set_cursor_icon(CursorIcon::Move).unwrap();
            
            let api = get_api_ipc();
            api.with_ipc(|ipc| {
                let _ = ipc.link();

            });

            let ipc = api.ipc();
            let _ = PunkIPC::start_ipc_server(ipc);
            
            // punk_ipc.start_ipc_server();
            app.listen("IPC-Front", move |event| {
                if let Ok(msg) = serde_json::from_str::<IpcMessage>(event.payload().trim()) {
                    dock_handler(msg);
                }
            });

            app.listen("IPC-Settings", move |event| {
                if let Ok(msg) = serde_json::from_str::<IpcMessage>(event.payload().trim()){
                    settings_handler(msg);
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error running tauri app");
}
