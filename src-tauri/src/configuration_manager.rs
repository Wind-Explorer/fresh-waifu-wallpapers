use std::{io::Write, path::PathBuf};

use serde::{Serialize, Deserialize};
use tauri::api::path::{config_dir, cache_dir};

use nekosbest::Category;

#[derive(Serialize, Deserialize)]
pub struct ConfigData {
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
              register_configuration(None);
            }
          },
          Err(x) => eprintln!("Error: {}", x)
        };
      },
      Err(x) => eprintln!("Error: {}", x)
    }
  }
}

pub fn register_configuration(preference: Option<&str>) {
  let default_config = ConfigData {
    user_preference: preference.unwrap_or("neko").to_string()
  };
  let config_data = toml::to_string(&default_config).unwrap();
  match std::fs::write(resolve_configuration_path(), config_data) {
    Ok(_) => (),
    Err(x) => eprintln!("Error: {}", x)
  };
}

pub fn retrieve_user_config() -> ConfigData {
  let config_data = match std::fs::read_to_string(resolve_configuration_path()) {
    Ok(val) => val,
    Err(x) => {
      eprintln!("Error: {}\nResetting preference and defaulting to Neko...", x);
      register_configuration(None);
      "user_preference: \"neko\"".to_string()
    }
  };
  let user_config: ConfigData = toml::from_str(&config_data.as_str()).unwrap();
  return user_config;
}

pub fn str_to_category(user_preference: String) -> Category {
  let preference = user_preference.as_str();
  match preference {
    "neko" => return Category::Neko,
    "bored" => return Category::Bored,
    "cry" => return Category::Cry,
    "facepalm" => return Category::Facepalm,
    "happy" => return Category::Happy,
    "dance" => return Category::Dance,
    "laugh" => return Category::Laugh,
    "smile" => return Category::Smile,
    "blush" => return Category::Blush,
    "handhold" => return Category::Handhold,
    "shoot" => return Category::Shoot,
    "smug" => return Category::Smug,
    "think" => return Category::Think,
    "cuddle" => return Category::Cuddle,
    &_ => todo!()
  }
}

pub fn category_to_str(category: Category) -> &'static str {
  match category {
      Category::Neko => "neko",
      Category::Bored => "bored",
      Category::Cry => "cry",
      Category::Facepalm => "facepalm",
      Category::Happy => "happy",
      Category::Dance => "dance",
      Category::Laugh => "laugh",
      Category::Smile => "smile",
      Category::Blush => "blush",
      Category::Handhold => "handhold",
      Category::Shoot => "shoot",
      Category::Smug => "smug",
      Category::Think => "think",
      Category::Cuddle => "cuddle",
      _ => todo!()
  }
}

pub fn resolve_user_preference() -> nekosbest::Category {
  let user_config = retrieve_user_config();
  return str_to_category(user_config.user_preference);
}

#[tauri::command]
pub fn resolve_user_preference_as_string() -> String {
  let user_config = retrieve_user_config();
  return user_config.user_preference;
}

// JS/TS wrapper for `register_configuration()` above.
#[tauri::command]
pub fn set_preference_from_string(user_preference: String) {
  register_configuration(Some(user_preference.as_str()));
}
