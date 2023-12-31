use image::{DynamicImage, EncodableLayout, GenericImageView, ImageOutputFormat};
use std::{io::Cursor, path::PathBuf};

use crate::configuration_manager;

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
    } else {
        return None;
    }
}

pub async fn download_file_from_url(
    client: &reqwest::Client,
    url: String,
    path: PathBuf,
) -> Result<PathBuf, reqwest::Error> {
    let result = client.get(url).send().await?.bytes().await?;
    let img = image::load_from_memory(result.as_bytes()).unwrap();
    let mut image_bytes: Vec<u8> = Vec::new();
    let result_image = sanitize_image_dimensions(&img).unwrap_or(img);
    result_image
        .write_to(&mut Cursor::new(&mut image_bytes), ImageOutputFormat::Png)
        .unwrap();
    match std::fs::create_dir(&path.parent().unwrap()) {
      Ok(_) => (),
      Err(_) => (),
    };
    match std::fs::write(&path, image_bytes) {
        Ok(_) => return Ok(path),
        Err(x) => panic!("{}", x),
    };
}

pub async fn new_wallpaper_url() -> Result<String, reqwest::Error> {
  let img_url: String = nekosbest::get(configuration_manager::resolve_user_preference()).await.unwrap().url;
  return Ok(img_url);
}

pub async fn refresh_wallpaper() -> Result<(), ()> {
    crate::notifications::send_notification("Finding new wallpaper...", "This will take a second.", crate::configuration_manager::resolve_bundle_identifier().as_str());
    let client = reqwest::Client::new();
    let dl_url = new_wallpaper_url().await.unwrap();
    let file_name = dl_url.split("/").collect::<Vec<&str>>();
    let image_file = crate::cache_management::resolve_cache_dir().join(file_name[file_name.len() - 1]);
    let downloaded_file = download_file_from_url(&client, dl_url, image_file)
        .await
        .unwrap();
    match wallpaper::set_from_path(downloaded_file.display().to_string().as_str()) {
      Ok(_) => {
        println!("Successfully set wallpaper.");
        crate::notifications::send_notification("Found one!", "Hope you like it.", crate::configuration_manager::resolve_bundle_identifier().as_str());
        match wallpaper::set_mode(wallpaper::Mode::Crop) {
          Ok(()) => (),
          Err(_) => eprintln!("Failed to set wallpaper crop mode!"),
        };
      },
      Err(x) => eprintln!("Error setting wallpaper: {}", x)
    };
    return Ok(());
}
