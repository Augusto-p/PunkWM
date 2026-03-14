use crate::ipc::message::IpcMessage;
use crate::utils::config::print_in_tty;
use crate::custom_event::main_thread_notifier::MainThreadNotifier;
use crate::custom_event::entity::CustomEvent;
pub fn handler_layout(msg: IpcMessage,notifier: &MainThreadNotifier) {
     match msg.get_name().as_str() {
        "Toggle"=>{
            notifier.send(CustomEvent::ToggleLayout());
        }

        _ => {
            let t = format!("Nombre desconocido: [{}:{}]",
                msg.get_category(),
                msg.get_name()
            );
            let _ = print_in_tty(&t);
        }
     }
    
}