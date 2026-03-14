use super::super::manager::WorkspaceManager;
use x11rb::{connection::Connection, protocol::xproto::*};

impl WorkspaceManager {
    pub fn tile_left(&self, windows: &Vec<Window>, width: u16, height: u16) {
        let n = windows.len();
        if n == 0 {
            return;
        }
        if n == 1 {
            self.max(windows, width, height);
            return;
        } else {
            let gap = self.vh(1, height);
            let n = windows.len();
            let master_w = (width as f32 * 0.6) as i32;
            let panel_zone = self.panel.current_width();
            for (i, &w) in windows.iter().enumerate() {
                if i == 0 {
                    let _ = self.conn.configure_window(
                        w,
                        &ConfigureWindowAux::new()
                            .x(Some((panel_zone + gap) as i32))
                            .y(Some((gap) as i32))
                            .width(Some((master_w - (gap as i32) * 2) as u32))
                            .height(Some((height- gap * 2) as u32)),
                    );
                } else {
                    let stack_n = (n - 1) as i32;
                    let stack_h = (height as i32 - (gap as i32) * (stack_n + 1)) / stack_n;
                    let y = (gap as i32) + (i as i32 - 1) * (stack_h + (gap as i32));

                    let _ = self.conn.configure_window(
                        w,
                        &ConfigureWindowAux::new()
                            .x(((panel_zone + gap) as i32) + master_w )
                            .y(y)
                            .width(Some((width as i32 - master_w - (gap as i32) * 2) as u32))
                            .height(Some(stack_h as u32)),
                    );
                }
            }

            let _ = self.conn.flush();
        }
    }

    pub fn tile_right(&self, windows: &Vec<Window>, width: u16, height: u16) {
        let n = windows.len();
        if n == 0 {
            return;
        }
        if n == 1 {
            self.max(windows, width, height);
            return;
        } else {
            let gap = self.vh(1, height);
            let n = windows.len();
            let master_w = (width as f32 * 0.6) as i32;
            let master_x = width as i32 - master_w + (gap as i32);
            let panel_zone = self.panel.current_width();
            for (i, &w) in windows.iter().enumerate() {
                if i == 0 {
                    let _ = self.conn.configure_window(
                        w,
                        &ConfigureWindowAux::new()
                            .x(((panel_zone - gap) as i32 ) + master_x)
                            .y(gap as i32)
                            .width(Some((master_w - (gap as i32) * 2) as u32))
                            .height(Some((height - gap * 2) as u32)),
                    );
                } else {
                    let stack_n = (n - 1) as i32;
                    let stack_h = (height as i32 - (gap as i32) * (stack_n + 1)) / stack_n;
                    let y = (gap as i32) + (i as i32 - 1) * (stack_h + (gap as i32));

                    let _ = self.conn.configure_window(
                        w,
                        &ConfigureWindowAux::new()
                            .x(Some(panel_zone as i32))
                            .y(y)
                            .width(Some(((width - gap * 2) as i32 - master_w ) as u32))
                            .height(Some(stack_h as u32)),
                    );
                }
            }

            let _ = self.conn.flush();
        }
    }

    pub fn tile_top(&self, windows: &Vec<Window>, width: u16, height: u16) {
        let n = windows.len();
        if n == 0 {
            return;
        }
        if n == 1 {
            self.max(windows, width, height);
            return;
        } else {
            let gap = self.vh(1, height);
            let n = windows.len();
            let master_h = (height as f32 * 0.6) as i32;
            let panel_zone = self.panel.current_width();
            for (i, &w) in windows.iter().enumerate() {
                if i == 0 {
                    let _ = self.conn.configure_window(
                        w,
                        &ConfigureWindowAux::new()
                            .x(Some(panel_zone as i32))
                            .y(gap as i32)
                            .width(Some((width - gap * 2) as u32))
                            .height(Some((master_h - 2*(gap as i32)) as u32)),
                    );
                } else {
                    let stack_n = (n - 1) as i32;
                    let stack_w = (((width-gap) as i32) * (stack_n + 1)) / stack_n;
                    let x = (gap as i32) + (i as i32 - 1) * (stack_w + (gap as i32));

                    let _ = self.conn.configure_window(
                        w,
                        &ConfigureWindowAux::new()
                            .x((panel_zone as i32) + x)
                            .y(master_h + (gap as i32))
                            .width(Some(stack_w as u32))
                            .height(Some(((height- gap * 2) as i32 - master_h ) as u32)),
                    );
                }
            }

            let _ = self.conn.flush();
        }
    }
    pub fn tile_bottom(&self, windows: &Vec<Window>, width: u16, height: u16) {
        let n = windows.len();
        if n == 0 {
            return;
        }
        if n == 1 {
            self.max(windows, width, height);
            return;
        } else {
            let gap = self.vh(1, height);
            let n = windows.len();
            let master_h = (height as f32 * 0.6) as i32;
            let master_y = height as i32 - master_h + (gap as i32);
            let panel_zone = self.panel.current_width();
            for (i, &w) in windows.iter().enumerate() {
                if i == 0 {
                    let _ = self.conn.configure_window(
                        w,
                        &ConfigureWindowAux::new()
                            .x(Some(panel_zone as i32))
                            .y(master_y - (gap as i32))
                            .width(Some((width - gap * 2) as u32))
                            .height(Some((master_h - 2*(gap as i32)) as u32)),
                    );
                } else {
                    let stack_n = (n - 1) as i32;
                    let stack_w = (((width - gap) as i32) * (stack_n + 1)) / stack_n;
                    let x = (gap as i32) + (i as i32 - 1) * (stack_w + (gap as i32));

                    let _ = self.conn.configure_window(
                        w,
                        &ConfigureWindowAux::new()
                            .x((panel_zone as i32) + x)
                            .y(gap as i32)
                            .width(Some(stack_w as u32))
                            .height(Some(((height - 3*gap) as i32 - master_h) as u32)),
                    );
                }
            }

            let _ = self.conn.flush();
        }
    }
}
