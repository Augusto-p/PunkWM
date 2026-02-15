use crate::ipc_front::handlers::system::ipc_front_handler_system;
use crate::ipc_front::handlers::workspace::ipc_front_handler_workspace;
use crate::ipc_front::handlers::layout::ipc_front_handler_layout;
use crate::ipc_front::handlers::panel::home::ipc_front_handler_home_panel;
use crate::ipc_front::handlers::panel::apps::ipc_front_handler_apps_panel;
use crate::ipc_front::message::IpcFrontMessage;

pub fn ipc_front_handler(msg: IpcFrontMessage) {
    match msg.category.as_str() {
        "System" =>{
            ipc_front_handler_system(msg);
        },
        "Workspace" =>{
            ipc_front_handler_workspace(msg);
        },
        "Layout" =>{
            ipc_front_handler_layout(msg);
        },
        "Panel:Home"=>{
            ipc_front_handler_home_panel(msg);
        },
        "Panel:Apps"=>{
            ipc_front_handler_apps_panel(msg);
        },
        _ => {
            println!(
                "Categoria desconocido: [{}]",
                msg.category
            );
        }
    }
}
