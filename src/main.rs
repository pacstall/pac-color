use colors::Colors;
use image::{ImageFormat, Rgb, RgbImage};
use image_gen::{from_image_format, hex_to_color, ImageSpecifications};
use rocket::{
    get, http::ContentType, launch, response::status::BadRequest, routes, serde::json::Json,
};
use std::io::Cursor;

mod colors;
mod image_gen;

#[get("/<color>/preview?<spec..>")]
#[allow(clippy::needless_pass_by_value)]
fn colorize(
    color: &str,
    spec: ImageSpecifications,
) -> Result<(ContentType, Vec<u8>), BadRequest<&'static str>> {
    let (height, weight) = spec.get_sizes();

    let Some(format) = ImageFormat::from_extension(spec.r#type) else {
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

#[get("/<color>")]
fn jsonize(color: &str) -> Result<Json<Colors>, BadRequest<&'static str>> {
    let rgb = hex_to_color(color)?;

    Ok(Colors::new(&rgb))
}

#[get("/")]
fn splash() -> &'static str {
    include_str!("../README.md")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![colorize, jsonize, splash])
}
