use std::process::id;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use glium::vertex::MultiVerticesSource;
use image::{DynamicImage, Rgba, RgbaImage};
use image::DynamicImage::ImageRgb8;
use image::math::utils::clamp;
use rand::{Rng, thread_rng};
use rand::prelude::SliceRandom;
use rayon::prelude::*;
use crate::{Camera, Color, HitList, HitRecord, Hittable, Point3, Ray, Vec3};

pub struct RTParams {
    aspect_ratio: f64,
    width: u32,
    height: u32,
    samples_per_pixel: u32,
    max_depth: i32
}

impl RTParams {
    pub fn new(aspect_ratio: f64, image_width: u32, samples_per_pixel: u32, max_depth: i32) -> RTParams {
        RTParams {
            height: (image_width as f64 / aspect_ratio) as u32,
            width: image_width,
            aspect_ratio,
            samples_per_pixel,
            max_depth
        }
    }
}

fn ray_color(ray: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    let mut rec = HitRecord::default();

    if depth <= 0 {
        return Color::new_empty();
    }

    if world.hit(ray, 0.001, f64::INFINITY, &mut rec) {
        let mut scattered = Ray::new_empty();
        let mut attenuation = Color::new_empty();
        if rec.material.scatter(&ray, &rec, &mut attenuation, &mut scattered) {
            if attenuation.length() < 0.1 {
                return attenuation;
            }
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
        return Color::new_empty();
    }

    let unit_direction = ray.dir().normalized();
    let t = 0.5 * (unit_direction.y() + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

pub fn run_rt(mut params: RTParams, world: &HitList, image: Arc<Mutex<RgbaImage>>) {
    let start = Instant::now();

    let lookfrom = Point3::new(12, 2, 3);
    let lookat = Point3::new(0, 0, 0);
    let vup = Vec3::new(0, 1, 0);
    let dist_to_focus = 10.0;
    let aperature = 0.00001;//0.1;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        params.aspect_ratio, aperature, dist_to_focus);

    (0..16).into_par_iter().for_each(|j| {
        let mut rng = rand::thread_rng();
        let mut pixels: Vec<((u32, u32), Rgba<u8>)> = Vec::new();
        let mut next_pos = j;
        pixels.reserve(params.width as usize);

        let mut indices: Vec<u32> = (j..(params.width * params.height)).step_by(16).collect();
        indices.shuffle(&mut rng);

        for i in indices {
            let mut pixel_color = Color::new_empty();

            let px = i % params.width;
            let py = (i / params.width);

            for _ in 0..params.samples_per_pixel {
                let u = (px as f64 + rng.gen::<f64>()) / (params.width as f64 - 1.0);
                let v = ((params.height - py) as f64 + rng.gen::<f64>()) / (params.height as f64 - 1.0);

                let r = cam.ray(u, v);
                pixel_color += ray_color(&r, world, params.max_depth);
            }

            let scale = 1.0 / params.samples_per_pixel as f64;
            //pixel_color = pixel_color * scale;
            pixel_color.e[0] = (scale * pixel_color.x()).sqrt();
            pixel_color.e[1] = (scale * pixel_color.y()).sqrt();
            pixel_color.e[2] = (scale * pixel_color.z()).sqrt();

            let ir = (pixel_color.x().clamp(0.0, 0.999) * 256.0) as i32;
            let ig = (pixel_color.y().clamp(0.0, 0.999) * 256.0) as i32;
            let ib = (pixel_color.z().clamp(0.0, 0.999) * 256.0) as i32;

            pixels.push(((px, py), Rgba([ir as u8, ig as u8, ib as u8, 255])));

            if pixels.len() > 250 {
                let mut img = image.lock().unwrap();
                for &pixel in &pixels {
                    img.put_pixel(pixel.0.0, pixel.0.1, pixel.1);
                }
                pixels.clear();
            }
        }

        let mut img = image.lock().unwrap();
        for &pixel in &pixels {
            img.put_pixel(pixel.0.0, pixel.0.1, pixel.1);
        }
        pixels.clear();
    });

    let elapsed = Instant::now() - start;
    println!("Total time taken for rt: {}.{}s", elapsed.as_secs(), elapsed.as_millis() % 1000);

    image.lock().unwrap().save("output.png").expect("Error saving image!");

    params.samples_per_pixel *= 2;
    run_rt(params, world, image);
}