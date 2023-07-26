use tauri::api::path::cache_dir;

use std::{collections::HashMap, path::PathBuf};

use rand::Rng;


pub async fn new_wallpaper_url(client: &reqwest::Client) -> Result<String, reqwest::Error> {
  let result = client.get("https://api.waifu.pics/sfw/waifu")
  .send()
  .await?
  .text()
  .await?;
  let decoded_result: &String = &serde_json::from_str::<HashMap::<String, String>>(&result).unwrap()["url"];
  return Ok(decoded_result.clone());
}

pub async fn download_file_from_url(client: &reqwest::Client, url: String, path: PathBuf) -> Result<PathBuf, reqwest::Error> {
  let result = client.get(url)
  .send()
  .await?
  .bytes()
  .await?;
  match std::fs::write(&path, result) {
    Ok(_) => return Ok(path),
    Err(x) => panic!("{}", x)
  };
}

pub async fn refresh_wallpaper() -> Result<(), ()> {
  println!("Refreshing wallpaper in function...");
  let client = reqwest::Client::new();
  let image_file = cache_dir().unwrap().join(format!("{}.png", rand::thread_rng().gen_range(1000..10000)));
  let dl_url = new_wallpaper_url(&client).await.unwrap();
  let downloaded_file = download_file_from_url(&client, dl_url, image_file).await.unwrap();
  wallpaper::set_from_path(downloaded_file.display().to_string().as_str()).unwrap();
  match wallpaper::set_mode(wallpaper::Mode::Fit) {
    Ok(()) => (),
    Err(_) => eprintln!("Failed to set wallpaper crop mode!")
  };
  return Ok(());
}
