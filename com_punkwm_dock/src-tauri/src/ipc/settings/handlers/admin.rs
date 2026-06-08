use crate::{ipc::message::IpcMessage, utils::image::Image};
// use punkwm_dock_lib::print_in_tty;

pub fn handler_admin(msg: IpcMessage) {
     match msg.name().as_str() {
        "Save Lock Wallpaper" =>{
            let content = msg.data()["Img"].to_string();
            let path = Image::save_from_base64("SessionWall".to_string(), content);
            println!("{}",path);
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