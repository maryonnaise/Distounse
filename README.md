# Distounse 🖱️
![Distounse](https://github.com/user-attachments/assets/0b05a7ec-4db5-4031-9034-d29ab43f30cf)

**Distounse** is a minimalist macOS tray application that tracks the total distance your mouse cursor has moved and displays it directly in the system tray.

## 🧭 Features

- 📏 **Real-time distance tracking** of your mouse cursor, measured in kilometers.
- 🖥️ **System tray integration**: no GUI windows, only a tray icon with live distance updates.
- 🧼 **Reset option** with confirmation dialog to clear the total tracked distance.
- 💾 **Auto-save** the distance every minute and on quit.
- 🔒 Saves data to a hidden `.mouse_distance.txt` file in the app's resource directory.

## 🖼️ Screenshot

<img width="404" alt="Schermata 2025-06-21 alle 19 56 12" src="https://github.com/user-attachments/assets/c39c74e1-7dd1-4c5d-9878-96e16eec9fd5" />


>Distounse appereance in the system tray

## 🚀 Installation

1. Download the latest `.dmg` file from the [Releases](https://github.com/maryonnaise/Distounse/releases) section.
2. Open the `.dmg` file and drag **Distounse** to your Applications folder.
3. Launch the app. It will appear in the **menu bar**.

> ⚠️ The app will not show a visible window — it works entirely from the tray.

## 🛡️ macOS Accessibility Permissions

To function properly, **Distounse** requires permission to monitor mouse movements.  
On **macOS**, this means you must grant the app **Accessibility permissions**.

### How to enable:
1. Open `System Settings` → `Privacy & Security` → `Accessibility`
2. Click the lock icon 🔒 and enter your password
3. Add or enable **Distounse** in the list of allowed apps

> ⚠️ The app will not work correctly unless this permission is granted.


## 📦 Build Instructions

To build the app locally:

```bash
# Install Rust & Tauri prerequisites first
git clone git@github.com:maryonnaise/Distounse.git
cd Distounse
npm install
npm run tauri build
```

This will produce a `.app` and `.dmg` file in the `src-tauri/target/release/bundle/macos/` folder.

## 🧠 How It Works

Distounse uses the [`device_query`](https://docs.rs/device_query/) crate to capture mouse coordinates at ~200ms intervals. The distance is calculated using Euclidean geometry, converted to meters with a scaling factor (`0.000264583` per pixel), then shown as **km** in the tray.

### Data Persistence

- The app saves the current distance in a plain text file named `.mouse_distance.txt`, stored in the Tauri **resource directory**.
- The file is hidden to avoid cluttering user-visible folders.
- Data is reloaded on launch and saved every 60 seconds and on quit.

## ⚙️ Tray Menu

Right-click (or left-click on macOS) on the tray icon to open the menu:

- 🔁 **Reset** – Clear tracked distance after confirmation.
- ❌ **Quit** – Save and exit the app.

## 🔒 Permissions

The app reads only the mouse coordinates and **does not collect or transmit any data**. All data is stored **locally**.

## 📜 License

MIT License © [maryonnaise](https://github.com/maryonnaise)
