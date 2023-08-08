use std::path::PathBuf;
use tauri::api::path::cache_dir;
use opener::reveal;

pub fn resolve_cache_dir() -> PathBuf {
  return cache_dir().unwrap().join(crate::configuration_manager::resolve_bundle_identifier());
}

pub fn clear_cache() {
  let path = resolve_cache_dir();
  match std::fs::remove_dir_all(&path) {
    Ok(_) => {
      println!("Remove cache dir successful!");
      match std::fs::create_dir(&path) {
        Ok(_) => {
          println!("Creation of cache dir successful!");
          crate::notifications::send_notification("Cache has been cleared!", "Room for new waifu wallpapers!", crate::configuration_manager::resolve_bundle_identifier().as_str());
        },
        Err(_) => eprintln!("Creation of cache dir failed!")
      };
    },
    Err(_) => eprintln!("Remove cache dir failed!")
  };
}

pub fn open_cache_dir() {
  match reveal(resolve_cache_dir()) {
    Ok(_) => (),
    Err(x) => eprintln!("{}", x)
  };
}
