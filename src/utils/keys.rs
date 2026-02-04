use x11rb::protocol::xproto::ModMask;

/// Keycodes X11 comunes (layout US / LATAM)
pub fn keycode_from_name(name: &str) -> Option<u8> {
    match name.to_lowercase().as_str() {
        "a" => Some(38),
        "b" => Some(56),
        "c" => Some(54),
        "d" => Some(40),
        "e" => Some(26),
        "f" => Some(41),
        "g" => Some(42),
        "h" => Some(43),
        "i" => Some(31),
        "j" => Some(44),
        "k" => Some(45),
        "l" => Some(46),
        "m" => Some(58),
        "n" => Some(57),
        "o" => Some(32),
        "p" => Some(33),
        "q" => Some(24),
        "r" => Some(27),
        "s" => Some(39),
        "t" => Some(28),
        "u" => Some(30),
        "v" => Some(55),
        "w" => Some(25),
        "x" => Some(53),
        "y" => Some(29),
        "z" => Some(52),
        "enter" | "return" => Some(36),
        "space" => Some(65),
        "tab" => Some(23),
        _ => None,
    }
}

/// "alt" | "super" | "alt|shift"
pub fn parse_mods(s: &str) -> ModMask {
    let mut mask = ModMask::from(0u16);

    for part in s.split('|') {
        match part.to_lowercase().as_str() {
            "shift" => mask |= ModMask::SHIFT,
            "ctrl" | "control" => mask |= ModMask::CONTROL,
            "alt" | "mod1" => mask |= ModMask::M1,
            "super" | "mod4" => mask |= ModMask::M4,
            _ => {}
        }
    }

    mask
}
