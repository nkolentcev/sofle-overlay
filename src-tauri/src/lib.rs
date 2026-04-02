mod input_lang;
mod keycodes;
mod vial;

use serde::Serialize;
use std::collections::HashMap;
use std::sync::Mutex;
use tauri::State;

struct AppState {
    device: Option<vial::VialDevice>,
    keymap: Option<vial::KeymapData>,
    layer_keys: HashMap<usize, Vec<(usize, usize)>>,
}

type SharedState = Mutex<AppState>;

#[derive(Serialize)]
struct KeyInfo {
    code: u16,
    label: String,
}

#[derive(Serialize)]
struct FullKeymap {
    num_layers: usize,
    layers: Vec<Vec<Vec<KeyInfo>>>, // [layer][row][col]
    layer_keys: HashMap<usize, Vec<(usize, usize)>>,
}

#[tauri::command]
fn connect_keyboard(state: State<SharedState>) -> Result<String, String> {
    let mut s = state.lock().map_err(|e| e.to_string())?;
    let device = vial::VialDevice::open()?;
    let keymap = device.read_keymap()?;
    let layer_keys = vial::find_layer_keys(&keymap);
    let msg = format!("Connected: {} layers", keymap.num_layers);
    s.device = Some(device);
    s.keymap = Some(keymap);
    s.layer_keys = layer_keys;
    Ok(msg)
}

#[tauri::command]
fn get_keymap(state: State<SharedState>) -> Result<FullKeymap, String> {
    let s = state.lock().map_err(|e| e.to_string())?;
    let keymap = s.keymap.as_ref().ok_or("Not connected")?;

    let layers = keymap
        .layers
        .iter()
        .map(|layer| {
            layer
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|&code| KeyInfo {
                            code,
                            label: keycodes::decode_keycode(code),
                        })
                        .collect()
                })
                .collect()
        })
        .collect();

    Ok(FullKeymap {
        num_layers: keymap.num_layers,
        layers,
        layer_keys: s.layer_keys.clone(),
    })
}

#[tauri::command]
fn poll_layer(state: State<SharedState>) -> Result<vial::LayerState, String> {
    let s = state.lock().map_err(|e| e.to_string())?;
    let device = s.device.as_ref().ok_or("Not connected")?;
    let matrix = device.get_matrix_state()?;
    let active = vial::detect_active_layer(&matrix.pressed, &s.layer_keys);
    Ok(vial::LayerState {
        active_layer: active,
        pressed: matrix.pressed,
    })
}

#[tauri::command]
fn get_input_lang() -> Result<input_lang::InputLang, String> {
    input_lang::get_current_input_lang()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(Mutex::new(AppState {
            device: None,
            keymap: None,
            layer_keys: HashMap::new(),
        }))
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            connect_keyboard,
            get_keymap,
            poll_layer,
            get_input_lang,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
