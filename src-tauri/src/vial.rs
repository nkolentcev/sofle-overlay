use hidapi::HidApi;
use serde::{Deserialize, Serialize};

const SOFLE_VID: u16 = 0xFC32;
const SOFLE_PID: u16 = 0x0287;
const MATRIX_ROWS: usize = 10;
const MATRIX_COLS: usize = 6;
const MSG_LEN: usize = 32;

// VIA protocol commands
const CMD_GET_KEYBOARD_VALUE: u8 = 0x02;
const SUBCMD_SWITCH_MATRIX_STATE: u8 = 0x03;
const CMD_GET_KEYCODE: u8 = 0x04;
const CMD_GET_LAYER_COUNT: u8 = 0x11;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeymapData {
    pub layers: Vec<Vec<Vec<u16>>>, // [layer][row][col]
    pub num_layers: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatrixState {
    pub pressed: Vec<(usize, usize)>, // (row, col)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayerState {
    pub active_layer: usize,
    pub pressed: Vec<(usize, usize)>,
}

/// Find and return the path to the Sofle Vial raw HID interface.
fn find_sofle_raw_hid(api: &HidApi) -> Option<hidapi::DeviceInfo> {
    // Enumerate all Sofle interfaces, find the one that responds to VIA commands
    let devices: Vec<_> = api
        .device_list()
        .filter(|d| d.vendor_id() == SOFLE_VID && d.product_id() == SOFLE_PID)
        .cloned()
        .collect();

    for dev_info in devices {
        if let Ok(device) = dev_info.open_device(api) {
            let mut msg = [0u8; MSG_LEN];
            msg[0] = CMD_GET_LAYER_COUNT;
            if device.write(&msg).is_ok() {
                let mut resp = [0u8; MSG_LEN];
                if let Ok(n) = device.read_timeout(&mut resp, 200) {
                    if n > 0 && resp[0] == CMD_GET_LAYER_COUNT && resp[1] > 0 {
                        return Some(dev_info);
                    }
                }
            }
        }
    }
    None
}

pub struct VialDevice {
    device: hidapi::HidDevice,
}

impl VialDevice {
    pub fn open() -> Result<Self, String> {
        let api = HidApi::new().map_err(|e| format!("HidApi init failed: {e}"))?;
        let info = find_sofle_raw_hid(&api).ok_or("Sofle keyboard not found")?;
        let device = info
            .open_device(&api)
            .map_err(|e| format!("Failed to open device: {e}"))?;
        device
            .set_blocking_mode(true)
            .map_err(|e| format!("Failed to set blocking mode: {e}"))?;
        Ok(Self { device })
    }

    fn cmd(&self, data: &[u8]) -> Result<[u8; MSG_LEN], String> {
        let mut msg = [0u8; MSG_LEN];
        let len = data.len().min(MSG_LEN);
        msg[..len].copy_from_slice(&data[..len]);
        self.device
            .write(&msg)
            .map_err(|e| format!("HID write error: {e}"))?;
        let mut resp = [0u8; MSG_LEN];
        self.device
            .read_timeout(&mut resp, 500)
            .map_err(|e| format!("HID read error: {e}"))?;
        Ok(resp)
    }

    pub fn get_layer_count(&self) -> Result<usize, String> {
        let resp = self.cmd(&[CMD_GET_LAYER_COUNT])?;
        Ok(resp[1] as usize)
    }

    pub fn get_keycode(&self, layer: u8, row: u8, col: u8) -> Result<u16, String> {
        let resp = self.cmd(&[CMD_GET_KEYCODE, layer, row, col])?;
        Ok(((resp[4] as u16) << 8) | resp[5] as u16)
    }

    pub fn read_keymap(&self) -> Result<KeymapData, String> {
        let num_layers = self.get_layer_count()?;
        let mut layers = Vec::with_capacity(num_layers);

        for layer in 0..num_layers {
            let mut rows = Vec::with_capacity(MATRIX_ROWS);
            for row in 0..MATRIX_ROWS {
                let mut cols = Vec::with_capacity(MATRIX_COLS);
                for col in 0..MATRIX_COLS {
                    let kc = self.get_keycode(layer as u8, row as u8, col as u8)?;
                    cols.push(kc);
                }
                rows.push(cols);
            }
            layers.push(rows);
        }

        Ok(KeymapData { layers, num_layers })
    }

    pub fn get_matrix_state(&self) -> Result<MatrixState, String> {
        let resp = self.cmd(&[CMD_GET_KEYBOARD_VALUE, SUBCMD_SWITCH_MATRIX_STATE])?;
        let bitmap = &resp[2..]; // skip command echo
        let mut pressed = Vec::new();

        // QMK packs matrix as ceil(COLS/8) bytes per row
        let bytes_per_row = (MATRIX_COLS + 7) / 8; // = 1 for 6 cols
        for row in 0..MATRIX_ROWS {
            let byte_offset = row * bytes_per_row;
            if byte_offset >= bitmap.len() {
                break;
            }
            let b = bitmap[byte_offset];
            if b == 0 {
                continue;
            }
            for col in 0..MATRIX_COLS {
                if b & (1 << col) != 0 {
                    pressed.push((row, col));
                }
            }
        }

        Ok(MatrixState { pressed })
    }
}

/// Scan the keymap for MO/TG/LT layer keys, returns map of target_layer -> [(row, col)]
pub fn find_layer_keys(keymap: &KeymapData) -> std::collections::HashMap<usize, Vec<(usize, usize)>> {
    let mut map = std::collections::HashMap::new();
    if keymap.layers.is_empty() {
        return map;
    }
    let base = &keymap.layers[0];
    for (row_idx, row) in base.iter().enumerate() {
        for (col_idx, &kc) in row.iter().enumerate() {
            let target = if (kc & 0xFF00) == 0x5100 {
                Some((kc & 0xFF) as usize) // MO(layer)
            } else if (kc & 0xFF00) == 0x5300 {
                Some((kc & 0xFF) as usize) // TG(layer)
            } else if (kc & 0xF000) == 0x4000 {
                Some(((kc >> 8) & 0x0F) as usize) // LT(layer, kc)
            } else if (kc & 0xFF00) == 0x5400 {
                Some((kc & 0xFF) as usize) // OSL(layer)
            } else {
                None
            };
            if let Some(t) = target {
                map.entry(t).or_insert_with(Vec::new).push((row_idx, col_idx));
            }
        }
    }
    map
}

/// Determine active layer from pressed keys and layer key map.
pub fn detect_active_layer(
    pressed: &[(usize, usize)],
    layer_keys: &std::collections::HashMap<usize, Vec<(usize, usize)>>,
) -> usize {
    // Higher layers take priority
    let mut max_layer = 0;
    for (&target_layer, positions) in layer_keys {
        for pos in positions {
            if pressed.contains(pos) && target_layer > max_layer {
                max_layer = target_layer;
            }
        }
    }
    max_layer
}
