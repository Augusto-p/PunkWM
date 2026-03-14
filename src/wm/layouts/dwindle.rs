use super::super::manager::WorkspaceManager;
use x11rb::{connection::Connection, protocol::xproto::*};

impl WorkspaceManager {
    pub fn dwindle(&self, windows: &Vec<Window>, width: u16, height: u16) {
        let n = windows.len();
        if n == 0 {
            return;
        }
        if n == 1 {
            self.max(windows, width, height);
            return;
        } else {
            let gap = self.vh(1, height);

            // área disponible
            let mut x: i32 = gap as i32;
            let mut y: i32 = gap as i32;
            let mut w: i32 = (width - gap  * 4) as i32;
            let mut h: i32 = (height - gap * 4) as i32;
            let panel_zone = self.panel.current_width();
            for (i, &win) in windows.iter().enumerate() {
                if w <= 0 || h <= 0 {
                    break;
                }

                // tamaño de la ventana actual
                let (wx, wy, ww, wh);

                if i % 2 == 0 {
                    // split vertical
                    let cw = w / 2;
                    wx = x;
                    wy = y;
                    ww = cw - gap as i32;
                    wh = h;

                    // actualizar área libre
                    x += cw + gap as i32;
                    w -= cw + gap as i32;
                } else {
                    // split horizontal
                    let ch = h / 2;
                    wx = x;
                    wy = y;
                    ww = w;
                    wh = ch - gap as i32;

                    // actualizar área libre
                    y += ch + gap as i32;
                    h -= ch + gap as i32;
                }

                let _ = self.conn.configure_window(
                    win,
                    &ConfigureWindowAux::new()
                        .x(panel_zone as i32 + wx)
                        .y(wy)
                        .width(Some(ww.max(1) as u32))
                        .height(Some(wh.max(1) as u32)),
                );
            }

            let _ = self.conn.flush();
        }
    }
}
