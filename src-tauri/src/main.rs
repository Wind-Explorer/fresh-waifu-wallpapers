// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod refresh_wallpaper;
mod notifications;
mod cache_management;
mod window_management;

use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem, SystemTraySubmenu};

fn main() {
    // Define menu item entries
    let refresh = CustomMenuItem::new("refresh".to_string(), "Refresh wallpaper");
    let show_configuration = CustomMenuItem::new("show_configuration".to_string(), "Preferences");
    let clear_cache = CustomMenuItem::new("clear_cache".to_string(), "Clear");
    let open_cache = CustomMenuItem::new("open_cache".to_string(), "Reveal");
    let cache_menu = SystemTraySubmenu::new(
        "Manage cache".to_string(),
        SystemTrayMenu::new()
        .add_item(clear_cache)
        .add_item(open_cache)
    );
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    
    // Populate tray menu with entries
    let tray_menu = SystemTrayMenu::new()
        .add_item(refresh)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(show_configuration)
        .add_submenu(cache_menu)
        .add_item(quit);
    
    let system_tray = SystemTray::new().with_menu(tray_menu);
    tauri::Builder::default()
        .setup(|_| {
            notifications::send_notification("Hello, waifu wallpaper enjoyer!", "Look for me in the system tray.");
            Ok(())
        })
        .on_window_event(|e| {
            if let tauri::WindowEvent::Resized(_) = e.event() {
                std::thread::sleep(std::time::Duration::from_nanos(1000));
            }
        })
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
              event.window().hide().unwrap();
              api.prevent_close();
            }
            _ => {}
          })
        .invoke_handler(tauri::generate_handler![])
        .system_tray(system_tray)
        .on_system_tray_event(|_app, event| match event {
            SystemTrayEvent::LeftClick {
                position: _,
                size: _,
                ..
            } => {
                println!("system tray received a left click");
            }
            SystemTrayEvent::RightClick {
                position: _,
                size: _,
                ..
            } => {
                println!("system tray received a right click");
            }
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "refresh" => {
                    match tauri::async_runtime::block_on(refresh_wallpaper::refresh_wallpaper()) {
                        Ok(()) => (),
                        Err(_) => eprintln!("Error refreshing"),
                    };
                }
                "show_configuration" => {
                    window_management::show_main_window(_app)
                }
                "open_cache" => {
                    cache_management::open_cache_dir();
                }
                "clear_cache" => {
                    cache_management::clear_cache();
                }
                "quit" => {
                    std::process::exit(0);
                }
                _ => {}
            },
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}
