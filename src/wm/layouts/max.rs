use super::super::manager::WorkspaceManager;
use x11rb::{connection::Connection, protocol::xproto::*};

impl WorkspaceManager {
    pub fn max(&self, windows: &Vec<Window>, width: u16, height: u16) {
        let gap = self.vh(1,height);
        let panel_zone = self.panel.current_width();
        for &w in windows {
            let _ = self.conn.configure_window(
                w,
                &ConfigureWindowAux::new()
                    .x(Some(panel_zone as i32))
                    .y(gap as i32)
                    .width(Some((width - gap ) as u32))
                    .height(Some((height) as u32)),
            );
        }

        let _ = self.conn.flush();
    }
}
