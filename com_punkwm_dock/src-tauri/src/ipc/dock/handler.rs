use crate::ipc::message::{IpcMessage};
use crate::ipc::dock::handlers::system::handler_system;
use crate::ipc::dock::handlers::lock::system::handler_lock_system;
use crate::ipc::dock::handlers::panel::home::handler_home_panel;
use crate::ipc::dock::handlers::panel::music::handler_music_panel;
use crate::get_api_ipc;

pub fn dock_handler(msg: IpcMessage) {
    if msg.bridge() {
        let api = get_api_ipc();
        api.with_ipc(|ipc| {
            let _ = ipc.send(msg);
        });
        // println!("{:?}", msgwm);
    } else {
        match msg.category().as_str() {
            "System" => {
                handler_system(msg);
            }
            "Lock:System"=>{handler_lock_system(msg);}
            "Panel:Home"=>{
                handler_home_panel(msg);
            },
            
            "Panel:Music"=>{
                handler_music_panel(msg);
            },
            _ => {
                println!("Categoria desconocido: [{}]", msg.category());
            }
        }
    }
}
