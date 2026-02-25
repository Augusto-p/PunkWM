use std::sync::{Arc};
use std::sync::mpsc::Sender;

use x11rb::connection::Connection;
use x11rb::protocol::xproto::{
    ClientMessageEvent,
    ConnectionExt,
    EventMask,
};
use x11rb::rust_connection::RustConnection;
use crate::localaudio::entity::LocalAudioCommand;
use crate::custom_event::{entity::CustomEvent,};




// =========================
// ENUM UNIFICADO
// =========================

pub enum CustomizableEvent {
    Custom(CustomEvent),
    Audio(LocalAudioCommand),
}



// =========================
// FROM IMPLEMENTATIONS
// =========================

impl From<CustomEvent> for CustomizableEvent {
    fn from(e: CustomEvent) -> Self {
        CustomizableEvent::Custom(e)
    }
}

impl From<LocalAudioCommand> for CustomizableEvent {
    fn from(e: LocalAudioCommand) -> Self {
        CustomizableEvent::Audio(e)
    }
}



// =========================
// NOTIFIER (CLONABLE)
// =========================

#[derive(Clone)]
pub struct MainThreadNotifier {
    pub tx_custom: Sender<CustomEvent>,
    pub tx_audio: Sender<LocalAudioCommand>,
    pub conn: Arc<RustConnection>,
    pub root: u32,
    pub wake_up_atom: u32,
}

impl MainThreadNotifier {
    pub fn send(&self, event: impl Into<CustomizableEvent>) {
        let event = event.into();

        match event {
        CustomizableEvent::Custom(c) => {
            if self.tx_custom.send(c).is_ok() {
                // Enviar "timbrazo" X11
                let msg = ClientMessageEvent::new(
                    32,
                    self.root,
                    self.wake_up_atom,
                    [0; 20],
                );

                let _ = self.conn.send_event(
                    false,
                    self.root,
                    EventMask::SUBSTRUCTURE_NOTIFY,
                    msg,
                );

                let _ = self.conn.flush();
            }
        }
        CustomizableEvent::Audio(a) => {
            if self.tx_audio.send(a).is_ok() {
            // Enviar "timbrazo" X11
            let msg = ClientMessageEvent::new(
                32,
                self.root,
                self.wake_up_atom,
                [0; 20],
            );

            let _ = self.conn.send_event(
                false,
                self.root,
                EventMask::SUBSTRUCTURE_NOTIFY,
                msg,
            );

            let _ = self.conn.flush();
        }
        }
    }

        // if self.tx.send(event).is_ok() {
        //     // Enviar "timbrazo" X11
        //     let msg = ClientMessageEvent::new(
        //         32,
        //         self.root,
        //         self.wake_up_atom,
        //         [0; 20],
        //     );

        //     let _ = self.conn.send_event(
        //         false,
        //         self.root,
        //         EventMask::SUBSTRUCTURE_NOTIFY,
        //         msg,
        //     );

        //     let _ = self.conn.flush();
        // }
    }
}



// =========================
// RECEIVER (SOLO MAIN)
// =========================





