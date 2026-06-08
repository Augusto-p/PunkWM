use crate::ipc::message::IpcMessage;
use crate::apphandle::get_api_ipc;
// use punkwm_dock_lib::print_in_tty;

pub fn handler_dock_pannel(msg: IpcMessage) {
     match msg.name().as_str() {
        "Save Width Pannel" =>{
            let api = get_api_ipc();
            let width: u16 = msg.data()["Width"].to_string().replace("\"", "").parse::<u16>().unwrap_or(300);
            api.with_config(|cfg| {
               cfg.styles().set_pannel_width(width); 
            });
        },
        "Save Width Dock" =>{
            let api = get_api_ipc();
            let width: u16 = msg.data()["Width"].to_string().replace("\"", "").parse::<u16>().unwrap_or(300);
            api.with_config(|cfg| {
               cfg.styles().set_dock_width(width); 
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