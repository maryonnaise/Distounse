use std::{ fs, path::PathBuf, sync::{ Arc, Mutex }, thread, time::{ Duration, Instant } };

use device_query::{ DeviceQuery, DeviceState };
use dirs::home_dir;
use tauri::{
    path::BaseDirectory,
    Manager,
    image::Image,
    menu::{ Menu, MenuItem },
    tray::TrayIconBuilder,
    tray::TrayIcon,
    Builder,
    State,
};
use tauri_plugin_dialog::{ DialogExt, MessageDialogButtons, MessageDialogKind };
use std::path::Path;
#[tauri::command]
fn get_mouse_distance(state: State<'_, SharedTracker>) -> f64 {
    let tracker = state.lock().unwrap();
    tracker.get_kilometers()
}

#[tauri::command]
fn reset_mouse_distance(state: State<'_, SharedTracker>) {
    let mut tracker = state.lock().unwrap();
    tracker.total_distance_m = 0.0;
    let _ = fs::write(&tracker.save_path, "0");
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![get_mouse_distance, reset_mouse_distance])
        .setup(move |app| {
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            let save_path = app
                .path()
                .resolve("mouse/.mouse_distance.txt", BaseDirectory::Resource)?;

            let tracker = Arc::new(Mutex::new(Tracker::new(save_path)));
            let tracker_clone = Arc::clone(&tracker);

            let tray_icon_holder: Arc<Mutex<Option<TrayIcon>>> = Arc::new(Mutex::new(None));
            let tray_icon_clone = tray_icon_holder.clone();

            // 🧵 Thread che aggiorna distanza + tooltip della tray icon
            thread::spawn(move || {
                let mut device_state = DeviceState::new();

                loop {
                    {
                        let mut tracker = tracker_clone.lock().unwrap();
                        tracker.update(&mut device_state);
                    }

                    {
                        let tracker = tracker_clone.lock().unwrap();
                        let km = tracker.get_kilometers();
                        let title = format!("{:.3} km", km);

                        if let Ok(mut tray_lock) = tray_icon_clone.lock() {
                            if let Some(ref mut tray) = *tray_lock {
                                let _ = tray.set_title(Some(&title));
                            }
                        }
                    }

                    thread::sleep(Duration::from_millis(200));
                }
            });

            app.manage(tracker.clone());
            let reset_i = MenuItem::with_id(app, "reset", "Reset", true, None::<&str>)?;
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&reset_i, &quit_i])?;
            let tray = TrayIconBuilder::new()
                .menu(&menu)
                .show_menu_on_left_click(true)
                .title("Km: 0.000") // titolo iniziale
                .on_menu_event(move |app, event| {
                    match event.id.as_ref() {
                        "reset" => {
                            let mut tracker = tracker.lock().unwrap();
                            // Mostra dialog di conferma sincrono
                            let answer = app
                                .dialog()
                                .message("Sei sicuro di voler azzerare i chilometri?")
                                .kind(MessageDialogKind::Warning)
                                .buttons(MessageDialogButtons::OkCancel)
                                .blocking_show();

                            if answer {
                                tracker.total_distance_m = 0.0;
                                let _ = fs::write(&tracker.save_path, "0");
                            } else {
                                println!("❌ Reset annullato");
                            }
                        }
                        "quit" => {
                            let tracker = tracker.lock().unwrap();
                            let _ = fs::write(
                                &tracker.save_path,
                                format!("{}", tracker.total_distance_m)
                            );
                            println!(
                                "💾 Distanza salvata alla chiusura: {:.4} m",
                                tracker.total_distance_m
                            );
                            app.exit(0);
                        }
                        _ => {
                            println!("menu item {:?} not handled", event.id);
                        }
                    }
                })
                .build(app)?;

            // Salva il tray icon per aggiornare il titolo nel thread
            *tray_icon_holder.lock().unwrap() = Some(tray);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

type SharedTracker = Arc<Mutex<Tracker>>;

#[derive(Debug)]
pub struct Tracker {
    last_pos: (i32, i32),
    total_distance_m: f64,
    save_path: PathBuf,
    last_save_time: Instant,
}

impl Tracker {
    pub fn new(save_path: PathBuf) -> Self {
        let pos = DeviceState::new().get_mouse().coords;

        let total_distance_m = fs
            ::read_to_string(&save_path)
            .ok()
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.0);

        Tracker {
            last_pos: pos,
            total_distance_m,
            save_path,
            last_save_time: Instant::now(),
        }
    }

    pub fn update(&mut self, device_state: &mut DeviceState) {
        let current_pos = device_state.get_mouse().coords;
        let dx = (current_pos.0 - self.last_pos.0) as f64;
        let dy = (current_pos.1 - self.last_pos.1) as f64;
        let dist = (dx.powi(2) + dy.powi(2)).sqrt();

        self.total_distance_m += dist * 0.000264583; // pixel -> metri
        self.last_pos = current_pos;

        if self.last_save_time.elapsed() >= Duration::from_secs(60) {
            let _ = fs::write(&self.save_path, format!("{}", self.total_distance_m));
            self.last_save_time = Instant::now();
        }
    }

    pub fn get_kilometers(&self) -> f64 {
        self.total_distance_m / 1000.0
    }
}
