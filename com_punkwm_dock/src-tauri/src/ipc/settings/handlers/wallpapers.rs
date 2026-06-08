use crate::{ipc::message::IpcMessage, utils::image::Image};
use crate::apphandle::get_api_ipc;
// use punkwm_dock_lib::print_in_tty;

pub fn handler_wallpapers(msg: IpcMessage) {
     match msg.name().as_str() {
        "Save Main" =>{
            let api = get_api_ipc();
            let content = msg.data()["Img"].to_string();
            api.with_config(|cfg| {
                let path = Image::save_from_base64(format!("{}/MainWallpaper", cfg.folder()), content);
                cfg.styles().set_bg(path);
            });
        },
        "Save Lock" =>{
            let api = get_api_ipc();
            let content = msg.data()["Img"].to_string();
            api.with_config(|cfg| {
                let path = Image::save_from_base64(format!("{}/LockWallpaper", cfg.folder()), content);
                cfg.styles().set_lock_bg(path);
            });
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