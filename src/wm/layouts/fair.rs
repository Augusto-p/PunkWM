use super::super::manager::WorkspaceManager;
use x11rb::{connection::Connection, protocol::xproto::*};

impl WorkspaceManager {
    pub fn fair_h(&self, windows: &Vec<Window>, width: u16, height: u16) {
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
        let w = (width -2 * gap) as i32;
        let h = (height -2 * gap) as i32;
        let mut top_stack = Vec::new(); // Índices 0, 2, 4...
        let mut bottom_stack = Vec::new(); // Índices 1, 3, 5...
        let panel_zone = self.panel.current_width();
        for i in 0..n {
            if i % 2 == 0 {
                top_stack.push(windows[i]);
            } else {
                bottom_stack.push(windows[i]);
            }
        }

        let half_h = (h - (gap as i32)) * 7 / 10;

        // 1. Fila Superior (Top)
        if !top_stack.is_empty() {
            let count = top_stack.len() as i32;
            let win_w = (w - (gap as i32) * (count - 1)) / count;

            for (i, win) in top_stack.iter().enumerate() {
                let _ = self.conn.configure_window(
                    *win,
                    &ConfigureWindowAux::new()
                        .x(panel_zone as i32 + x + i as i32  * (win_w + (gap as i32)))
                        .y(y)
                        .width(Some(win_w.max(1) as u32))
                        .height(Some(half_h as u32)),
                );
            }
        }

        // 2. Fila Inferior (Bottom)
        if !bottom_stack.is_empty() {
            let count = bottom_stack.len() as i32;
            let win_w = (w - (gap as i32) * (count - 1)) / count;
            let bottom_y = y + half_h + (gap as i32);

            for (i, win) in bottom_stack.iter().enumerate() {
                let _ = self.conn.configure_window(
                    *win,
                    &ConfigureWindowAux::new()
                        .x(panel_zone as i32 + x + i as i32 * (win_w + (gap as i32)))
                        .y(bottom_y)
                        .width(Some(win_w.max(1) as u32))
                        .height(Some(half_h as u32)),
                );
            }
        }

        let _ = self.conn.flush();
    }
    pub fn fair_v(&self, windows: &Vec<Window>, width: u16, height: u16) {
        let n = windows.len();
        if n == 0 {
            return;
        }

        // Si solo hay una ventana, usa el método max que ya tienes definido
        if n == 1 {
            self.max(windows, width, height);
            return;
        }

        let gap = self.vh(1, height);
        let x = gap as i32;
        let y = gap as i32;
        let w = (width -2 * gap) as i32;
        let h = (height -2 * gap) as i32;
        let mut left_stack = Vec::new();
        let mut right_stack = Vec::new();
        let panel_zone = self.panel.current_width();

        for i in 0..n {
            if i % 2 == 0 {
                left_stack.push(windows[i]);
            } else {
                right_stack.push(windows[i]);
            }
        }

        let half_w = (w - (gap as i32)) * 7 / 10;

        // 1. Columna Izquierda (Índices 0, 2, 4...)
        if !left_stack.is_empty() {
            let count = left_stack.len() as i32;
            let win_h = (h - (gap as i32) * (count - 1)) / count;

            for (i, win) in left_stack.iter().enumerate() {
                let _ = self.conn.configure_window(
                    *win,
                    &ConfigureWindowAux::new()
                        .x(panel_zone as i32 + x)
                        .y(y + i as i32 * (win_h + gap as i32))
                        .width(Some(half_w as u32))
                        .height(Some(win_h.max(1) as u32)),
                );
            }
        }

        // 2. Columna Derecha (Índices 1, 3, 5...)
        if !right_stack.is_empty() {
            let count = right_stack.len() as i32;
            let win_h = (h - (gap as i32) * (count - 1)) / count;
            let right_x = x + half_w + gap as i32;

            for (i, win) in right_stack.iter().enumerate() {
                let _ = self.conn.configure_window(
                    *win,
                    &ConfigureWindowAux::new()
                        .x(panel_zone as i32 + right_x)
                        .y(y + i as i32 * (win_h + gap as i32))
                        .width(Some(half_w as u32))
                        .height(Some(win_h.max(1) as u32)),
                );
            }
        }

        let _ = self.conn.flush();
    }
}
