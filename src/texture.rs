use std::sync::Arc;
use image::RgbaImage;
use noise::{NoiseFn, Perlin, Turbulence};
use crate::{Color, Point3};

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum Texture {
    SolidColor {
        color_value: Color
    },
    Checker {
        texture_odd: Arc<Texture>,
        texture_even: Arc<Texture>
    },
    Image {
        image: Arc<RgbaImage>
    },
    Perlin {
        turbulence: Turbulence<Perlin>
    }
}

impl Texture {
    pub fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        return match self {
            Texture::SolidColor { color_value } => {
                *color_value
            },
            Texture::Checker { texture_odd, texture_even } => {
                let sines = (10.0 * p.x()).sin() * (10.0 * p.y()).sin() * (10.0 * p.z()).sin();
                if sines < 0.0 {
                    return texture_even.value(u, v, p);
                } else {
                    return texture_odd.value(u, v, p);
                }
            }
            Texture::Image { image } => {
                if image.is_empty() {
                    return Color::new(0, 1, 1);
                }

                let u = u.clamp(0.0, 1.0);
                let v = 1.0 - v.clamp(0.0, 1.0);

                let i = ((u * image.width() as f64) as u32).min(image.width() - 1);
                let j = ((v * image.height() as f64) as u32).min(image.height() - 1);

                let color_scale = 1.0 / 255.0;
                let pixel = image.get_pixel(i, j);

                return Color::new(color_scale * pixel.0[0] as f32, color_scale * pixel.0[1] as f32, color_scale * pixel.0[2] as f32);
            }
            Texture::Perlin { turbulence } => {
                return Color::new(1, 1,1) * 0.5 * (1.0 + (1.0 * p.z() * 10.0 * turbulence.get([p.x(), p.y(), p.z()])).sin())
            }
            _ => {
                Color::new_empty()
            }
        }
    }
}