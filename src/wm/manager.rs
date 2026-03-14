use x11rb::rust_connection::RustConnection;
use x11rb::connection::Connection;
use x11rb::protocol::xproto::Window;
use crate::wm::dock::Dock;
use crate::wm::panel::PanelOptions;
use super::layout::Layout;
use crate::wm::lock::Lock;



pub struct WorkspaceManager {
    pub conn: RustConnection,
    pub current: usize,
    pub workspaces: Vec<Vec<Window>>,
    pub focused: Vec<Option<usize>>,
    pub layouts: Vec<Layout>,
    pub dock: Option<Dock>, // 👈 nuevo atributo
    pub lock: Option<Lock>,
    pub panel: PanelOptions
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
            lock: None,
            panel: panel
        }
    }
    // pub fn conn(&self) -> &RustConnection {
    //     &self.conn
    // }
    // pub fn conn_mut(&mut self) -> &mut RustConnection {
    //     &mut self.conn
    // }

    pub fn init_dock(&mut self, width: u16, height: u16, path: String) {
        let screen = &self.conn.setup().roots[0];
        self.panel.set_current_width(width + screen.height_in_pixels/50);

        if self.dock.is_some() {
            return;
        }
        let dock = Dock::new(width, height, self.panel.width());
        let dock_name = dock.clone();
        dock.make(path);          // lanza el proceso
        self.dock = Some(dock); // guarda el estado

        let lock = Lock::new(dock_name.name());
        self.lock = Some(lock);
    }
    
}



