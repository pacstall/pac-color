use std::io::Cursor;

use color_processing::Color;
use image::{ImageFormat, Rgb, RgbImage};
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

pub fn generate_img(
    color: &str,
    size: (u32, u32),
    r#type: &str,
) -> Result<(ContentType, Vec<u8>), BadRequest<&'static str>> {
    let (height, weight) = (size.0, size.1);

    let Some(format) = ImageFormat::from_extension(r#type) else {
        return Err(BadRequest("Invalid extension"));
    };

    let Some(content_type) = from_image_format(format) else {
        return Err(BadRequest("Unsupported extension"));
    };

    let mut img = RgbImage::new(height, weight);

    let color = hex_to_color(color)?;

    let fill: Rgb<u8> = [color.red, color.green, color.blue].into();

    img.pixels_mut().for_each(|pixel| *pixel = fill);

    let mut buffer = Vec::new();

    img.write_to(&mut Cursor::new(&mut buffer), format)
        .map_err(|_| BadRequest("Could not make image"))?;

    Ok((content_type, buffer))
}
