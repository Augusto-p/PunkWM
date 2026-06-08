use crate::config::config::GLOBAL_CFG;
use serde::{Deserialize, Serialize};
use std::{
    io::{SeekFrom,Seek,Write,Read},
    fs::{OpenOptions,File},
};
pub struct LayoutsManager;

impl LayoutsManager{
    
    pub fn save(pos:u8, layout:LayoutType)->bool{
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
            archivo.seek(SeekFrom::Start(pos.into()))?;
            archivo.write_all(&[layout.id()])?;
            Ok(())
        };

        // Si el proceso interno fue exitoso, devolvemos true
        proceso().is_ok()
    }

    pub fn get(pos:u8)->LayoutType{
        let cfg = GLOBAL_CFG.read().expect("No se pudo bloquear para lectura");
        let mut archivo = match OpenOptions::new().read(true).open(format!("{}/layouts.bin",cfg.folder())) {
            Ok(file) => file,
            Err(_) => return LayoutType::from_u8(0),
        };
        // Intentamos mover el cursor y leer
        let mut buffer = [0u8; 1];
        let mut proceso = || -> std::io::Result<()> {
            archivo.seek(SeekFrom::Start(pos.into()))?;
            archivo.read_exact(&mut buffer)?;
            Ok(())
        };

        if proceso().is_ok() {
            LayoutType::from_u8(buffer[0])
        } else {
            LayoutType::from_u8(0)
        }
    }

    pub fn load()->Vec<LayoutType>{
        let cfg = GLOBAL_CFG.read().expect("No se pudo bloquear para lectura");
        let mut buffer = vec![0u8; 9];
        if let Ok(mut file) = File::open(format!("{}/layouts.bin",cfg.folder())) {
            if file.read_exact(&mut buffer).is_ok() {
                return buffer
                    .into_iter()
                    .map(|b| LayoutType::from_u8(b))
                    .collect();
            }
        }

        vec![LayoutType::from_u8(0); 9]
    }

}

 
#[derive(Clone, Copy, Serialize, Deserialize, Debug,PartialEq)]
pub enum LayoutType {
    // Corner layouts
    Max = 0,        // READY

    TileLeft,       // READY
    TileTop,        // READY
    TileRight,      // READY
    TileBottom,     // READY

    CornerNE,
    CornerNW,
    CornerSE,
    CornerSW,
    // Tile variants
    // Grid / fair

    FairH,          // READY
    FairV,          // READY

    // Special
    Fullscreen,
    Dwindle,
    Magnifier,
    Spiral,
    Floating

}

impl LayoutType {
    pub fn id(self) -> u8 {
        self as u8
    }

    pub fn from_u8(i: u8) -> Self {
        match i%15 {
            0 => Self::Max,
            1 => Self::TileLeft,
            2 => Self::TileTop,
            3 => Self::TileRight,
            4 => Self::TileBottom,
            
            5 => Self::CornerNW,
            6 => Self::CornerNE,
            7 => Self::CornerSE,
            8 => Self::CornerSW,
          
            9 => Self::FairH,
            10 => Self::FairV,

            11 => Self::Fullscreen,
            12 => Self::Dwindle,
            13 => Self::Magnifier,
            14 => Self::Spiral,
            _ => Self::Max,
        }
    }

    pub fn next(self)-> Self{
        let a = Self::from_u8(self.id()+1);
        println!("{:?}",a);
        a

    }

    


}