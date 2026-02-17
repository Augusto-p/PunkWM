mod ipc;
use tauri::Manager;
use tauri::Listener;
use crate::ipc::handler::ipc_handler;
use crate::ipc::message::IpcMessage;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let win = app.get_webview_window("main").unwrap();

            win.set_fullscreen(true)?;
            win.set_resizable(false)?;
            win.set_decorations(false)?;
            win.set_maximizable(false)?;
            win.set_minimizable(false)?;
            win.set_closable(false)?;


            app.listen("IPC", move |event| {
                if let Ok(msg) = serde_json::from_str::<IpcMessage>(event.payload().trim()){
                    ipc_handler(msg);
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .unwrap();
}
