use image::{ImageFormat, Rgb, RgbImage};
use rocket::{get, http::ContentType, launch, response::status::BadRequest, routes, FromForm};
use std::io::Cursor;

#[derive(FromForm)]
struct ImageSpecifications<'r> {
    #[field(default = "png")]
    r#type: &'r str,
}

fn from_image_format(format: ImageFormat) -> Option<ContentType> {
    match format {
        ImageFormat::Png => Some(ContentType::PNG),
        ImageFormat::Jpeg => Some(ContentType::JPEG),
        ImageFormat::Gif => Some(ContentType::GIF),
        ImageFormat::WebP => Some(ContentType::WEBP),
        ImageFormat::Ico => Some(ContentType::Icon),
        _ => None,
    }
}

fn hex_to_rgb(hex: &str) -> Result<Rgb<u8>, BadRequest<&'static str>> {
    let hex = hex.trim_start_matches('#');

    if hex.len() != 6 {
        return Err(BadRequest("Invalid color format. Use RRGGBB"));
    }

    let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| BadRequest("Invalid hex digits"))?;
    let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| BadRequest("Invalid hex digits"))?;
    let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| BadRequest("Invalid hex digits"))?;

    Ok(Rgb([r, g, b]))
}

#[get("/<size>/<color>?<spec..>")]
#[allow(clippy::needless_pass_by_value)]
fn colorize(
    size: &str,
    color: &str,
    spec: ImageSpecifications,
) -> Result<(ContentType, Vec<u8>), BadRequest<&'static str>> {
    let (height, weight) = match size.split_once('x') {
        Some((height, weight)) => {
            let height: u32 = height
                .parse()
                .map_err(|_| BadRequest("Could not parse height into u32"))?;
            let weight: u32 = weight
                .parse()
                .map_err(|_| BadRequest("Could not parse weight into u32"))?;
            (height, weight)
        }
        None => {
            return Err(BadRequest(
                "Invalid size qualifier. Use `HEIGHTxWEIGHT` style",
            ))
        }
    };

    // Sanity checks
    if height > 500 || weight > 500 {
        return Err(BadRequest("Size exceeds 500 in one direction"));
    }

    let Some(format) = ImageFormat::from_extension(spec.r#type) else {
        return Err(BadRequest("Invalid extension"));
    };

    let Some(content_type) = from_image_format(format) else {
        return Err(BadRequest("Unsupported extension"));
    };

    let mut img = RgbImage::new(height, weight);

    let fill = hex_to_rgb(color)?;

    img.pixels_mut().for_each(|pixel| *pixel = fill);

    let mut buffer = Vec::new();

    img.write_to(&mut Cursor::new(&mut buffer), format)
        .map_err(|_| BadRequest("Could not make image"))?;

    Ok((content_type, buffer))
}

#[get("/")]
fn splash() -> &'static str {
    "Use the slug: `/100x100/FFFFFF` for example."
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![colorize, splash])
}
