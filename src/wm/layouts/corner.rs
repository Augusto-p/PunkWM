use super::super::manager::WorkspaceManager;
use x11rb::{connection::Connection, protocol::xproto::*};

impl WorkspaceManager {
    pub fn corner_nw(&self, windows: &Vec<Window>, width: u16, height: u16) {
        let n = windows.len();
        if n == 0 {
            return;
        }
        if n == 1 {
            self.max(windows, width, height);
            return;
        } else {
            let gap = self.vh(1, height);
            let x = gap as i32;
            let y = gap as i32;
            let w = (width - gap * 2) as i32;
            let h = (height - gap * 2) as i32;
            let panel_zone = self.panel.current_width();
            let master_w = (w as f32 * 0.75) as i32;
            let master_h = (h as f32 * 0.75) as i32;

            // 1. Master (NW) - Se queda en el 60% de alto
            let _ = self.conn.configure_window(
                windows[0],
                &ConfigureWindowAux::new()
                    .x(panel_zone as i32 + x)
                    .y(y)
                    .width(Some((master_w - gap as i32) as u32))
                    .height(Some((master_h - gap as i32) as u32)),
            );

            let mut right_stack = Vec::new();
            let mut bottom_stack = Vec::new();

            for i in 1..n {
                if i % 2 != 0 {
                    right_stack.push(windows[i]);
                } else {
                    bottom_stack.push(windows[i]);
                }
            }

            // 2. Stack Derecho (Full Height)
            let right_x = x + master_w + gap as i32;
            let right_w = w - master_w - gap as i32;
            if !right_stack.is_empty() && right_w > 0 {
                let count = right_stack.len() as i32;
                // CAMBIO: Ahora dividimos 'h' (altura total) en lugar de 'master_h'
                let rh = (h - (gap as i32) * (count - 1)) / count;

                for (i, win) in right_stack.iter().enumerate() {
                    let _ = self.conn.configure_window(
                        *win,
                        &ConfigureWindowAux::new()
                            .x(panel_zone as i32 + right_x)
                            .y(y + i as i32 * (rh + gap as i32))
                            .width(Some(right_w as u32))
                            .height(Some(rh.max(1) as u32)),
                    );
                }
            }

            // 3. Stack Inferior (Solo debajo de la Master)
            let bottom_y = y + master_h + gap as i32;
            let bottom_h = h - master_h - gap as i32;
            if !bottom_stack.is_empty() && bottom_h > 0 {
                let count = bottom_stack.len() as i32;
                // La anchura se calcula solo sobre 'master_w' para no solaparse con el stack derecho
                let bw = (master_w - (gap as i32) * (count - 1)) / count;

                for (i, win) in bottom_stack.iter().enumerate() {
                    let _ = self.conn.configure_window(
                        *win,
                        &ConfigureWindowAux::new()
                            .x(panel_zone as i32 + x + i as i32 * (bw + gap as i32))
                            .y(bottom_y)
                            .width(Some(bw.max(1) as u32))
                            .height(Some(bottom_h as u32)),
                    );
                }
            }

            let _ = self.conn.flush();
        }
    }

    pub fn corner_sw(&self, windows: &Vec<Window>, width: u16, height: u16) {
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
        let w = (width  - gap * 2) as i32;
        let h = (height - gap * 2) as i32;
        let master_w = (w as f32 * 0.75) as i32;
        let master_h = (h as f32 * 0.75) as i32;
        let panel_zone = self.panel.current_width();

        // Calculamos la posición Y para que la Master esté abajo
        let master_y = y + (h - master_h) + gap as i32;

        let mut right_stack = Vec::new();
        let mut top_stack = Vec::new(); // Cambiamos bottom por top

        for i in 1..n {
            if i % 2 != 0 {
                right_stack.push(windows[i]);
            } else {
                top_stack.push(windows[i]);
            }
        }

        // 1. Master (SW) - Ubicada abajo a la izquierda
        let _ = self.conn.configure_window(
            windows[0],
            &ConfigureWindowAux::new()
                .x(panel_zone as i32 + x)
                .y(master_y)
                .width(Some((master_w - gap as i32) as u32))
                .height(Some((master_h - gap as i32) as u32)),
        );

        // 2. Stack Derecho (Full Height) - Se mantiene igual
        let right_x = x + master_w + gap as i32;
        let right_w = w - master_w - gap as i32;
        if !right_stack.is_empty() && right_w > 0 {
            let count = right_stack.len() as i32;
            let rh = (h - (gap as i32) * (count - 1)) / count;

            for (i, win) in right_stack.iter().enumerate() {
                let _ = self.conn.configure_window(
                    *win,
                    &ConfigureWindowAux::new()
                        .x(panel_zone as i32 + right_x)
                        .y(y + i as i32 * (rh + gap as i32))
                        .width(Some(right_w as u32))
                        .height(Some(rh.max(1) as u32)),
                );
            }
        }

        // 3. Stack Superior (Encima de la Master)
        let top_h = h - master_h - gap as i32;
        if !top_stack.is_empty() && top_h > 0 {
            let count = top_stack.len() as i32;
            let bw = (master_w - (gap as i32) * (count - 1)) / count;

            for (i, win) in top_stack.iter().enumerate() {
                let _ = self.conn.configure_window(
                    *win,
                    &ConfigureWindowAux::new()
                        .x(panel_zone as i32 + x + i as i32 * (bw + gap as i32))
                        .y(y) // Empieza en el tope de la pantalla
                        .width(Some(bw.max(1) as u32))
                        .height(Some(top_h as u32)),
                );
            }
        }

        let _ = self.conn.flush();
    }

    pub fn corner_ne(&self, windows: &Vec<Window>, width: u16, height: u16) {
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
        let w = (width - gap * 2) as i32;
        let h = (height - gap * 2) as i32;
        let master_w = (w as f32 * 0.75) as i32;
        let master_h = (h as f32 * 0.75) as i32;
        let left_w = w - master_w - gap as i32;
        let master_x = x + left_w + gap as i32;
        let mut left_stack = Vec::new(); // Ventanas Impares (1, 3, 5...)
        let mut bottom_stack = Vec::new(); // Ventanas Pares (2, 4, 6...)
        let panel_zone = self.panel.current_width();

        for i in 1..n {
            if i % 2 != 0 {
                left_stack.push(windows[i]);
            } else {
                bottom_stack.push(windows[i]);
            }
        }

        // 1. Master (NE) - Superior Derecha
        let _ = self.conn.configure_window(
            windows[0],
            &ConfigureWindowAux::new()
                .x(panel_zone as i32 + master_x)
                .y(y)
                .width(Some((master_w - gap as i32) as u32))
                .height(Some((master_h - gap as i32) as u32)),
        );

        // 2. Stack Izquierdo (Full Height)
        if !left_stack.is_empty() && left_w > 0 {
            let count = left_stack.len() as i32;
            let lh = (h - (gap as i32) * (count - 1)) / count;

            for (i, win) in left_stack.iter().enumerate() {
                let _ = self.conn.configure_window(
                    *win,
                    &ConfigureWindowAux::new()
                        .x(panel_zone as i32 + x)
                        .y(y + i as i32 * (lh + gap as i32))
                        .width(Some(left_w as u32))
                        .height(Some(lh.max(1) as u32)),
                );
            }
        }

        // 3. Stack Inferior (Solo debajo de la Master)
        let bottom_y = y + master_h + gap as i32;
        let bottom_h = h - master_h - gap as i32;
        if !bottom_stack.is_empty() && bottom_h > 0 {
            let count = bottom_stack.len() as i32;
            let bw = (master_w - (gap as i32) * (count - 1)) / count;

            for (i, win) in bottom_stack.iter().enumerate() {
                let _ = self.conn.configure_window(
                    *win,
                    &ConfigureWindowAux::new()
                        .x(panel_zone as i32 + master_x + i as i32 * (bw + gap as i32))
                        .y(bottom_y)
                        .width(Some(bw.max(1) as u32))
                        .height(Some(bottom_h as u32)),
                );
            }
        }

        let _ = self.conn.flush();
    }

    pub fn corner_se(&self, windows: &Vec<Window>, width: u16, height: u16) {
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
        let w = (width - gap * 2) as i32;
        let h = (height - gap * 2) as i32;
        let master_w = (w as f32 * 0.75) as i32;
        let master_h = (h as f32 * 0.75) as i32;
        let left_w = w - master_w - gap as i32;
        let master_x = x + left_w + gap as i32;
        let master_y = y + (h - master_h) + gap as i32;
        let mut left_stack = Vec::new(); // Ventanas Impares (1, 3, 5...)
        let mut top_stack = Vec::new(); // Ventanas Pares (2, 4, 6...)

        let panel_zone = self.panel.current_width();
        for i in 1..n {
            if i % 2 != 0 {
                left_stack.push(windows[i]);
            } else {
                top_stack.push(windows[i]);
            }
        }

        // 1. Master (SE) - Inferior Derecha
        let _ = self.conn.configure_window(
            windows[0],
            &ConfigureWindowAux::new()
                .x(panel_zone as i32 + master_x)
                .y(master_y)
                .width(Some((master_w - gap as i32) as u32))
                .height(Some((master_h - gap as i32) as u32)),
        );

        // 2. Stack Izquierdo (Full Height)
        if !left_stack.is_empty() && left_w > 0 {
            let count = left_stack.len() as i32;
            let lh = (h - (gap as i32) * (count - 1)) / count;

            for (i, win) in left_stack.iter().enumerate() {
                let _ = self.conn.configure_window(
                    *win,
                    &ConfigureWindowAux::new()
                        .x(panel_zone as i32 + x)
                        .y(y + i as i32 * (lh + gap as i32))
                        .width(Some(left_w as u32))
                        .height(Some(lh.max(1) as u32)),
                );
            }
        }

        // 3. Stack Superior (Solo encima de la Master)
        let top_h = h - master_h - gap as i32;
        if !top_stack.is_empty() && top_h > 0 {
            let count = top_stack.len() as i32;
            let bw = (master_w - (gap as i32) * (count - 1)) / count;

            for (i, win) in top_stack.iter().enumerate() {
                let _ = self.conn.configure_window(
                    *win,
                    &ConfigureWindowAux::new()
                        .x(panel_zone as i32 + master_x + i as i32 * (bw + gap as i32))
                        .y(y)
                        .width(Some(bw.max(1) as u32))
                        .height(Some(top_h as u32)),
                );
            }
        }

        let _ = self.conn.flush();
    }
}
