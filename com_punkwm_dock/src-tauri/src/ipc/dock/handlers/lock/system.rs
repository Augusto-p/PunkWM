use crate::apphandle::get_api_ipc;
use tauri::{Manager};
use serde_json::json;
use crate::utils::image::Image;
use crate::utils::sysuser::SysUser;
use crate::ipc::message::IpcMessage;


pub fn handler_lock_system(msg: IpcMessage) {
     match msg.name().as_str() {
        "Start Lock" =>{
            let api = get_api_ipc();
            api.with_config(|cfg| {
                let bg = Image::get_image_base64(cfg.styles().get_bg().to_string());
                let userimage = Image::get_image_base64(SysUser::get_photo());
                let _ = api.emit_lock(IpcMessage::new(None,"System:Lock", "User", json!({"image": userimage, "name": SysUser::get_user()})));
                let _ = api.emit_lock(IpcMessage::new(None, "System:Lock", "Bg", json!({"bg": bg})));
            });
            
        },

        _ => {
            println!(
                "Nombre desconocido::: [{}:{}]",
                msg.category(),
                msg.name()
            );
        }
     }
    
}