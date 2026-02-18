mod ipc;
mod utils;

use tauri::{Manager, Size, PhysicalSize, Position, PhysicalPosition};
use tauri::Listener;

use crate::ipc::handler::ipc_handler;
use crate::ipc::message::IpcMessage;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let win = app.get_webview_window("main").unwrap();

            // ⚠️ No usar fullscreen en xinit sin WM
            win.set_fullscreen(false)?;
            win.set_decorations(false)?;
            win.set_resizable(false)?;
            win.set_maximizable(false)?;
            win.set_minimizable(false)?;
            win.set_closable(false)?;

            // ✅ Forzar tamaño = tamaño del monitor
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

            Ok(())
        })
        .run(tauri::generate_context!())
        .unwrap();
}
