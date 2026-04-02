# Sofle Keyboard Overlay

Interactive overlay for Sofle split keyboard that reads the keymap directly from Vial firmware and displays active layer in real-time.

**No firmware modification required** — the app communicates with the keyboard via standard Vial/VIA HID protocol.

## Features

- Reads keymap from keyboard via Vial HID protocol (no firmware changes needed)
- Auto layer detection by polling switch matrix state (detects MO/TG key presses)
- Real-time key press highlighting
- EN/RU input language indicator (GNOME/gsettings)
- Columnar stagger layout matching Sofle v2 physical layout
- Manual layer switching (arrow keys, 1-4) with auto/manual mode toggle
- Transparent always-on-top window, draggable by header
- Built with Tauri 2 (Rust + HTML Canvas)

## Screenshot

```
Layer 0  [0] 1 2 3   EN   AUTO

   `    1    2    3    4    5        6    7    8    9    0    `
  Esc   Q    W    E    R    T        Y    U    I    O    P   ⌫
  Tab   A    S    D    F    G   ()   H    J    K    L    ;    '
 Shift  Z    X    C    V    B   ()   N    M    ,    .    /  RShift
       Super Alt  Ctrl MO(2) ⏎      ␣  MO(3) RCtrl RAlt RSuper
```

## Requirements

- Linux (X11 or Wayland)
- Sofle keyboard with Vial firmware (USB)
- System dependencies: `libwebkit2gtk-4.1-dev`, `libgtk-3-dev`, `libhidapi-dev`, `libudev-dev`
- Rust toolchain
- Node.js

## Installation

### System dependencies (Ubuntu/Debian)

```bash
sudo apt install -y libwebkit2gtk-4.1-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev libhidapi-dev libudev-dev pkg-config
```

### udev rules (access HID without sudo)

```bash
sudo tee /etc/udev/rules.d/99-vial.rules > /dev/null << 'EOF'
KERNEL=="hidraw*", SUBSYSTEM=="hidraw", ATTRS{serial}=="vial:*", MODE="0660", GROUP="plugdev", TAG+="uaccess"
EOF
sudo udevadm control --reload-rules && sudo udevadm trigger
```

Make sure your user is in the `plugdev` group:

```bash
sudo usermod -aG plugdev $USER
```

### Build

```bash
npm install
npx tauri build
```

The binary will be in `src-tauri/target/release/sofle-overlay`.

### Development

```bash
npm install
npx tauri dev
```

## Usage

Run the app with the Sofle keyboard connected via USB:

```bash
./sofle-overlay
```

### Controls (when overlay window is focused)

| Key | Action |
|-----|--------|
| `A` | Toggle auto/manual layer detection |
| `←` / `→` | Switch layer (disables auto mode) |
| `1`-`4` | Jump to specific layer (disables auto mode) |
| `Esc` / `Q` | Close overlay |

### Auto layer detection

The app polls the keyboard's switch matrix state every 60ms. When it detects that a MO() or TG() key is physically held down, it automatically switches the displayed layer.

## How it works

1. **Connect** — finds the Sofle keyboard on hidraw by VID/PID (`FC32:0287`), probes for Vial raw HID interface
2. **Read keymap** — reads all layers via VIA protocol command `dynamic_keymap_get_keycode` (0x04)
3. **Detect layer keys** — scans layer 0 for MO/TG/LT/OSL keycodes and records their matrix positions
4. **Poll matrix** — sends `get_keyboard_value` / `switch_matrix_state` (0x02/0x03) to read which keys are physically pressed
5. **Render** — draws the keyboard layout on HTML Canvas with columnar stagger, highlights pressed keys

## Adapting to other keyboards

The layout is currently hardcoded for Sofle v2 (10×6 matrix, 5 rows per half). To adapt for another Vial keyboard:

1. Update `SOFLE_VID` / `SOFLE_PID` in `src-tauri/src/vial.rs`
2. Update `MATRIX_ROWS` / `MATRIX_COLS` in `src-tauri/src/vial.rs`
3. Update the physical layout coordinates in `src/index.html` (`buildKeys()` function)

## License

MIT

---

# Sofle Keyboard Overlay (RU)

Интерактивный оверлей для сплит-клавиатуры Sofle, который читает раскладку напрямую из прошивки Vial и отображает активный слой в реальном времени.

**Модификация прошивки не требуется** — приложение общается с клавиатурой через стандартный Vial/VIA HID-протокол.

## Возможности

- Чтение раскладки с клавиатуры через Vial HID-протокол (без изменения прошивки)
- Автоопределение активного слоя через polling состояния матрицы (детектит нажатие MO/TG клавиш)
- Подсветка нажатых клавиш в реальном времени
- Индикатор языка ввода EN/RU (GNOME/gsettings)
- Колоночный стаггер, соответствующий физической раскладке Sofle v2
- Ручное переключение слоёв (стрелки, 1-4) с режимом авто/ручной
- Прозрачное окно поверх всех, перетаскивается за шапку
- Построен на Tauri 2 (Rust + HTML Canvas)

## Требования

- Linux (X11 или Wayland)
- Клавиатура Sofle с прошивкой Vial (USB)
- Системные зависимости: `libwebkit2gtk-4.1-dev`, `libgtk-3-dev`, `libhidapi-dev`, `libudev-dev`
- Rust toolchain
- Node.js

## Установка

### Системные зависимости (Ubuntu/Debian)

```bash
sudo apt install -y libwebkit2gtk-4.1-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev libhidapi-dev libudev-dev pkg-config
```

### Правила udev (доступ к HID без sudo)

```bash
sudo tee /etc/udev/rules.d/99-vial.rules > /dev/null << 'EOF'
KERNEL=="hidraw*", SUBSYSTEM=="hidraw", ATTRS{serial}=="vial:*", MODE="0660", GROUP="plugdev", TAG+="uaccess"
EOF
sudo udevadm control --reload-rules && sudo udevadm trigger
```

Убедитесь, что ваш пользователь в группе `plugdev`:

```bash
sudo usermod -aG plugdev $USER
```

### Сборка

```bash
npm install
npx tauri build
```

Бинарник будет в `src-tauri/target/release/sofle-overlay`.

### Разработка

```bash
npm install
npx tauri dev
```

## Использование

Запустите приложение с подключённой по USB клавиатурой Sofle:

```bash
./sofle-overlay
```

### Управление (когда окно оверлея в фокусе)

| Клавиша | Действие |
|---------|----------|
| `A` | Переключить авто/ручной режим определения слоя |
| `←` / `→` | Переключить слой (отключает авто) |
| `1`-`4` | Перейти на конкретный слой (отключает авто) |
| `Esc` / `Q` | Закрыть оверлей |

### Автоопределение слоя

Приложение опрашивает состояние матрицы клавиатуры каждые 60мс. Когда обнаруживает, что клавиша MO() или TG() физически зажата, автоматически переключает отображаемый слой.

## Как это работает

1. **Подключение** — находит Sofle на hidraw по VID/PID (`FC32:0287`), определяет Vial raw HID интерфейс
2. **Чтение раскладки** — читает все слои через VIA-протокол, команда `dynamic_keymap_get_keycode` (0x04)
3. **Поиск клавиш слоёв** — сканирует слой 0 на наличие MO/TG/LT/OSL кейкодов и запоминает их позиции в матрице
4. **Polling матрицы** — отправляет `get_keyboard_value` / `switch_matrix_state` (0x02/0x03) для чтения физически нажатых клавиш
5. **Рендер** — рисует раскладку на HTML Canvas с колоночным стаггером, подсвечивает нажатые клавиши

## Адаптация под другие клавиатуры

Раскладка сейчас захардкожена под Sofle v2 (матрица 10×6, 5 строк на половину). Для другой Vial-клавиатуры:

1. Обновите `SOFLE_VID` / `SOFLE_PID` в `src-tauri/src/vial.rs`
2. Обновите `MATRIX_ROWS` / `MATRIX_COLS` в `src-tauri/src/vial.rs`
3. Обновите координаты физической раскладки в `src/index.html` (функция `buildKeys()`)

## Лицензия

MIT
