pub fn send_notification(title: &str, body: &str, bundle_identifier: &str) {
  match tauri::api::notification::Notification::new(bundle_identifier)
  .title(title)
  .body(body)
  .show() {
    Ok(_) => (),
    Err(x) => eprintln!("Error sending notification: {}", x)
  };
}
