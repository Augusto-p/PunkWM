mod ipc;
mod utils;
mod apphandle;

// use punkwm_lock_lib::print_in_tty;
use tauri::{Manager, Size, PhysicalSize, Position, PhysicalPosition};
use tauri::Listener;
use crate::ipc::handler::ipc_handler;
use crate::ipc::message::IpcMessage;
use crate::utils::config::load_config;
use serde_json::json;
use crate::apphandle::set_app_handle;
use tauri::Emitter;

fn main() {
    let config = load_config();
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init()) // <--- Añade esta línea
        .setup(move |app| {
            set_app_handle(app.handle().clone());
            let win = app.get_webview_window("main").unwrap();
            win.set_fullscreen(false)?;
            win.set_decorations(false)?;
            win.set_resizable(false)?;
            win.set_maximizable(false)?;
            win.set_minimizable(false)?;
            win.set_closable(false)?;


            if let Some(monitor) = win.primary_monitor()? {
                let s = monitor.size();

                win.set_size(Size::Physical(PhysicalSize {
                    width: s.width,
                    height: s.height,
                }))?;

                win.set_position(Position::Physical(PhysicalPosition {
                    x: 0,
                    y: 0,
                }))?;
            }

            // opcional kiosco duro
            win.set_always_on_top(true)?;

            // IPC listener
            app.listen("IPC", move |event| {
                if let Ok(msg) = serde_json::from_str::<IpcMessage>(
                    event.payload().trim()
                ) {
                    ipc_handler(msg);
                }
            });

            let handle = app.handle().clone();
            let message = IpcMessage::new("System", "Set Background", json!({"bg": config.lock_screen.bg.clone()}));

            win.on_window_event(move |event| {
                if let tauri::WindowEvent::Focused(true) = event {
                    let _ = handle.emit("ipc", message.clone());
                }
            });


            Ok(())
        })
        .run(tauri::generate_context!())
        .unwrap();
}
