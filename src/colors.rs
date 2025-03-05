use color_processing::Color;
use rocket::serde::{json::Json, Serialize};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Colors {
    pub rgb: Rgb,
    pub cmyk: Cmyk,
    pub hsv: Hsv,
    pub hsl: Hsl,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Cmyk {
    pub c: f64,
    pub m: f64,
    pub y: f64,
    pub k: f64,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Hsv {
    pub h: f64,
    pub s: f64,
    pub v: f64,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Hsl {
    pub h: f64,
    pub s: f64,
    pub l: f64,
}

impl Colors {
    pub fn new(rgb: &Color) -> Json<Colors> {
        let (c, m, y, k) = rgb.get_cmyk();
        let (hsh, hss, hsv, _) = rgb.get_hsva();
        let (hslh, hsls, hslv, _) = rgb.get_hsla();
        Json(Colors {
            rgb: Rgb {
                r: rgb.red,
                g: rgb.green,
                b: rgb.blue,
            },
            cmyk: Cmyk { c, m, y, k },
            hsv: Hsv {
                h: hsh,
                s: hss,
                v: hsv,
            },
            hsl: Hsl {
                h: hslh,
                s: hsls,
                l: hslv,
            },
        })
    }
}
