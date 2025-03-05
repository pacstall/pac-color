use image::{Rgb, RgbImage};
use rocket::{get, launch, response::status::BadRequest, routes};
use std::io::Cursor;

fn hex_to_rgb(hex: &str) -> Result<Rgb<u8>, BadRequest<String>> {
    let hex = hex.trim_start_matches('#');

    if hex.len() != 6 {
        return Err(BadRequest("Invalid color format. Use RRGGBB".into()));
    }

    let r =
        u8::from_str_radix(&hex[0..2], 16).map_err(|_| BadRequest("Invalid hex digits".into()))?;
    let g =
        u8::from_str_radix(&hex[2..4], 16).map_err(|_| BadRequest("Invalid hex digits".into()))?;
    let b =
        u8::from_str_radix(&hex[4..6], 16).map_err(|_| BadRequest("Invalid hex digits".into()))?;

    Ok(Rgb([r, g, b]))
}

#[get("/<color>")]
fn colorize(color: &str) -> Result<Vec<u8>, BadRequest<String>> {
    let mut img = RgbImage::new(100, 100);

    let fill = hex_to_rgb(color)?;

    for pixel in img.pixels_mut() {
        *pixel = fill;
    }

    let mut buffer = Vec::new();

    img.write_to(&mut Cursor::new(&mut buffer), image::ImageFormat::Png)
        .expect("Failed to write PNG");

    Ok(buffer)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![colorize])
}
