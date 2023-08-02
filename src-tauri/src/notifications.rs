pub fn send_notification(title: &str, body: &str) {
  match tauri::api::notification::Notification::new("cn.Wind-Explorer.freshwaifuwallpaper")
  .title(title)
  .body(body)
  .show() {
    Ok(_) => (),
    Err(x) => eprintln!("Error sending notification: {}", x)
  };
}
