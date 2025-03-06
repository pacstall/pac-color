use colors::Colors;
use image_gen::{generate_img, hex_to_color, ImageSpecifications};
use rand::Rng;
use rocket::{
    get, http::ContentType, launch, response::status::BadRequest, routes, serde::json::Json,
};

mod colors;
mod image_gen;

#[get("/<color>/preview?<spec..>")]
#[allow(clippy::needless_pass_by_value)]
fn colorize(
    color: &str,
    spec: ImageSpecifications,
) -> Result<(ContentType, Vec<u8>), BadRequest<&'static str>> {
    let (height, weight) = spec.get_sizes();

    generate_img(color, (height, weight), spec.r#type)
}

#[get("/random/preview?<spec..>")]
#[allow(clippy::needless_pass_by_value)]
fn random(spec: ImageSpecifications) -> Result<(ContentType, Vec<u8>), BadRequest<&'static str>> {
    let (height, weight) = spec.get_sizes();

    let mut rng = rand::rng();
    let string = format!(
        "rgb({}, {}, {})",
        rng.random_range(0..=255),
        rng.random_range(0..=255),
        rng.random_range(0..=255)
    );

    generate_img(&string, (height, weight), spec.r#type)
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
    rocket::build().mount("/", routes![colorize, jsonize, random, splash])
}
