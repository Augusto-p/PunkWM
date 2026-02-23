use x11rb::protocol::xproto::ModMask;
use x11rb::protocol::xproto::{ConnectionExt};
use x11rb::rust_connection::RustConnection;
use x11rb::connection::Connection; 
use crate::utils::config::Config;

#[derive(Clone)]
pub struct Binding {
    pub modifier: ModMask,
    pub keycode: u8,
    pub action: String,
}


// Si la función está fuera del struct:
pub(crate) fn parse_bindings(conn: &RustConnection, cfg: &Config) -> Vec<Binding> {
    let mut bindings = Vec::new();

    for (k, action) in &cfg.keybindings {
        let parts: Vec<&str> = k.split('+').collect();
        if parts.len() < 2 { continue; }

        let key_name = parts.last().unwrap();
        let mod_parts = &parts[..parts.len() - 1];

        let modifier = parse_mods(mod_parts);
        
        // Pasamos la referencia de la conexión
        if let Some(code) = keycode_from_name(conn, key_name) {
            bindings.push(Binding {
                modifier,
                keycode: code,
                action: action.clone(),
            });
        }
    }
    bindings
}

// La función que causaba el error ahora con el trait correcto
fn keycode_from_name(conn: &impl Connection, name: &str) -> Option<u8> {
    let target_keysym = string_to_keysym(name)?;

    let setup = conn.setup();
    let min = setup.min_keycode;
    let max = setup.max_keycode;

    // get_keyboard_mapping requiere que ConnectionExt esté en el scope
    let reply = conn.get_keyboard_mapping(min, max - min + 1).ok()?.reply().ok()?;
    
    let keysyms_per_code = reply.keysyms_per_keycode as usize;
    for (idx, chunk) in reply.keysyms.chunks(keysyms_per_code).enumerate() {
        if chunk.contains(&target_keysym) {
            return Some(min + idx as u8);
        }
    }
    None
}

fn parse_mods(mods: &[&str]) -> ModMask {
    let mut mask: u16 = 0;

    for m in mods {
        match m.to_uppercase().as_str() {
            "CTRL"  => mask |= u16::from(ModMask::CONTROL),
            "SHIFT" => mask |= u16::from(ModMask::SHIFT),
            "ALT"   => mask |= u16::from(ModMask::M1), // Alt suele ser Mod1
            "SUPER" | "WIN" => mask |= u16::from(ModMask::M4), // Super suele ser Mod4
            _ => {}
        }
    }

    ModMask::from(mask)
}

fn string_to_keysym(name: &str) -> Option<u32> {
    match name.to_uppercase().as_str() {
        // Letras (X11 usa valores ASCII para minúsculas como base)
        "A" => Some(0x61), "B" => Some(0x62), "C" => Some(0x63), "D" => Some(0x64),
        "E" => Some(0x65), "F" => Some(0x66), "G" => Some(0x67), "H" => Some(0x68),
        "I" => Some(0x69), "J" => Some(0x6a), "K" => Some(0x6b), "L" => Some(0x6c),
        "M" => Some(0x6d), "N" => Some(0x6e), "O" => Some(0x6f), "P" => Some(0x70),
        "Q" => Some(0x71), "R" => Some(0x72), "S" => Some(0x73), "T" => Some(0x74),
        "U" => Some(0x75), "V" => Some(0x76), "W" => Some(0x77), "X" => Some(0x78),
        "Y" => Some(0x79), "Z" => Some(0x7a),

        // Números
        "0" => Some(0x30), "1" => Some(0x31), "2" => Some(0x32), "3" => Some(0x33),
        "4" => Some(0x34), "5" => Some(0x35), "6" => Some(0x36), "7" => Some(0x37),
        "8" => Some(0x38), "9" => Some(0x39),

        // Teclas Especiales (F-Keys)
        "F1" => Some(0xFFBE), "F2" => Some(0xFFBF), "F3" => Some(0xFFC0),
        "F4" => Some(0xFFC1), "F5" => Some(0xFFC2), "F6" => Some(0xFFC3),
        "F7" => Some(0xFFC4), "F8" => Some(0xFFC5), "F9" => Some(0xFFC6),
        "F10" => Some(0xFFC7), "F11" => Some(0xFFC8), "F12" => Some(0xFFC9),

        // Navegación y Control
        "RETURN" | "ENTER" => Some(0xFF0D),
        "ESCAPE" | "ESC"    => Some(0xFF1B),
        "SPACE"            => Some(0x0020),
        "BACKSPACE"        => Some(0xFF08),
        "TAB"              => Some(0xFF09),
        "LEFT"             => Some(0xFF51),
        "UP"               => Some(0xFF52),
        "RIGHT"            => Some(0xFF53),
        "DOWN"             => Some(0xFF54),

        _ => None,
    }
}