use crate::commands::image::ImageStore;
use image::{imageops::FilterType, DynamicImage, GenericImageView, ImageReader};
use leptess::{LepTess, Variable};
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Deserialize)]
pub struct RegionInput {
    pub id: String,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    #[allow(dead_code)]
    pub label: Option<String>,
}

#[derive(Serialize)]
pub struct RegionResult {
    #[serde(rename = "regionId")]
    pub region_id: String,
    pub text: String,
    pub columns: Vec<Vec<String>>,
}

fn preprocess(img: &DynamicImage) -> DynamicImage {
    let (w, h) = img.dimensions();

    // Scale up if too small — Tesseract works best at 300 DPI equivalent
    // For phone photos, aim for at least 1000px tall
    let scale = if h < 1000 { 1000.0 / h as f32 } else { 1.0 };

    if scale > 1.0 {
        let nw = (w as f32 * scale) as u32;
        let nh = (h as f32 * scale) as u32;
        img.resize(nw, nh, FilterType::Lanczos3)
    } else {
        img.clone()
    }
}

#[derive(Debug, Clone)]
struct Word {
    text: String,
    x: f32,
    y: f32,
    right: f32,
    bottom: f32,
}

fn group_into_rows(words: &[Word], threshold: f32) -> Vec<Vec<Word>> {
    if words.is_empty() {
        return vec![];
    }

    let mut sorted = words.to_vec();
    sorted.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap());

    let mut rows: Vec<Vec<Word>> = vec![];
    let mut current_row: Vec<Word> = vec![sorted[0].clone()];
    let mut row_center_y = (sorted[0].y + sorted[0].bottom) / 2.0;

    for word in &sorted[1..] {
        let word_center_y = (word.y + word.bottom) / 2.0;
        if (word_center_y - row_center_y).abs() <= threshold {
            current_row.push(word.clone());
        } else {
            current_row.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
            rows.push(current_row.clone());
            current_row = vec![word.clone()];
            row_center_y = word_center_y;
        }
    }
    current_row.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
    rows.push(current_row);

    rows
}

fn detect_column_splits(rows: &[Vec<Word>], gap_threshold: f32) -> Vec<f32> {
    if rows.is_empty() {
        return vec![];
    }

    let all_words: Vec<&Word> = rows.iter().flatten().collect();
    if all_words.is_empty() {
        return vec![];
    }

    let min_x = all_words.iter().map(|w| w.x).fold(f32::MAX, f32::min);
    let max_x = all_words.iter().map(|w| w.right).fold(f32::MIN, f32::max);
    let width = (max_x - min_x).ceil() as usize;
    if width == 0 {
        return vec![];
    }

    let mut coverage = vec![0usize; width + 1];
    for row in rows {
        let mut row_coverage = vec![false; width + 1];
        for word in row {
            let start = ((word.x - min_x) as usize).min(width);
            let end = ((word.right - min_x) as usize).min(width);
            for c in start..=end {
                row_coverage[c] = true;
            }
        }
        for (i, &covered) in row_coverage.iter().enumerate() {
            if covered {
                coverage[i] += 1;
            }
        }
    }

    let min_coverage = (rows.len() as f32 * 0.2).ceil() as usize;
    let mut in_gap = false;
    let mut gap_start = 0;
    let mut splits: Vec<f32> = vec![];

    for (i, &cov) in coverage.iter().enumerate() {
        if cov <= min_coverage && !in_gap {
            in_gap = true;
            gap_start = i;
        } else if cov > min_coverage && in_gap {
            in_gap = false;
            let gap_width = i - gap_start;
            if gap_width as f32 >= gap_threshold {
                splits.push(min_x + (gap_start + gap_width / 2) as f32);
            }
        }
    }

    splits
}

fn assign_to_columns(row: &[Word], splits: &[f32], num_cols: usize) -> Vec<String> {
    let mut cells: Vec<Vec<&str>> = vec![vec![]; num_cols];
    for word in row {
        let word_center_x = (word.x + word.right) / 2.0;
        let col_idx = splits
            .partition_point(|&split| word_center_x > split)
            .min(num_cols - 1);
        cells[col_idx].push(&word.text);
    }
    cells.iter().map(|c| c.join(" ")).collect()
}

fn build_grid(words: Vec<Word>, row_threshold: f32, gap_threshold: f32) -> Vec<Vec<String>> {
    let rows = group_into_rows(&words, row_threshold);
    if rows.is_empty() {
        return vec![];
    }

    let splits = detect_column_splits(&rows, gap_threshold);
    if splits.is_empty() {
        return rows
            .iter()
            .map(|row| {
                vec![row
                    .iter()
                    .map(|w| w.text.clone())
                    .collect::<Vec<_>>()
                    .join(" ")]
            })
            .collect();
    }

    let num_cols = splits.len() + 1;
    rows.iter()
        .map(|row| assign_to_columns(row, &splits, num_cols))
        .collect()
}

#[tauri::command]
pub async fn process_region(
    image_id: String,
    region: RegionInput,
    store: State<'_, ImageStore>,
) -> Result<RegionResult, String> {
    // Resolve tessdata path before entering the blocking thread
    let tessdata_path = {
        let exe = std::env::current_exe().map_err(|e| e.to_string())?;
        let exe_dir = exe.parent().ok_or("no exe dir")?.to_path_buf();
        let bundled = exe_dir.join("tessdata");

        // Log what we're seeing for debugging
        eprintln!("exe: {:?}", exe);
        eprintln!("exe_dir: {:?}", exe_dir);
        eprintln!("bundled tessdata path: {:?}", bundled);
        eprintln!("bundled exists: {}", bundled.exists());

        if bundled.exists() {
            Some(bundled.to_string_lossy().to_string())
        } else {
            // Try one level up (some Tauri versions put resources alongside the exe differently)
            let parent_bundled = exe_dir.parent().map(|p| p.join("tessdata"));
            eprintln!("parent tessdata: {:?}", parent_bundled);
            eprintln!(
                "parent exists: {}",
                parent_bundled.as_ref().map(|p| p.exists()).unwrap_or(false)
            );

            if let Some(p) = parent_bundled {
                if p.exists() {
                    Some(p.to_string_lossy().to_string())
                } else {
                    None
                }
            } else {
                None
            }
        }
    };

    let path = {
        let map = store.lock().unwrap();
        map.get(&image_id)
            .cloned()
            .ok_or_else(|| format!("Image ID not found: {}", image_id))?
    };

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
        let processed = preprocess(&cropped);

        let tmp_path = std::env::temp_dir().join(format!("ocr_region_{}.png", region.id));
        processed.save(&tmp_path).map_err(|e| e.to_string())?;

        // Single LepTess::new call using the resolved tessdata path
        let tessdata_ref = tessdata_path.as_deref();
        let mut lt = LepTess::new(tessdata_ref, "eng").map_err(|e| e.to_string())?;

        lt.set_variable(Variable::TesseditPagesegMode, "6")
            .map_err(|e| e.to_string())?;
        lt.set_variable(Variable::TesseditCharWhitelist, "")
            .map_err(|e| e.to_string())?;

        lt.set_image(tmp_path.to_str().unwrap())
            .map_err(|e| e.to_string())?;

        let tsv = lt.get_tsv_text(0).map_err(|e| e.to_string())?;
        let plain_text = lt.get_utf8_text().map_err(|e| e.to_string())?;

        let mut words: Vec<Word> = vec![];
        for line in tsv.lines().skip(1) {
            let cols: Vec<&str> = line.split('\t').collect();
            if cols.len() < 12 {
                continue;
            }
            let level: i32 = cols[0].parse().unwrap_or(0);
            if level != 5 {
                continue;
            }
            let left: f32 = cols[6].parse().unwrap_or(0.0);
            let top: f32 = cols[7].parse().unwrap_or(0.0);
            let width: f32 = cols[8].parse().unwrap_or(0.0);
            let height: f32 = cols[9].parse().unwrap_or(0.0);
            let text = cols[11].trim().to_string();

            if text.is_empty() || text == "-1" {
                continue;
            }

            words.push(Word {
                text,
                x: left,
                y: top,
                right: left + width,
                bottom: top + height,
            });
        }

        let _ = std::fs::remove_file(&tmp_path);

        let avg_height = if words.is_empty() {
            12.0
        } else {
            words.iter().map(|w| w.bottom - w.y).sum::<f32>() / words.len() as f32
        };

        let columns = build_grid(words, avg_height * 0.6, avg_height * 0.8);

        Ok(RegionResult {
            region_id: region.id,
            text: plain_text,
            columns,
        })
    })
    .await
    .map_err(|e| e.to_string())?
}
