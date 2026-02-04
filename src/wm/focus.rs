use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;
use x11rb::rust_connection::ReplyError;
use x11rb::CURRENT_TIME;

use super::manager::WorkspaceManager;

impl WorkspaceManager {
    pub fn focus_next(&mut self) {
        let ws = self.current;
        let len = self.workspaces[ws].len();
        if len == 0 {
            return;
        }

        let next = match self.focused[ws] {
            Some(i) => (i + 1) % len,
            None => 0,
        };

        self.focused[ws] = Some(next);
        let _ = self.focus_window(self.workspaces[ws][next]);
    }

    pub fn focus_prev(&mut self) {
        let ws = self.current;
        let len = self.workspaces[ws].len();
        if len == 0 {
            return;
        }

        let prev = match self.focused[ws] {
            Some(i) => (i + len - 1) % len,
            None => 0,
        };

        self.focused[ws] = Some(prev);
        let _ = self.focus_window(self.workspaces[ws][prev]);
    }

    pub fn focus_window(&self, win: Window) -> Result<(), ReplyError> {
        self.conn
            .set_input_focus(InputFocus::POINTER_ROOT, win, CURRENT_TIME)?;
        self.conn.flush()?;
        Ok(())
    }

    pub fn close_focused(&mut self, wm_protocols: Atom, wm_delete: Atom) {
        let ws = self.current;
        let idx = match self.focused[ws] {
            Some(i) => i,
            None => return,
        };

        let win = self.workspaces[ws][idx];

        let data = ClientMessageData::from([wm_delete, 0, 0, 0, 0]);
        let event = ClientMessageEvent {
            response_type: CLIENT_MESSAGE_EVENT,
            format: 32,
            sequence: 0,
            window: win,
            type_: wm_protocols,
            data,
        };

        let _ = self.conn.send_event(false, win, EventMask::NO_EVENT, event);
        let _ = self.conn.flush();
    }

        
    pub fn move_focus_window(&mut self, target_ws: usize) {
        let current_ws = self.current;

        // No hacer nada si el workspace destino es inválido o es el mismo
        if target_ws >= self.workspaces.len() || target_ws == current_ws {
            return;
        }

        // Obtener índice de ventana enfocada
        let focused_idx = match self.focused[current_ws] {
            Some(i) => i,
            None => return, // nada enfocado en este workspace
        };

        let win = self.workspaces[current_ws][focused_idx];

        // Usamos la función ya creada para mover la ventana
        self.move_window_to(win, target_ws);

        // Si movimos la ventana fuera del workspace actual, debemos actualizar foco allí
        if self.workspaces[current_ws].is_empty() {
            self.focused[current_ws] = None;
        } else if focused_idx >= self.workspaces[current_ws].len() {
            self.focused[current_ws] = Some(self.workspaces[current_ws].len() - 1);
            let _ = self
                .focus_window(self.workspaces[current_ws][self.workspaces[current_ws].len() - 1]);
        } else {
            self.focused[current_ws] = Some(focused_idx);
            let _ = self.focus_window(self.workspaces[current_ws][focused_idx]);
        }
    }

 
}
