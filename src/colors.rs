use color_processing::Color;
use rocket::serde::{Serialize, json::Json};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Colors {
    pub rgb: Rgb,
    pub cmyk: Cmyk,
    pub hsv: Hsv,
    pub hsl: Hsl,
    pub oklab: OKLab,
    pub oklch: Oklch,
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

#[derive(Serialize, Clone, Copy)]
#[serde(crate = "rocket::serde")]
pub struct OKLab {
    pub l: f32,
    pub a: f32,
    pub b: f32,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Oklch {
    pub l: f32,
    pub c: f32,
    pub h: f32,
}

impl From<OKLab> for Oklch {
    fn from(value: OKLab) -> Self {
        let c = (value.a * value.a + value.b * value.b).sqrt();
        let mut h = value.b.atan2(value.a).to_degrees();
        if h < 0.0 {
            h += 360.0;
        }

        Self { l: value.l, c, h }
    }
}

impl From<Rgb> for OKLab {
    fn from(value: Rgb) -> Self {
        fn srgb_to_linear(c: u8) -> f32 {
            let c = c as f32 / 255.0;
            if c <= 0.040_45 {
                c / 12.92
            } else {
                ((c + 0.055) / 1.055).powf(2.4)
            }
        }

        let r = srgb_to_linear(value.r);
        let g = srgb_to_linear(value.g);
        let b = srgb_to_linear(value.b);

        let l = 0.412_221_470_8 * r + 0.536_332_536_3 * g + 0.051_445_992_9 * b;
        let m = 0.211_903_498_2 * r + 0.680_699_545_1 * g + 0.107_396_956_6 * b;
        let s = 0.088_302_461_9 * r + 0.281_718_837_6 * g + 0.629_978_700_5 * b;

        let l_ = l.cbrt();
        let m_ = m.cbrt();
        let s_ = s.cbrt();

        let l_ok = 0.210_454_255_3 * l_ + 0.793_617_785_0 * m_ - 0.004_072_046_8 * s_;
        let a = 1.977_998_495_1 * l_ - 2.428_592_205_0 * m_ + 0.450_593_709_9 * s_;
        let b = 0.025_904_037_1 * l_ + 0.782_771_766_2 * m_ - 0.808_675_766_0 * s_;

        Self { l: l_ok, a, b }
    }
}

impl Colors {
    pub fn new(rgb: &Color) -> Json<Self> {
        let (c, m, y, k) = rgb.get_cmyk();
        let (hsh, hss, hsv, _) = rgb.get_hsva();
        let (hslh, hsls, hslv, _) = rgb.get_hsla();
        let oklab = OKLab::from(Rgb {
            r: rgb.red,
            g: rgb.green,
            b: rgb.blue,
        });
        Json(Self {
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
            oklab,
            oklch: Oklch::from(oklab),
        })
    }
}
