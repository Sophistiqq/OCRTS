use crate::commands::image::ImageStore;
use image::{DynamicImage, GenericImageView, ImageReader};
use ocrs::{ImageSource, OcrEngine, OcrEngineParams};
use rten::Model;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tauri::State;

pub struct OcrEngineState(pub Arc<Mutex<OcrEngine>>);

#[derive(Debug, Deserialize)]
pub struct RegionInput {
    pub id: String,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub label: Option<String>,
}

#[derive(Serialize)]
pub struct RegionResult {
    #[serde(rename = "regionId")]
    pub region_id: String,
    pub text: String,
    pub columns: Vec<Vec<String>>,
}
pub fn build_engine() -> Result<OcrEngine, String> {
    let detection_model_bytes = include_bytes!("../../models/text-detection.rten");
    let recognition_model_bytes = include_bytes!("../../models/text-recognition.rten");

    let detection_model = Model::load(detection_model_bytes.to_vec()).map_err(|e| e.to_string())?;
    let recognition_model =
        Model::load(recognition_model_bytes.to_vec()).map_err(|e| e.to_string())?;

    OcrEngine::new(OcrEngineParams {
        detection_model: Some(detection_model),
        recognition_model: Some(recognition_model),
        ..Default::default()
    })
    .map_err(|e| e.to_string())
}

fn detect_columns(lines: &[String]) -> Vec<Vec<String>> {
    // Simple whitespace-based column detection:
    // Find consistent gap positions across lines, split there
    if lines.is_empty() {
        return vec![];
    }

    // Find gap positions: positions where ALL lines have a space
    let max_len = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    let padded: Vec<Vec<char>> = lines
        .iter()
        .map(|l| {
            let mut chars: Vec<char> = l.chars().collect();
            chars.resize(max_len, ' ');
            chars
        })
        .collect();

    // A column gap is a position where every line has a space char
    // and it's at least 2 spaces wide
    let mut gap_positions: Vec<bool> = vec![true; max_len];
    for col in 0..max_len {
        for row in &padded {
            if row[col] != ' ' {
                gap_positions[col] = false;
                break;
            }
        }
    }

    // Find contiguous gap ranges, pick midpoints as split points
    let mut splits: Vec<usize> = vec![];
    let mut in_gap = false;
    let mut gap_start = 0;
    for (i, &is_gap) in gap_positions.iter().enumerate() {
        if is_gap && !in_gap {
            in_gap = true;
            gap_start = i;
        } else if !is_gap && in_gap {
            in_gap = false;
            let gap_width = i - gap_start;
            if gap_width >= 2 {
                splits.push(gap_start + gap_width / 2);
            }
        }
    }

    if splits.is_empty() {
        // No columns detected — return each line as a single cell row
        return lines.iter().map(|l| vec![l.trim().to_string()]).collect();
    }

    // Split each line at the detected column positions
    lines
        .iter()
        .map(|line| {
            let chars: Vec<char> = line.chars().collect();
            let mut prev = 0;
            let mut cells: Vec<String> = vec![];
            for &split in &splits {
                let end = split.min(chars.len());
                let cell: String = chars[prev..end].iter().collect();
                cells.push(cell.trim().to_string());
                prev = end;
            }
            // Last column
            let last: String = if prev < chars.len() {
                chars[prev..].iter().collect::<String>().trim().to_string()
            } else {
                String::new()
            };
            cells.push(last);
            cells
        })
        .collect()
}

#[tauri::command]
pub async fn process_region(
    image_id: String,
    region: RegionInput,
    store: State<'_, ImageStore>,
    engine_state: State<'_, OcrEngineState>,
) -> Result<RegionResult, String> {
    let path = {
        let map = store.lock().unwrap();
        map.get(&image_id)
            .cloned()
            .ok_or_else(|| format!("Image ID not found: {}", image_id))?
    };

    // Clone what we need to move into the blocking thread
    let engine_state = engine_state.inner().0.clone();

    tokio::task::spawn_blocking(move || {
        let img = ImageReader::open(&path)
            .map_err(|e| e.to_string())?
            .decode()
            .map_err(|e| e.to_string())?;

        let (img_w, img_h) = img.dimensions();
        let x = region.x.min(img_w.saturating_sub(1));
        let y = region.y.min(img_h.saturating_sub(1));
        let w = region.width.min(img_w - x);
        let h = region.height.min(img_h - y);

        let cropped = img.crop_imm(x, y, w, h);

        let engine = engine_state.lock().unwrap();
        let rgb = cropped.to_rgb8();
        let source =
            ImageSource::from_bytes(rgb.as_raw(), rgb.dimensions()).map_err(|e| e.to_string())?;
        let ocr_input = engine.prepare_input(source).map_err(|e| e.to_string())?;
        let text = engine.get_text(&ocr_input).map_err(|e| e.to_string())?;

        let lines: Vec<String> = text
            .lines()
            .map(|l| l.to_string())
            .filter(|l| !l.trim().is_empty())
            .collect();

        let columns = detect_columns(&lines);

        Ok(RegionResult {
            region_id: region.id,
            text,
            columns,
        })
    })
    .await
    .map_err(|e| e.to_string())?
}
