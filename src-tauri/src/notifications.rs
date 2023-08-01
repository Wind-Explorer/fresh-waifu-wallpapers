pub fn send_notification(title: &str, body: &str) {
  match tauri::api::notification::Notification::new("")
  .title(title)
  .body(body)
  .show() {
    Ok(_) => (),
    Err(x) => eprintln!("Error sending notification: {}", x)
  };
}
