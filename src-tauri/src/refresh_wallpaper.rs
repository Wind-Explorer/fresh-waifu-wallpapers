use tauri::api::path::cache_dir;
use std::{collections::HashMap, path::PathBuf, io::Cursor};
use image::{GenericImageView, DynamicImage, EncodableLayout, ImageOutputFormat};
use rand::Rng;

pub fn sanitize_image_dimensions(image_obj: &DynamicImage) -> Option<DynamicImage> {
  // Get image dimensions
  let (width, height) = image_obj.dimensions();
  // Check if height is larger than width
  if height > width {
      // Define the coordinates and dimensions of the crop rectangle
      let x = 0;
      let y = 0;
      let new_height = height / 2;
      
      // Crop the image
      return Some(image_obj.crop_imm(x, y, width, new_height));
  }
  else {
    return None;
  }
}

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
  let img = image::load_from_memory(result.as_bytes()).unwrap();
  let mut image_bytes: Vec<u8> = Vec::new();
  let result_image = sanitize_image_dimensions(&img).unwrap_or(img);
  result_image.write_to(&mut Cursor::new(&mut image_bytes), ImageOutputFormat::Png).unwrap();
  match std::fs::write(&path, image_bytes) {
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
  match wallpaper::set_mode(wallpaper::Mode::Crop) {
    Ok(()) => (),
    Err(_) => eprintln!("Failed to set wallpaper crop mode!")
  };
  return Ok(());
}
