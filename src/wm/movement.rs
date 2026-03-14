use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;


use super::manager::WorkspaceManager;

impl WorkspaceManager {
        pub fn move_window_to(&mut self, win: Window, ws: usize) {
        if ws >= self.workspaces.len() {
            return;
        }

        // 1️⃣ Sacar ventana del workspace actual
        if let Some(pos) = self.workspaces[self.current].iter().position(|&w| w == win) {
            self.workspaces[self.current].remove(pos);

            // Ajustar foco del workspace actual
            match self.focused[self.current] {
                Some(f) if f == pos => {
                    if self.workspaces[self.current].is_empty() {
                        self.focused[self.current] = None;
                    } else {
                        self.focused[self.current] =
                            Some(pos.min(self.workspaces[self.current].len() - 1));
                    }
                }
                Some(f) if f > pos => {
                    self.focused[self.current] = Some(f - 1);
                }
                _ => {}
            }
        }

        // 2️⃣ Agregar ventana al workspace destino
        self.workspaces[ws].push(win);
        self.focused[ws] = Some(self.workspaces[ws].len() - 1);

        // 3️⃣ Map / Unmap + foco real en X11
        if ws != self.current {
            let _ = self.conn.unmap_window(win);
        } else {
            let _ = self.focus_window(win);
        }

        let _ = self.conn.flush();
    }


}