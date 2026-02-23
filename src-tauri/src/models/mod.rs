// A rectangle the user drew on the image
pub struct Region {
    pub id: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub label: Option<String>,
}

// Result of OCR on one region
pub struct RegionResult {
    pub region_id: String,
    pub text: String,
    pub columns: Vec<Vec<String>>, // detected columns, each is a list of cell values
}

// One card = one image + all its region results
pub struct OutputCard {
    pub image_id: String,
    pub image_name: String,
    pub results: Vec<RegionResult>,
}
