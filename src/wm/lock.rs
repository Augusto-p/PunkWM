use x11rb::{
    connection::Connection,
    protocol::xproto::{ConfigureWindowAux, ConnectionExt, Window},
    rust_connection::RustConnection,
};


#[derive(Clone)]
pub struct Lock {
    window: Option<Window>,
    name: String,
    running: bool
}

impl Lock{
    pub fn new(name: &str) -> Self {
        Self {
            running: false,
            name: format!("Lock_{}", name),
            window: None,
        }
    }


     pub fn manage(&mut self, win: Window, conn: &RustConnection) {
        self.window = Some(win);
        self.running = true;
        let _ = conn.map_window(win);
        let screen = &conn.setup().roots[0];


        let _ = conn.configure_window(
            win,
            &ConfigureWindowAux::new()
                .x(0)
                .y(0)
                .width(screen.width_in_pixels as u32 )
                .height(screen.height_in_pixels as u32),
        );
        let _ = conn.flush();
    }

    pub fn close(&mut self){
        self.running = false;
        self.window = None;
        
    }

    pub fn running(&self)->&bool{
        &self.running
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}