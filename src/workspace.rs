
use std::{
    io::{SeekFrom,Seek,Write,Read},
    fs::OpenOptions,
};
use crate::shell::WindowElement;

use smithay::{desktop::{Space,}};
use crate::config::config::GLOBAL_CFG;
use crate::layout::LayoutType;
use crate::dock::PunkDock;
use crate::state::SpaceExt;

#[derive(Debug)]
pub struct Workspace{
    pub id: u8,
    pub space: Space<WindowElement>,
    pub layout: LayoutType,
    pub windows: Vec<WindowElement>,
}


impl Workspace{
    pub fn id(&self)->u8{
        return self.id.clone();
    }
    pub fn new(id:u8)->Self{
         let mut novo = Self{
            id,
            space: Space::default(),
            windows: Vec::new(),
            layout:LayoutType::Max,
            //  dock:'a dock

        };
        novo.load_layout();
        return novo;
    }

    pub fn load_layout(&mut self)->bool{
        let cfg = GLOBAL_CFG.read().expect("No se pudo bloquear para lectura");
        let mut archivo = match OpenOptions::new().read(true).open(format!("{}/layouts.bin",cfg.folder())) {
            Ok(file) => file,
            Err(_) => return false,
        };
        // Intentamos mover el cursor y leer
        let mut buffer = [0u8; 1];
        let mut proceso = || -> std::io::Result<()> {
            archivo.seek(SeekFrom::Start(self.id.into()))?;
            archivo.read_exact(&mut buffer)?;
            Ok(())
        };

        if proceso().is_ok() {
            self.layout = LayoutType::from_u8(buffer[0]); // Guardamos el byte leído
            true
        } else {
            false
        }
    }

    pub fn next_layout(&mut self)->bool{
        println!("netx");
        self.layout = self.layout.next();
        let cfg = GLOBAL_CFG.read().expect("No se pudo bloquear para lectura");


        let mut archivo = match OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(format!("{}/layouts.bin",cfg.folder())) {
                Ok(file) => file,
                Err(_) => return false, // Si no se puede abrir, devolvemos false
        };

        // Intentamos mover el cursor y escribir el byte
        // Usamos el operador ? de forma interna o un bloque que maneje el error
        let mut proceso = || -> std::io::Result<()> {
            archivo.seek(SeekFrom::Start(self.id.into()))?;
            archivo.write_all(&[self.layout.id()])?;
            Ok(())
        };
        
        // Si el proceso interno fue exitoso, devolvemos true
        proceso().is_ok()
    }

    pub fn arrange(&mut self, dock: &mut PunkDock) {
        
        match self.layout {
            LayoutType::Max => self.arrange_max(dock),
            LayoutType::TileLeft => self.arrange_tile_left(dock),
            LayoutType::TileRight => self.arrange_tile_right(dock),
            LayoutType::TileTop => self.arrange_tile_top(dock),
            LayoutType::TileBottom => self.arrange_tile_bottom(dock),
            LayoutType::FairH => self.arrange_fair_h(dock),
            LayoutType::FairV => self.arrange_fair_v(dock),
            LayoutType::CornerNW => self.arrange_corner_nw(dock),
            LayoutType::CornerNE => self.arrange_corner_ne(dock),
            LayoutType::CornerSW => self.arrange_corner_sw(dock),
            LayoutType::CornerSE => self.arrange_corner_se(dock),
            _ => self.arrange_corner_se(dock),
        }
        
    }

    pub fn arrange_dock(&mut self, dock: &mut PunkDock) {
        let margin = self.space.vh();
        let output = self.space.outputs().next().cloned();
        let Some(output_geo) = output.and_then(|o| self.space.output_geometry(&o)) else {
            return;
        };

        // Usamos un patrón idiomático de Rust (if let) para desempaquetar de forma segura
        if let Some(dock_win) = &dock.window {
            let dock_x = output_geo.loc.x + margin;
            let dock_y = output_geo.loc.y + margin;          
            let dock_h = output_geo.size.h - (2 * margin);
            self.map_window(dock_win.clone(), dock_x, dock_y, dock.size(), dock_h);
        }
    }

    pub fn map_window(&mut self, win: WindowElement, x: i32, y:i32, w:i32, h:i32){
        self.space.map_element(win.clone(), (x, y), true);
        if let Some(toplevel) = win.window.toplevel() {
            let mut size_changed = false;
            toplevel.with_pending_state(|state| {
                let new_size = Some((w, h).into());
                if state.size != new_size {
                    state.size = new_size;
                    size_changed = true;
                }
            });
            if size_changed {
                toplevel.send_configure();
            }
        }            
    }

    pub fn arrange_lock_window(&mut self, window: Option<WindowElement>){
        println!("{:?}", window);
        if let Some(win) = window {   
            let output = self.space.outputs().next().cloned();
            let Some(output_geo) = output.and_then(|o| self.space.output_geometry(&o)) else { return; };
            
            let master_x = output_geo.loc.x;
            let master_y = output_geo.loc.y;
            let total_w = output_geo.size.w;
            let total_h = output_geo.size.h;
            self.map_window(win.clone(), master_x, master_y, total_w, total_h);
        }

    }

}

