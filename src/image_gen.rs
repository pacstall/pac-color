use image::{ImageFormat, Rgb};
use rocket::{http::ContentType, response::status::BadRequest, FromForm};

fn validate_size(size: &&str) -> bool {
    match size.split_once('x') {
        Some((height, weight)) => height.parse::<u32>().is_ok() && weight.parse::<u32>().is_ok(),
        None => false,
    }
}

#[derive(FromForm)]
pub struct ImageSpecifications<'a> {
    #[field(default = "png")]
    pub r#type: &'a str,
    #[field(default = "128x128", validate = with(validate_size, "Invalid size qualifier"))]
    pub size: &'a str,
}

impl ImageSpecifications<'_> {
    pub fn get_sizes(&self) -> (u32, u32) {
        match self.size.split_once('x') {
            Some((height, weight)) => (
                height.parse().unwrap_or(128).clamp(1, 500),
                weight.parse().unwrap_or(128).clamp(1, 500),
            ),
            None => (128, 128),
        }
    }
}

pub fn from_image_format(format: ImageFormat) -> Option<ContentType> {
    match format {
        ImageFormat::Png => Some(ContentType::PNG),
        ImageFormat::Jpeg => Some(ContentType::JPEG),
        ImageFormat::Gif => Some(ContentType::GIF),
        ImageFormat::WebP => Some(ContentType::WEBP),
        ImageFormat::Ico => Some(ContentType::Icon),
        _ => None,
    }
}

pub fn hex_to_rgb(hex: &str) -> Result<Rgb<u8>, BadRequest<&'static str>> {
    let hex = hex.trim_start_matches('#');

    if hex.len() != 6 {
        return Err(BadRequest("Invalid color format. Use RRGGBB"));
    }

    let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| BadRequest("Invalid hex digits"))?;
    let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| BadRequest("Invalid hex digits"))?;
    let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| BadRequest("Invalid hex digits"))?;

    Ok(Rgb([r, g, b]))
}
