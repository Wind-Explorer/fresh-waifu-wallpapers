use tauri::Manager;

pub fn show_main_window(app: &tauri::AppHandle) {
  let window = app.get_window("main").unwrap();
  window.show().unwrap();
}