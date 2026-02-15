use crate::ipc::message::IpcMessage;
use crate::utils::config::print_in_tty;
use crate::custom_event::main_thread_notifier::MainThreadNotifier;
use crate::custom_event::entity::CustomEvent;
pub fn handler_apps_panel(msg: IpcMessage,notifier: &MainThreadNotifier) {
     match msg.name.as_str() {
        "Open" => {
            notifier.send(CustomEvent::OpenAppsPanel());
        }
        "Search" => {
            let q = msg.data["q"].as_str().unwrap().to_string();
            notifier.send(CustomEvent::AppsPanelSearch(q));
        }
        "Load Apps" => {
            notifier.send(CustomEvent::AppsPanelLoadApps());
        }
        "Open App" => {
            let package = msg.data["package"].as_str().unwrap().to_string();
            notifier.send(CustomEvent::AppsPanelOpenApp(package));
        }
        _ => {
            let t = format!("Nombre desconocido: [{}:{}]",
                msg.category,
                msg.name
            );
            let _ = print_in_tty(&t);
        }
     }
    
}