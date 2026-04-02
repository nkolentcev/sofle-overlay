/// Decode a 16-bit QMK keycode to a human-readable label.
pub fn decode_keycode(kc: u16) -> String {
    if kc == 0x0000 {
        return String::new();
    }
    if kc == 0x0001 {
        return "\u{25BD}".into(); // ▽ transparent
    }

    // Basic keycodes
    if kc <= 0x00FF {
        return basic_kc_name(kc as u8).into();
    }

    // Shifted keycodes (0x02xx)
    if (kc & 0xFF00) == 0x0200 {
        let base = (kc & 0xFF) as u8;
        return shift_symbol(base)
            .map(String::from)
            .unwrap_or_else(|| format!("S({})", basic_kc_name(base)));
    }

    // Modifier combos (0x01xx-0x1Fxx)
    if (0x0100..=0x1FFF).contains(&kc) {
        let mods = ((kc >> 8) & 0x1F) as u8;
        let base = (kc & 0xFF) as u8;
        return format!("{}-{}", mod_prefix(mods), basic_kc_name(base));
    }

    // MO(layer)
    if (kc & 0xFF00) == 0x5100 {
        return format!("MO({})", kc & 0xFF);
    }
    // DF(layer)
    if (kc & 0xFF00) == 0x5200 {
        return format!("DF({})", kc & 0xFF);
    }
    // TG(layer)
    if (kc & 0xFF00) == 0x5300 {
        return format!("TG({})", kc & 0xFF);
    }
    // OSL(layer)
    if (kc & 0xFF00) == 0x5400 {
        return format!("OSL({})", kc & 0xFF);
    }
    // TT(layer)
    if (kc & 0xFF00) == 0x5800 {
        return format!("TT({})", kc & 0xFF);
    }
    // LT(layer, kc)
    if (kc & 0xF000) == 0x4000 {
        let layer = (kc >> 8) & 0x0F;
        let base = (kc & 0xFF) as u8;
        return format!("LT{}({})", layer, basic_kc_name(base));
    }
    // Mod-Tap
    if kc & 0x8000 != 0 {
        let mods = ((kc >> 8) & 0x1F) as u8;
        let base = (kc & 0xFF) as u8;
        return format!("{}_{}", mod_prefix(mods), basic_kc_name(base));
    }

    format!("0x{:04X}", kc)
}

fn mod_prefix(mods: u8) -> String {
    let mut s = String::new();
    if mods & 0x01 != 0 { s.push('C'); }
    if mods & 0x02 != 0 { s.push('S'); }
    if mods & 0x04 != 0 { s.push('A'); }
    if mods & 0x08 != 0 { s.push('G'); }
    s
}

fn basic_kc_name(kc: u8) -> &'static str {
    match kc {
        0x04 => "A", 0x05 => "B", 0x06 => "C", 0x07 => "D", 0x08 => "E",
        0x09 => "F", 0x0A => "G", 0x0B => "H", 0x0C => "I", 0x0D => "J",
        0x0E => "K", 0x0F => "L", 0x10 => "M", 0x11 => "N", 0x12 => "O",
        0x13 => "P", 0x14 => "Q", 0x15 => "R", 0x16 => "S", 0x17 => "T",
        0x18 => "U", 0x19 => "V", 0x1A => "W", 0x1B => "X", 0x1C => "Y",
        0x1D => "Z",
        0x1E => "1", 0x1F => "2", 0x20 => "3", 0x21 => "4", 0x22 => "5",
        0x23 => "6", 0x24 => "7", 0x25 => "8", 0x26 => "9", 0x27 => "0",
        0x28 => "\u{23CE}", // ⏎
        0x29 => "Esc", 0x2A => "\u{232B}", // ⌫
        0x2B => "Tab",
        0x2C => "\u{2423}", // ␣
        0x2D => "-", 0x2E => "=", 0x2F => "[", 0x30 => "]", 0x31 => "\\",
        0x33 => ";", 0x34 => "'", 0x35 => "`", 0x36 => ",", 0x37 => ".",
        0x38 => "/", 0x39 => "Caps",
        0x3A => "F1", 0x3B => "F2", 0x3C => "F3", 0x3D => "F4", 0x3E => "F5",
        0x3F => "F6", 0x40 => "F7", 0x41 => "F8", 0x42 => "F9", 0x43 => "F10",
        0x44 => "F11", 0x45 => "F12",
        0x46 => "PScr", 0x47 => "SLck", 0x48 => "Paus", 0x49 => "Ins",
        0x4A => "Home", 0x4B => "PgUp", 0x4C => "Del", 0x4D => "End", 0x4E => "PgDn",
        0x4F => "\u{2192}", // →
        0x50 => "\u{2190}", // ←
        0x51 => "\u{2193}", // ↓
        0x52 => "\u{2191}", // ↑
        0x65 => "Menu",
        0x7A => "Undo", 0x7B => "Cut", 0x7C => "Copy", 0x7D => "Paste",
        0xA8 => "Mute", 0xA9 => "Vol+", 0xAA => "Vol-",
        0xE0 => "Ctrl", 0xE1 => "Shift", 0xE2 => "Alt", 0xE3 => "Super",
        0xE4 => "RCtrl", 0xE5 => "RShift", 0xE6 => "RAlt", 0xE7 => "RSuper",
        _ => "?",
    }
}

fn shift_symbol(kc: u8) -> Option<&'static str> {
    match kc {
        0x1E => Some("!"), 0x1F => Some("@"), 0x20 => Some("#"),
        0x21 => Some("$"), 0x22 => Some("%"), 0x23 => Some("^"),
        0x24 => Some("&"), 0x25 => Some("*"), 0x26 => Some("("),
        0x27 => Some(")"), 0x2D => Some("_"), 0x2E => Some("+"),
        0x2F => Some("{"), 0x30 => Some("}"), 0x31 => Some("|"),
        0x33 => Some(":"), 0x34 => Some("\""), 0x35 => Some("~"),
        0x36 => Some("<"), 0x37 => Some(">"), 0x38 => Some("?"),
        _ => None,
    }
}
