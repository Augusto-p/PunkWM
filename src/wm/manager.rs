use x11rb::rust_connection::RustConnection;
use x11rb::connection::Connection;
use x11rb::protocol::xproto::Window;
use crate::wm::dock::Dock;
use super::layout::Layout;

pub struct PanelOptions {
    open: bool,
    width: u16,
    current_width: u16,
}

impl PanelOptions {
    /// Constructor
    pub fn new(width: u16) -> Self {
        Self {
            open: false,
            width,
            current_width: 0,
        }
    }

    // -------- getters --------
    pub fn is_open(&self) -> bool {
        self.open
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn current_width(&self) -> u16 {
        self.current_width
    }

    // -------- setters --------
    pub fn open(&mut self) {
        if !self.open{
            self.current_width = self.current_width.saturating_add(self.width);
            self.open = true;
        }
    }
    pub fn close(&mut self) {
        if self.open{
            self.current_width = self.current_width.saturating_sub(self.width);
            self.open = false;
        }
    }
    pub fn set_current_width(&mut self, current_width: u16) {
        self.current_width = current_width;
    }


}



pub struct WorkspaceManager {
    pub conn: RustConnection,
    pub current: usize,
    pub workspaces: Vec<Vec<Window>>,
    pub focused: Vec<Option<usize>>,
    pub layouts: Vec<Layout>,
    pub dock: Option<Dock>, // 👈 nuevo atributo
    pub panel: PanelOptions,
}

impl WorkspaceManager {
    pub fn new(conn: RustConnection, panel_width: u16) -> Self {
        let panel = PanelOptions::new(panel_width);


        Self {
            conn,
            current: 0,
            workspaces: vec![Vec::new(); 9],
            focused: vec![None; 9],
            layouts: vec![Layout::TileRight; 9],
            dock: None,
            panel: panel
        }
    }
    // pub fn conn(&self) -> &RustConnection {
    //     &self.conn
    // }
    // pub fn conn_mut(&mut self) -> &mut RustConnection {
    //     &mut self.conn
    // }

    pub fn init_dock(&mut self, width: u16, height: u16) {
        let screen = &self.conn.setup().roots[0];
        self.panel.set_current_width(width + screen.height_in_pixels/50);

        if self.dock.is_some() {
            return;
        }
        let dock = Dock::new(width, height, self.panel.width);
        dock.make();          // lanza el proceso
        self.dock = Some(dock); // guarda el estado
    }
    
}



