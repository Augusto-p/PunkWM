use super::super::manager::WorkspaceManager;
use x11rb::{connection::Connection, protocol::xproto::*};

impl WorkspaceManager {
    pub fn spiral(&self, windows: &Vec<Window>, width: u16, height: u16) {
        let n = windows.len();
        if n == 0 {
            return;
        }
        if n == 1 {
            self.max(windows, width, height);
            return;
        }
        let gap = self.vh(1,height);
        let phi = (1.0 + 5.0_f32.sqrt()) / 2.0;
        let mut x = gap as f32;
        let mut y = gap as f32;
        let mut w = (width - gap  * 2) as f32;
        let mut h = (height - gap * 2) as f32;
        let panel_zone = self.panel.current_width();

        for i in 0..n {
            let last = i == n - 1;
            let dir = i % 4;

            // Variables para la ventana actual
            let (win_x, win_y, win_w, win_h);

            if last {
                // La última ventana llena el hueco restante
                win_x = x;
                win_y = y;
                win_w = w;
                win_h = h;
            } else {
                match dir {
                    0 => {
                        // Derecha (Corta ancho, mueve X)
                        win_x = x;
                        win_y = y;
                        win_w = w / phi;
                        win_h = h;
                        x += win_w;
                        w -= win_w;
                    }
                    1 => {
                        // Abajo (Corta alto, mueve Y)
                        win_x = x;
                        win_y = y;
                        win_w = w;
                        win_h = h / phi;
                        y += win_h;
                        h -= win_h;
                    }
                    2 => {
                        // Izquierda (Corta ancho, la ventana va al final del espacio actual)
                        win_w = w / phi;
                        win_h = h;
                        win_x = x + w - win_w;
                        win_y = y;
                        w -= win_w;
                    }
                    3 => {
                        // Arriba (Corta alto, la ventana va al final del espacio actual)
                        win_w = w;
                        win_h = h / phi;
                        win_x = x;
                        win_y = y + h - win_h;
                        h -= win_h;
                    }
                    _ => unreachable!(),
                }
            }

            // Aplicamos la configuración con gaps
            let _ = self.conn.configure_window(
                windows[i],
                &ConfigureWindowAux::new()
                    .x(panel_zone as i32 + win_x as i32)
                    .y(win_y as i32)
                    .width(Some((win_w as i32 - gap as i32).max(1) as u32))
                    .height(Some((win_h as i32 - gap as i32).max(1) as u32)),
            );
        }

        let _ = self.conn.flush();
    }
}
