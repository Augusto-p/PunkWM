use crate::ipc::message::IpcMessage;
use crate::utils::settingspanel::SettingsPanel;
use crate::utils::file::LocalFile;
use crate::utils::image::Image;
use crate::apphandle::get_api_ipc;
// use crate::ipc::message::IpcMessage;
use serde_json::json;

pub fn handler_system(msg: IpcMessage) {
     match msg.name().as_str() {
        "Close" =>{
            SettingsPanel::close();
        },
        "Start"=>{
            let api = get_api_ipc();
            //Send USerName & Image
            
            api.with_config(|cfg| {
                //send Language
                let _ = api.emit_settings(IpcMessage::new(None,"Language", "Load", json!({"Lang":cfg.styles().get_lang()})));
                //send layout
                let _ = api.emit_settings(IpcMessage::new(None,"Layout", "Load", json!({"Keymap":cfg.styles().get_keymap()})));
                //send clima
                let _ = api.emit_settings(IpcMessage::new(None,"Weather", "Load", json!({
                    "City":cfg.weather().get_city(),
                    "State":cfg.weather().get_state(),
                    "Country":cfg.weather().get_country(),
                    "Units": cfg.weather().get_units()})
                ));
                //send panel
                let _ = api.emit_settings(IpcMessage::new(None,"DockPannel", "Load", json!({
                    "Dock_Width":cfg.styles().get_dock_width(),
                    "Pannel_Width":cfg.styles().get_pannel_width()})
                ));
                //send musicfolders
                let _ = api.emit_settings(IpcMessage::new(None,"Music Folders", "Load", json!({"Folders":cfg.styles().get_music_folders()})));
                //send google
                let _ = api.emit_settings(IpcMessage::new(None,"Google", "Load", json!({
                    "Scopes":cfg.google().get_scopes(),
                    "Events":cfg.google().get_events(),
                    "Credentials": LocalFile::read_text_file(cfg.google().get_credentials().to_string())
                })));
                //send wallpapers
                let _ = api.emit_settings(IpcMessage::new(None,"Wallpapers", "Load", json!({
                    "Main": Image::get_image_base64(cfg.styles().get_bg().to_string()),
                    "Lock": Image::get_image_base64(cfg.styles().get_lock_bg().to_string())
                })));
                //send apps
                let _ = api.emit_settings(IpcMessage::new(None,"Apps", "Load", json!({
                    "Apps": cfg.apps().get_apps()
                })));
                //send keyblindigs
                let _ = api.emit_settings(IpcMessage::new(None,"Keybindings", "Load", json!({
                    "Shortcuts": cfg.keybindings().get_keybindings()
                })));


            });

            //send is sudo o whell

        }

        _ => {
            println!(
                "Nombre desconocido: [{}:{}]",
                msg.category(),
                msg.name()
            );
        }
     }
    
}