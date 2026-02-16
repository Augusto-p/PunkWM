use crate::ipc::message::IpcMessage;
use crate::ipc::handlers::workspace::handler_workspace;
use crate::ipc::handlers::system::handler_system;
use crate::ipc::handlers::layout::handler_layout;
use crate::utils::config::print_in_tty;
use crate::custom_event::main_thread_notifier::MainThreadNotifier;
use crate::ipc::handlers::panel::home::handler_home_panel;
use crate::ipc::handlers::panel::apps::handler_apps_panel;
use crate::ipc::handlers::panel::network::handler_network_panel;

pub fn handler(msg: IpcMessage, notifier: &MainThreadNotifier) {

    match msg.category.as_str() {
        "Workspace" =>{
            handler_workspace(msg, &notifier);
        }
        "System" =>{
            handler_system(msg, &notifier);
        },
        "Layout" =>{
            handler_layout(msg, &notifier);
        }
        "Panel:Home" =>{
            handler_home_panel(msg, &notifier);
        }
        "Panel:Apps" =>{
            handler_apps_panel(msg, &notifier);
        }
        "Panel:Network" =>{
            handler_network_panel(msg, &notifier);
        }
        _ => {
            let t = format!("Categoria desconocido: [{}]",msg.category);
            let _ = print_in_tty(
                &t
            );
        }
    }
}

