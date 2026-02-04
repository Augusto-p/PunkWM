use x11rb::{
    connection::Connection,
    protocol::xproto::{ConfigureWindowAux, ConnectionExt, Window},
    rust_connection::RustConnection,
};

use crate::utils::{ random::random_string, tools::spawn};


pub struct Dock {
    window: Option<Window>,
    width: u16,
    height: u16,
    panel_width: u16,
    name: String,
}

impl Dock {
    pub fn new(width: u16, height: u16, panel_width:u16) -> Self {
        Self {
            width: width,
            height: height,
            panel_width: panel_width,
            name: format!("Dock_{}", random_string(32)),
            window: None,
        }
    }

    pub fn make(&self) {
        let gap = self.vh(1, self.height);
        let command = format!(
            "/home/augus/punk_wm/dock --width={} --height={} --title=\"{}\"",
            self.width, (self.height as u32 - 2*gap), self.name
        );
        spawn(&command);
    }


    pub fn name(&self) -> &str {
        &self.name
    }


    pub fn panel_open(&mut self, conn: &RustConnection) {
        if let Some(win) = self.window {
            let width = self.width + self.panel_width;
            let gap = self.vh(1, self.height);
            let _ = conn.configure_window(
                win,
                &ConfigureWindowAux::new()
                    .width(width as u32)
                    .height(self.height as u32 - 2*gap)
                    .x(gap as i32)
                    .y(gap as i32),
            );

            let _ = conn.flush();
        }
    }

    pub fn panel_close(&mut self, conn: &RustConnection) {
        if let Some(win) = self.window {
             let gap = self.vh(1, self.height);
            let _ = conn.configure_window(
                win,
                &ConfigureWindowAux::new()
                    .width(self.width as u32)
                    .height(self.height as u32- 2*gap)
                    .x(gap as i32)
                    .y(gap as i32),
            );

            let _ = conn.flush();
        }
    }

    pub fn manage(&mut self, win: Window, conn: &RustConnection) {
        self.window = Some(win);
        let _ = conn.map_window(win);

        let gap = self.vh(1, self.height);

        let _ = conn.configure_window(
            win,
            &ConfigureWindowAux::new()
                .x(gap as i32)
                .y(gap as i32)
                .width(self.width as u32)
                .height(self.height as u32- 2*gap),
        );
        let _ = conn.flush();
    }

    pub fn width(&self) -> u16{
        self.width

    }

    pub fn vh(&self, count: u16, screen_height: u16) -> u32 {
        (count * screen_height / 100) as u32
    }
}
