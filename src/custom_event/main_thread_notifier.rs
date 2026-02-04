use x11rb::connection::Connection;
use x11rb::protocol::xproto::ConnectionExt;
use crate::custom_event::entity::CustomEvent;
use x11rb::protocol::xproto::EventMask;
#[derive(Clone)]
pub struct MainThreadNotifier {
    pub tx: std::sync::mpsc::Sender<CustomEvent>,
    pub conn: std::sync::Arc<x11rb::rust_connection::RustConnection>, // O el tipo de tu conexión
    pub root: u32,
    pub wake_up_atom: u32,
}

impl MainThreadNotifier {
    pub fn send(&self, event: CustomEvent) {
        // 1. Enviamos el dato por el canal de Rust
        if self.tx.send(event).is_ok() {
            // 2. Enviamos el ClientMessage para despertar al main (el "timbrazo")
            let msg = x11rb::protocol::xproto::ClientMessageEvent::new(
                32, 
                self.root, 
                self.wake_up_atom, 
                [0; 20]
            );
            let _ = self.conn.send_event(false, self.root, EventMask::SUBSTRUCTURE_NOTIFY, msg);
            let _ = self.conn.flush();
        }
    }
}