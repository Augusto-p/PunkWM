pub struct LockWin;
use crate::WebviewWindowBuilder;
use std::env;
use crate::apphandle::get_api_ipc;
use tauri::WebviewUrl;
use tauri::Manager;

impl LockWin{
    pub fn open(){
        let api = get_api_ipc();
        let args: Vec<String> = env::args().collect();
        let mut window_title = "Lock_Dock".to_string();
        for arg in &args {
        
            if let Some(t) = arg.strip_prefix("--title=") {
                window_title = format!("Lock_{}", t.to_string()).to_string();
            }
        }
        let _ = WebviewWindowBuilder::new(&api.get_handle(),"lock_win",WebviewUrl::App("lock.html".into()),)
                .title(&window_title)
                .visible(true)
                .resizable(false)
                .maximizable(false)
                .minimizable(false)
                .closable(false)
                .decorations(false)
                .devtools(false) // desactiva devtools
                .build();
    }

    pub fn close(){
        let api = get_api_ipc();
        if let Some(window) = api.get_handle().get_webview_window("lock_win") {
            let _ = window.close();
        } 
    }
}