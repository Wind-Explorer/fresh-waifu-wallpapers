use std::{io::Write, path::PathBuf};

use serde::{Serialize, Deserialize};
use tauri::api::path::{config_dir, cache_dir};

#[derive(Serialize, Deserialize)]
struct ConfigData {
  user_preference: String,
}

fn resolve_bundle_identifier_path() -> PathBuf {
  return cache_dir().unwrap().join("wwf.id");
}

pub fn retrieve_identifier(app: tauri::AppHandle) {
  match std::fs::File::create(resolve_bundle_identifier_path()) {
    Ok(mut x) => {
      println!("Bundle identifier: {}", app.config().tauri.bundle.identifier);
      match &x.write_all(app.config().tauri.bundle.identifier.as_bytes()) {
        Ok(_) => (),
        Err(error) => {
          eprintln!("Failed to write app ID to file: {}", error);
        }
      }
    },
    Err(x) => eprintln!("Error: {}", x)
  };
}

pub fn resolve_bundle_identifier() -> String {
  return std::fs::read_to_string(resolve_bundle_identifier_path()).unwrap();
}

pub fn resolve_configuration_path() -> PathBuf {
  return config_dir().unwrap().join(resolve_bundle_identifier()).join("wwc.toml");
}

pub fn initialize_configuration() {
  let config_file = resolve_configuration_path();
  let config_dir = config_file.parent().unwrap();
  if !config_dir.exists() {
    match std::fs::create_dir_all(&config_dir) {
      Ok(_) => {
        match std::fs::File::create(&config_file) {
          Ok(_) => {
            if std::fs::read_to_string(resolve_configuration_path()).unwrap().is_empty() {
              register_configuration();
            }
          },
          Err(x) => eprintln!("Error: {}", x)
        };
      },
      Err(x) => eprintln!("Error: {}", x)
    }
  }
}

pub fn register_configuration() {
  let default_config = ConfigData {
    user_preference: "neko".to_string()
  };
  let config_data = toml::to_string(&default_config).unwrap();
  match std::fs::write(resolve_configuration_path(), config_data) {
    Ok(_) => (),
    Err(x) => eprintln!("Error: {}", x)
  };
}
