use serde::Serialize;
use std::process::Command;

#[derive(Debug, Clone, Serialize)]
pub struct InputLang {
    pub index: u32,
    pub id: String,   // "us", "ru", etc.
    pub label: String, // "EN", "RU", etc.
}

const LANG_LABELS: &[(&str, &str)] = &[
    ("us", "EN"),
    ("ru", "RU"),
    ("ua", "UA"),
    ("de", "DE"),
    ("fr", "FR"),
    ("es", "ES"),
];

fn label_for(id: &str) -> String {
    LANG_LABELS
        .iter()
        .find(|(k, _)| *k == id)
        .map(|(_, v)| v.to_string())
        .unwrap_or_else(|| id.to_uppercase())
}

/// Read current input source index from GNOME gsettings.
pub fn get_current_input_lang() -> Result<InputLang, String> {
    // Get current index
    let idx_out = Command::new("gsettings")
        .args(["get", "org.gnome.desktop.input-sources", "current"])
        .output()
        .map_err(|e| format!("gsettings error: {e}"))?;
    let idx_str = String::from_utf8_lossy(&idx_out.stdout);
    // Format: "uint32 0"
    let index: u32 = idx_str
        .trim()
        .rsplit_once(' ')
        .and_then(|(_, n)| n.parse().ok())
        .unwrap_or(0);

    // Get sources list
    let src_out = Command::new("gsettings")
        .args(["get", "org.gnome.desktop.input-sources", "sources"])
        .output()
        .map_err(|e| format!("gsettings error: {e}"))?;
    let src_str = String::from_utf8_lossy(&src_out.stdout);
    // Format: [('xkb', 'us'), ('xkb', 'ru')]
    let ids: Vec<String> = src_str
        .split('\'')
        .enumerate()
        .filter_map(|(i, s)| {
            // Every 4th quoted string starting from index 3 is the layout id
            if i % 4 == 3 { Some(s.to_string()) } else { None }
        })
        .collect();

    let id = ids.get(index as usize).cloned().unwrap_or_default();
    let label = label_for(&id);

    Ok(InputLang { index, id, label })
}
