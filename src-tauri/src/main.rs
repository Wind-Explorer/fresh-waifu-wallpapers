// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod refresh_wallpaper;

use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
fn main() {
    // Define menu item entries
    let refresh = CustomMenuItem::new("refresh".to_string(), "Refresh wallpaper");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");

    // Populate tray menu with entries
    let tray_menu = SystemTrayMenu::new()
        .add_item(refresh)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .on_window_event(|e| {
            if let tauri::WindowEvent::Resized(_) = e.event() {
                std::thread::sleep(std::time::Duration::from_nanos(1000));
            }
        })
        .invoke_handler(tauri::generate_handler![greet])
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
                    match tauri::async_runtime::block_on(refresh_wallpaper::refresh_wallpaper("2".to_string())) {
                        Ok(()) => (),
                        Err(_) => eprintln!("Error refreshing"),
                    };
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
