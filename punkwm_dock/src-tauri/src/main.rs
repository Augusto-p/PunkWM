mod ipc;
mod apphandle;
mod ipc_front;

// use punkwm_dock_lib::print_in_tty;
use std::env;
use tauri::webview::WebviewWindowBuilder;
use crate::ipc::socket::socket_listen;
use crate::ipc::message::IpcMessage;
use crate::apphandle::set_app_handle;
use crate::ipc_front::handler::ipc_front_handler;
use crate::ipc_front::message::IpcFrontMessage;


  
use tauri::Listener;


#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
fn main() {
    // ───────────── ARGUMENTOS ─────────────
    let args: Vec<String> = env::args().collect();

    let mut window_title = "Dock".to_string();
    let mut width: f64 = 0.0;
    let mut height: f64 = 0.0;

    for arg in &args {
        if let Some(w) = arg.strip_prefix("--width=") {
            width = w.parse().unwrap_or(width);
        }
        if let Some(h) = arg.strip_prefix("--height=") {
            height = h.parse().unwrap_or(height);
        }
        if let Some(t) = arg.strip_prefix("--title=") {
            window_title = t.to_string();
        }
    }

    tauri::Builder::default()
        .setup(move |app| {
            set_app_handle(app.handle().clone());
            // ───────────── VENTANA ─────────────
            WebviewWindowBuilder::new(
                app,
                "main",
                tauri::WebviewUrl::App("index.html".into()),
            )
            .title(&window_title)
            .decorations(false)
            .resizable(true)
            .transparent(true)
            .inner_size(width, height)
            .min_inner_size(width, height)            
            .build()?;



            // ───────────── SOCKET IPC ─────────────
            
            std::thread::spawn(move || {
                socket_listen();
            });

            app.listen("IPC-Front", move |event| {
                // FRONT
                if let Ok(msg) = serde_json::from_str::<IpcFrontMessage>(event.payload().trim()){
                    ipc_front_handler(msg);
                }
            });


            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error running tauri app");
        //         .invoke_handler(tauri::generate_handler![
        //     commands::system::sys_poweroff,
        //     commands::system::sys_reboot,
        //     commands::system::sys_log_out,
        //     commands::system::sys_lock,
        //     commands::system::sys_start_dock,
        //     commands::layout::layout_toggle,
        //     commands::workspace::workspcace_set,
        // ])

}
