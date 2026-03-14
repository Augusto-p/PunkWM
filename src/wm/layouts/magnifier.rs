use super::super::manager::WorkspaceManager;
use x11rb::{connection::Connection, protocol::xproto::*};

impl WorkspaceManager {
    pub fn magnifier(&self, windows: &Vec<Window>, width: u16, height: u16) {
        let n = windows.len();
        if n == 0 {
            return;
        }

        if n == 1 {
            self.max(windows, width, height);
            return;
        }

        let gap = self.vh(1, height);
        let x = gap as i32;
        let y = gap as i32;
        let w = (width - 2 * gap) as i32;
        let h = (height - 2 * gap) as i32;
        let panel_zone = self.panel.current_width();

        if n > 1 {
            let count = n - 1;
            let win_h = (h - (gap as i32) * (count as i32 - 1)) / count as i32;

            for i in 1..n {
                let _ = self.conn.configure_window(
                    windows[i],
                    &ConfigureWindowAux::new()
                        .x(panel_zone as i32 + x) // En este caso, el fondo ocupa todo el ancho o se queda atrás
                        .y(y + (i - 1) as i32 * (win_h + gap as i32))
                        .width(Some(w as u32))
                        .height(Some(win_h.max(1) as u32)),
                );
            }
        }

        // 2. La ventana "Magnifier" (Ventana 0) se superpone en el centro
        // Usualmente ocupa un 70-80% del centro
        let mag_w = (w as f32 * 0.70) as i32;
        let mag_h = (h as f32 * 0.70) as i32;
        let mag_x = x + (w - mag_w) / 2;
        let mag_y = y + (h - mag_h) / 2;

        let _ = self.conn.configure_window(
            windows[0],
            &ConfigureWindowAux::new()
                .x(panel_zone as i32 + mag_x)
                .y(mag_y)
                .width(Some(mag_w as u32))
                .height(Some(mag_h as u32))
                // En X11, para que esté encima, podrías necesitar cambiar el stack_mode
                .stack_mode(Some(StackMode::ABOVE)),
        );

        let _ = self.conn.flush();
    }
}
