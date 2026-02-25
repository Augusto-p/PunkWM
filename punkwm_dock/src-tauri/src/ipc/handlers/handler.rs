use punkwm_dock_lib::print_in_tty;
use crate::ipc::message::IpcMessage;
use crate::ipc::handlers::workspace::handler_workspace;
use crate::ipc::handlers::layout::handler_layout;
use crate::ipc::handlers::battery::handler_battery;
use crate::ipc::handlers::network::handler_network;
use crate::ipc::handlers::system::handler_system;
use crate::ipc::handlers::panel::home::handler_panel_home;
use crate::ipc::handlers::panel::notify::handler_panel_notify;
use crate::ipc::handlers::panel::apps::handler_panel_apps;
use crate::ipc::handlers::panel::music::handler_panel_music;
use crate::ipc::handlers::panel::network::handler_panel_network;

pub fn handler(msg: IpcMessage) {
    match msg.category.as_str() {
        "Workspace" =>{
            handler_workspace(msg);
        },
        "Layout" =>{
            handler_layout(msg);
        },
        "Battery" =>{
            handler_battery(msg);
        },
        "Network" =>{
            handler_network(msg);
        },
        "System" =>{
            handler_system(msg);
        },
        "Panel:Home" =>{
            handler_panel_home(msg);
        },
        
        "Panel:Notify" =>{
            handler_panel_notify(msg);
        },
        "Panel:Apps" =>{
            handler_panel_apps(msg);
        },
        "Panel:Network" =>{
            handler_panel_network(msg);
        },
        "Panel:Music" =>{
            handler_panel_music(msg);
        },
        _ => {
            let _ = print_in_tty(&format!("Categoria desconocido: [{}]",msg.category));
        }
    }
}
