// use punkwm_dock_lib::print_in_tty;
use crate::ipc::{
    message::{IpcMessage},
    wm::handlers::system::handler_system,
};
use crate::apphandle::get_api_ipc;
// use crate::ipc::wm::handlers::workspace::handler_workspace;
// use crate::ipc::wm::handlers::layout::handler_layout;
// use crate::ipc::wm::handlers::panel::apps::handler_panel_apps;
// use crate::ipc::wm::handlers::panel::home::handler_panel_home;
// use crate::ipc::wm::handlers::panel::notify::handler_panel_notify;
// use crate::ipc::wm::handlers::panel::music::handler_panel_music;
// use crate::ipc::wm::handlers::panel::network::handler_panel_network;

pub fn handler(msg: IpcMessage) {
    if msg.bridge() {
        let api_ipc = get_api_ipc();
        let _ = api_ipc.emit(msg);
    } else {
        match msg.category().as_str() {
            "System" => {
                handler_system(msg);
            }
            _ => {
                println!("Categoria desconocido: [{}]", msg.category());
            }
        }
    }
}
