use color_processing::Color;
use image::ImageFormat;
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

pub fn hex_to_color(hex: &str) -> Result<Color, BadRequest<&'static str>> {
    match Color::new_string(hex) {
        Ok(o) => Ok(o),
        Err(e) => Err(BadRequest(match e.reason {
            color_processing::ParseErrorEnum::EmptyString => "empty string",
            color_processing::ParseErrorEnum::InvalidCssFunction => "invalid CSS function",
            color_processing::ParseErrorEnum::Unknown => "unknown",
            color_processing::ParseErrorEnum::InvalidHexValue => "invalid hex value",
            color_processing::ParseErrorEnum::InvalidColorName => "invalid color name",
            color_processing::ParseErrorEnum::InvalidAbbreviation => "invalid abbreviation",
        })),
    }
}
