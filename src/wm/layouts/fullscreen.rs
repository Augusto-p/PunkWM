use super::super::manager::WorkspaceManager;
use x11rb::{connection::Connection, protocol::xproto::*};

impl WorkspaceManager {
    pub fn fullscreen(&self, windows: &Vec<Window>, width: u16, height: u16) {
        let panel_zone = self.panel.current_width();
        for &w in windows {
            let _ = self.conn.configure_window(
                w,
                &ConfigureWindowAux::new()
                    .x(panel_zone as i32)
                    .y(0)
                    .width(Some(width as u32))
                    .height(Some(height as u32))
                    .border_width(0),
            );
        }
        let _ = self.conn.flush();
    }
}
