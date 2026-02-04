use crate::ipc::message::IpcMessage;
use crate::custom_event::entity::CustomEvent;
// use crate::print_in_tty;
use crate::MainThreadNotifier;

pub fn handler_workspace( msg: IpcMessage, notifier: &MainThreadNotifier) {
    match msg.name.as_str() {
        "Set" => {
            if let Some(space_value) = msg.data.get("space") {
                if let Some(space) = space_value.as_i64() {
                    if let Ok(space_u8) = u8::try_from(space) {
                        notifier.send(CustomEvent::SwitchTo(space_u8.into()));
                    }
                }
            }
        }

        _ => {
            println!(
                "Nombre desconocido: [{}:{}]",
                msg.category,
                msg.name
            );
        }
     }
    
}
