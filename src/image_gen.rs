use std::io::Cursor;

use color_processing::Color;
use image::{ImageFormat, Rgb, RgbImage};
use rayon::iter::ParallelIterator;
use rocket::{http::ContentType, response::status::BadRequest, FromForm};
use svg::{node::element::Rectangle, Document};

fn validate_size(size: &&str) -> bool {
    match size.split_once('x') {
        Some((height, width)) => height.parse::<u32>().is_ok() && width.parse::<u32>().is_ok(),
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
            Some((height, width)) => (
                height.parse().unwrap_or(128).clamp(1, 500),
                width.parse().unwrap_or(128).clamp(1, 500),
            ),
            None => (128, 128),
        }
    }
}

pub fn from_image_format(
    format: Option<ImageFormat>,
    extention: Option<&str>,
) -> Option<ContentType> {
    match format {
        Some(ImageFormat::Png) => Some(ContentType::PNG),
        Some(ImageFormat::Jpeg) => Some(ContentType::JPEG),
        Some(ImageFormat::Gif) => Some(ContentType::GIF),
        Some(ImageFormat::WebP) => Some(ContentType::WEBP),
        Some(ImageFormat::Ico) => Some(ContentType::Icon),
        Some(ImageFormat::Tiff) => Some(ContentType::TIFF),
        _ => match extention {
            Some("svg") => Some(ContentType::SVG),
            _ => None,
        },
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
    let (height, width) = (size.0, size.1);

    let Some(content_type) = from_image_format(ImageFormat::from_extension(r#type), Some(r#type))
    else {
        return Err(BadRequest("Unsupported extension"));
    };

    if content_type.exact_eq(&ContentType::SVG) {
        generate_svg_img(color, size)
    } else {
        let mut img = RgbImage::new(height, width);

        let color = hex_to_color(color)?;

        let fill = Rgb([color.red, color.green, color.blue]);

        img.par_pixels_mut().for_each(|pixel| *pixel = fill);

        let mut buffer = Vec::new();

        img.write_to(
            &mut Cursor::new(&mut buffer),
            ImageFormat::from_extension(r#type).expect("I did a fucky wucky here"),
        )
        .map_err(|_| BadRequest("Could not make image"))?;

        Ok((content_type, buffer))
    }
}

pub fn generate_svg_img(
    color: &str,
    size: (u32, u32),
) -> Result<(ContentType, Vec<u8>), BadRequest<&'static str>> {
    let (height, width) = (size.0, size.1);

    let content_type = ContentType::SVG;

    let img = Rectangle::new()
        .set("x", 0)
        .set("y", 0)
        .set("width", width)
        .set("height", height)
        .set("fill", format!("#{color}"));

    let doc = Document::new()
        .set("width", width)
        .set("height", height)
        .set("viewBox", (0, 0, width, height))
        .add(img);

    let mut buffer = Vec::new();

    svg::write(&mut Cursor::new(&mut buffer), &doc)
        .map_err(|_| BadRequest("Could not make image"))?;

    Ok((content_type, buffer))
}
