use super::manager::WorkspaceManager;
use x11rb::protocol::xproto::*;
// use crate::utils::config::print_in_tty;
use x11rb::{connection::Connection, protocol::xproto::Window};
use crate::ipc::senders::workspace::sender_workspace_update;
use crate::ipc::senders::layout::sender_layout_set;

impl WorkspaceManager {
    pub fn manage_window(&mut self, win: Window) {
        // 🟣 Dock global
        if self.is_dock_window(win) {
            if let Some(dock) = &mut self.dock {
                dock.manage(win, &self.conn);
            }
            return;
        }

        // 🔵 Ventana normal
        let ws = self.current;
        self.workspaces[ws].push(win);
        self.focused[ws] = Some(self.workspaces[ws].len() - 1);

        let _ = self.conn.map_window(win);
        let _ = self.focus_window(win);
        self.apply_layout();
    }

    pub fn unmanage_window(&mut self, win: Window) {
        for (ws, clients) in self.workspaces.iter_mut().enumerate() {
            if let Some(idx) = clients.iter().position(|&w| w == win) {
                clients.remove(idx);

                // Ajustar foco del workspace
                self.focused[ws] = if clients.is_empty() {
                    None
                } else if let Some(f) = self.focused[ws] {
                    if f == idx {
                        // se cerró la enfocada
                        Some(idx.min(clients.len() - 1))
                    } else if f > idx {
                        // corremos el foco hacia atrás
                        Some(f - 1)
                    } else {
                        Some(f)
                    }
                } else {
                    Some(0)
                };

                // Solo recalcular layout si es el workspace actual
                if ws == self.current {
                    self.apply_layout();
                }

                break; // una ventana solo puede estar en un workspace
            }
        }
    }

    pub fn switch_to(&mut self, ws: usize) {
        if ws >= self.workspaces.len() || ws == self.current {
            return;
        }
        for w in &self.workspaces[self.current] {
            let _ = self.conn.unmap_window(*w);
        }
        self.current = ws;

        for w in &self.workspaces[ws] {
            let _ = self.conn.map_window(*w);
        }

        self.apply_layout();

        if let Some(i) = self.focused[ws] {
            let _ = self.focus_window(self.workspaces[ws][i]);
        }

        let _ = self.conn.flush();
        sender_layout_set(self);
        sender_workspace_update(self);
    }

    fn is_dock_window(&self, win: Window) -> bool {
        let win_name = Self::get_window_name(&self.conn, win).unwrap_or_default();

        match &self.dock {
            Some(dock) => !win_name.is_empty() && win_name == dock.name(),
            None => false,
        }
    }

    fn get_window_name<C: Connection>(conn: &C, win: Window) -> Option<String> {
        let net_wm_name = conn
            .intern_atom(false, b"_NET_WM_NAME")
            .ok()?
            .reply()
            .ok()?
            .atom;

        let utf8_string = conn
            .intern_atom(false, b"UTF8_STRING")
            .ok()?
            .reply()
            .ok()?
            .atom;

        // _NET_WM_NAME (moderno)
        if let Ok(cookie) = conn.get_property(false, win, net_wm_name, utf8_string, 0, u32::MAX) {
            if let Ok(reply) = cookie.reply() {
                if !reply.value.is_empty() {
                    return String::from_utf8(reply.value).ok();
                }
            }
        }

        // WM_NAME (fallback)
        if let Ok(cookie) =
            conn.get_property(false, win, AtomEnum::WM_NAME, AtomEnum::STRING, 0, u32::MAX)
        {
            if let Ok(reply) = cookie.reply() {
                if !reply.value.is_empty() {
                    return String::from_utf8(reply.value).ok();
                }
            }
        }

        None
    }
}
