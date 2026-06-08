use crate::{ipc::message::IpcMessage, utils::image::Image};
use crate::apphandle::get_api_ipc;
// use punkwm_dock_lib::print_in_tty;

pub fn handler_user(msg: IpcMessage) {
     match msg.name().as_str() {
        "Save Name" =>{
            let name = msg.data()["Name"].to_string();
            println!("Name {}", name);
        },

        "Save Image" =>{
            let api = get_api_ipc();
            let content = msg.data()["Img"].to_string();
            api.with_config(|cfg| {
                let _ = Image::save_from_base64(format!("{}/User", cfg.folder()), content);
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