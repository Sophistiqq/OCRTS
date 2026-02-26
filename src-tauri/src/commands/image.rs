pub use base64::{engine::general_purpose, Engine};
use image::ImageReader;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Mutex;
use tauri::State;
use uuid::Uuid;

#[derive(Serialize, Clone)]
pub struct ImageFile {
    pub id: String,
    pub name: String,
    pub path: String,
    pub thumbnail: String, // base64 data URL
    pub width: u32,
    pub height: u32,
    pub rotation: u32,
}

// In-memory store: image_id → file path (so OCR commands can find the file)
pub type ImageStore = Mutex<HashMap<String, String>>;

#[tauri::command]
pub fn load_image(path: String, store: State<'_, ImageStore>) -> Result<ImageFile, String> {
    let img = ImageReader::open(&path)
        .map_err(|e| e.to_string())?
        .decode()
        .map_err(|e| e.to_string())?;

    let (width, height) = (img.width(), img.height());

    // Create thumbnail (max 200px wide, preserve aspect ratio)
    let thumb = img.thumbnail(200, 200);
    let mut thumb_bytes: Vec<u8> = Vec::new();
    thumb
        .write_to(
            &mut std::io::Cursor::new(&mut thumb_bytes),
            image::ImageFormat::Png,
        )
        .map_err(|e| e.to_string())?;

    let thumbnail = format!(
        "data:image/png;base64,{}",
        general_purpose::STANDARD.encode(&thumb_bytes)
    );

    let id = Uuid::new_v4().to_string();
    let name = std::path::Path::new(&path)
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    store.lock().unwrap().insert(id.clone(), path.clone());

    Ok(ImageFile {
        id,
        name,
        path,
        thumbnail,
        width,
        height,
        rotation: 0,
    })
}

#[tauri::command]
pub async fn preprocess_image(
    path: String,
    blur: f32,
    threshold: i32, // -1 for Otsu, 0-255 for fixed
) -> Result<String, String> {
    tokio::task::spawn_blocking(move || {
        let img = ImageReader::open(&path)
            .map_err(|e| e.to_string())?
            .decode()
            .map_err(|e| e.to_string())?;

        let processed_img = if blur <= 0.0 && threshold == -2 {
            img
        } else {
            let mut gray = img.to_luma8();
            
            // Denoising
            gray = imageproc::filter::median_filter(&gray, 1, 1);

            if blur > 0.0 {
                gray = imageproc::filter::gaussian_blur_f32(&gray, blur);
            }

            if threshold == -1 {
                let level = imageproc::contrast::otsu_level(&gray);
                imageproc::contrast::threshold_mut(
                    &mut gray,
                    level,
                    imageproc::contrast::ThresholdType::Binary,
                );
            } else if threshold == -3 {
                gray = imageproc::contrast::adaptive_threshold(&gray, 15);
            } else if threshold != -2 {
                let level = threshold as u8;
                imageproc::contrast::threshold_mut(
                    &mut gray,
                    level,
                    imageproc::contrast::ThresholdType::Binary,
                );
            }
            image::DynamicImage::ImageLuma8(gray)
        };

        let mut bytes: Vec<u8> = Vec::new();
        
        // Thumbnail for preview performance
        let preview = processed_img.thumbnail(1200, 1200);

        preview
            .write_to(
                &mut std::io::Cursor::new(&mut bytes),
                image::ImageFormat::Png,
            )
            .map_err(|e| e.to_string())?;

        Ok(format!(
            "data:image/png;base64,{}",
            general_purpose::STANDARD.encode(&bytes)
        ))
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub fn load_image_full(path: String) -> Result<String, String> {
    let img = ImageReader::open(&path)
        .map_err(|e| e.to_string())?
        .decode()
        .map_err(|e| e.to_string())?;

    let mut bytes: Vec<u8> = Vec::new();
    img.write_to(
        &mut std::io::Cursor::new(&mut bytes),
        image::ImageFormat::Png,
    )
    .map_err(|e| e.to_string())?;

    Ok(format!(
        "data:image/png;base64,{}",
        general_purpose::STANDARD.encode(&bytes)
    ))
}
