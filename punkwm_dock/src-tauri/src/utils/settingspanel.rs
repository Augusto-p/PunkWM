pub struct SettingsPanel;
use crate::apphandle::get_api_ipc;
use tauri::WebviewUrl;
use tauri::Manager;
use crate::WebviewWindowBuilder;

impl SettingsPanel{
    pub fn open_main(){
        let api = get_api_ipc();
        let _ = WebviewWindowBuilder::new(&api.get_handle(),"settings_main_win",WebviewUrl::App("settings.html".into()),)
                .visible(true)
                .resizable(false)
                .maximizable(false)
                .minimizable(false)
                .closable(false)
                .decorations(false)
                .build();
                // .devtools(false) // desactiva devtools
    }

    pub fn close(){
        let api = get_api_ipc();
        if let Some(window) = api.get_handle().get_webview_window("settings_main_win") {
            let _ = window.close();
        } 
    }
}