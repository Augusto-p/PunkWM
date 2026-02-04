use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};

use super::{layout::Layout, manager::WorkspaceManager};
use crate::utils::config::path_persistence;

impl WorkspaceManager {
    // ============================================================
    // 🔹 GUARDAR LAYOUTS (persistencia.bin)
    // ============================================================
    pub fn save_layouts(&self) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path_persistence())?;

        let data = bincode::serialize(&self.layouts)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        file.write_all(&data)?;
        Ok(())
    }

    // ============================================================
    // 🔹 CARGAR LAYOUTS (se llama AL INICIAR EL WM)
    // ============================================================
    pub fn load_layouts(&mut self) -> io::Result<()> {
        let mut file = match File::open(path_persistence()) {
            Ok(f) => f,
            Err(_) => return Ok(()), // no existe aún
        };

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let layouts: Vec<Layout> =
            bincode::deserialize(&buffer).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        // seguridad: solo si coincide la cantidad de workspaces
        if layouts.len() == self.layouts.len() {
            self.layouts = layouts;
        }

        Ok(())
    }
}
