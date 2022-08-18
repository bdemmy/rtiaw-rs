use std::sync::{Arc, Mutex};
use std::time::Instant;
use image::{ColorType, Rgba, RgbaImage};
use rand::{Rng};
use rand::prelude::SliceRandom;
use rayon::prelude::*;
use crate::{Camera, Color, HitList, HitRecord, Hittable, Point3, Ray, Vec3};
use crate::glutin::event::VirtualKeyCode::P;
use crate::scene::Scene;

pub struct RTParams {
    aspect_ratio: f64,
    width: u32,
    height: u32,
    samples_per_pixel: u32,
    max_depth: i32,
}

impl RTParams {
    pub fn new(aspect_ratio: f64, image_width: u32, samples_per_pixel: u32, max_depth: i32) -> RTParams {
        RTParams {
            height: (image_width as f64 / aspect_ratio) as u32,
            width: image_width,
            aspect_ratio,
            samples_per_pixel,
            max_depth,
        }
    }
}

fn ray_color(ray: &Ray, background: &Color, world: &dyn Hittable, depth: i32) -> Color {
    let mut rec = HitRecord::default();

    if depth <= 0 {
        return Color::new_empty();
    }

    if !world.hit(ray, 0.001, f64::INFINITY, &mut rec) {
        return *background;
    }

    let mut scattered = Ray::new_empty();
    let mut attenuation = Color::new_empty();
    let emitted = rec.material.emitted(rec.u, rec.v, &rec.p);

    if !rec.material.scatter(ray, &rec, &mut attenuation, &mut scattered) {
        return emitted;
    }

    return emitted + attenuation * ray_color(&scattered, background, world, depth - 1);
    /*if !rec.material.scatter(&ray, &rec, &mut attenuation, &mut scattered) {
        if attenuation.length() < 0.1 {
            return attenuation;
        }
        return attenuation * ray_color(&scattered, world, depth - 1);
    }
    return Color::new_empty();

    let unit_direction = ray.dir().normalized();
    let t = 0.5 * (unit_direction.y() + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t*/
}

fn rgba_to_color(rgba: &Rgba<u8>) -> Color {
    Color::new(rgba.0[0], rgba.0[1], rgba.0[2])
}

fn trace_pixel(px: u32, py: u32, params: &RTParams, scene: &Scene) -> Rgba<u8> {
    let mut rng = rand::thread_rng();

    let mut pixel_color = Color::new_empty();

    for _ in 0..params.samples_per_pixel {
        let u = (px as f64 + rng.gen::<f64>()) / (params.width as f64 - 1.0);
        let v = ((params.height - py) as f64 + rng.gen::<f64>()) / (params.height as f64 - 1.0);

        let r = scene.camera.ray(u, v);
        pixel_color += ray_color(&r, &scene.sky_color, &scene.hit_list, params.max_depth);
    }

    let scale = 1.0 / params.samples_per_pixel as f64;
    //pixel_color = pixel_color * scale;
    pixel_color.e[0] = (scale * pixel_color.x()).sqrt();
    pixel_color.e[1] = (scale * pixel_color.y()).sqrt();
    pixel_color.e[2] = (scale * pixel_color.z()).sqrt();

    let ir = (pixel_color.x().clamp(0.0, 0.999) * 256.0) as u8;
    let ig = (pixel_color.y().clamp(0.0, 0.999) * 256.0) as u8;
    let ib = (pixel_color.z().clamp(0.0, 0.999) * 256.0) as u8;

    return Rgba([ir, ig, ib, 255]);
}

fn oidn(image: &Arc<Mutex<RgbaImage>>) {
    let input = image.lock().unwrap().clone();

    // OIDN works on float images only, so convert this to a floating point image
    let mut input_img = vec![0.0f32; (3 * input.width() * input.height()) as usize];
    for y in 0..input.height() {
        for x in 0..input.width() {
            let p = input.get_pixel(x, y);
            for c in 0..3 {
                input_img[3 * ((y * input.width() + x) as usize) + c] = p[c] as f32 / 255.0;
            }
        }
    }

    println!("Image dims {}x{}", input.width(), input.height());

    let mut filter_output = vec![0.0f32; input_img.len()];

    let device = oidn::Device::new();
    let mut filter = oidn::RayTracing::new(&device);
    filter
        .srgb(true)
        .image_dimensions(input.width() as usize, input.height() as usize);
    filter
        .filter(&input_img[..], &mut filter_output[..])
        .expect("Invalid input image dimensions?");

    if let Err(e) = device.get_error() {
        println!("Error denoising image: {}", e.1);
    }

    let mut original_image = image.lock().unwrap();

    for (idx, chunk) in filter_output.chunks(3).enumerate() {
        let px = (idx % input.width() as usize) as u32;
        let py = (idx / input.width() as usize) as u32;

        let pr = (chunk[0] * 255.0).max(0.0).min(255.0) as u8;
        let pg = (chunk[1] * 255.0).max(0.0).min(255.0) as u8;
        let pb = (chunk[2] * 255.0).max(0.0).min(255.0) as u8;
        let pa = 255 as u8;

        original_image.put_pixel(px, py, Rgba([pr, pg, pb, pa]));
    }

    let mut output_img = vec![0u8; filter_output.len()];
    for i in 0..filter_output.len() {
        let p = filter_output[i] * 255.0;
        if p < 0.0 {
            output_img[i] = 0;
        } else if p > 255.0 {
            output_img[i] = 255;
        } else {
            output_img[i] = p as u8;
        }
    }

    image::save_buffer(
        "output_oidn.png",
        &output_img[..],
        input.width(),
        input.height(),
        ColorType::Rgb8,
    ).expect("Failed to save output image");
}

pub fn run_rt(mut params: RTParams, scene: &Scene, image: Arc<Mutex<RgbaImage>>) {
    let start = Instant::now();

    // Helper functions for raytracing single pixels
    //  as well as flushing to the output image when needed
    let flush_pixels = |imge: &Arc<Mutex<RgbaImage>>, mut pixels: &mut Vec<((u32, u32), Rgba<u8>)>, force: bool| {
        if pixels.len() > 250 || force {
            let mut img = image.lock().unwrap();
            pixels.iter().for_each(|pixel|{
                img.put_pixel(pixel.0.0, pixel.0.1, pixel.1);
            });
            pixels.clear();
        }
    };

    // Run the raytrace on half of the pixels
    (0..16).into_par_iter().for_each(|j| {
        let mut rng = rand::thread_rng();
        let mut pixels: Vec<((u32, u32), Rgba<u8>)> = Vec::new();
        pixels.reserve((params.width * params.height) as usize);

        (j..(params.width * params.height)).step_by(16)
            .filter(|x| {
                /*let row = x / params.width;
                return if row % 2 == 0 {
                    x % 2 == 0
                } else {
                    x % 2 == 1
                }*/
                true
            }).for_each(|i| {
            let px = i % params.width;
            let py = i / params.width;

            pixels.push(((px, py), trace_pixel(px, py, &params, &scene)));
            flush_pixels(&image, &mut pixels, false);
        });

        flush_pixels(&image, &mut pixels, true);
    });

    // Fill the other half of the pixels via interpolation
    // Only raytracing if interpolation would be too extreme
    /*(0..16).into_par_iter().for_each(|j| {
        let local_copy = image.lock().unwrap().clone();
        let mut rng = rand::thread_rng();
        let mut pixels: Vec<((u32, u32), Rgba<u8>)> = Vec::new();
        pixels.reserve((params.width * params.height) as usize);

        (j..(params.width * params.height))
            .step_by(16)
            .filter(|x| {
                let row = x / params.width;
                return if row % 2 == 0 {
                    x % 2 == 1
                } else {
                    x % 2 == 0
                }
            }).for_each(|i| {
            let px = i % params.width;
            let py = i / params.width;

            let avg = Color::new(255.0,255.0,255.0);
            pixels.push(((px, py), Rgba([avg.x() as u8, avg.y() as u8, avg.z() as u8, 255])));

            if px == 0 || px == params.width - 1 || py == 0 || py == params.height - 1 {
                pixels.push(((px, py), rt_pixel(px, py)));
            } else {
                let left = rgba_to_color(local_copy.get_pixel(px - 1, py));
                let right = rgba_to_color(local_copy.get_pixel(px + 1, py));
                let top = rgba_to_color(local_copy.get_pixel(px + 1, py));
                let bottom = rgba_to_color(local_copy.get_pixel(px + 1, py));
                let avg = (left + right + top + bottom) / 4.0;

                if (left - avg).length() > 25.0
                    || (right - avg).length() > 25.0
                    || (top - avg).length() > 25.0
                    || (bottom - avg).length() > 25.0 {
                    pixels.push(((px, py), rt_pixel(px, py)));
                } else {
                    pixels.push(((px, py), Rgba([avg.x() as u8, avg.y() as u8, avg.z() as u8, 255])));
                }
            }

            flush_pixels(&image, &mut pixels, false);
        });

        flush_pixels(&image, &mut pixels, true);
    });*/

    let elapsed = Instant::now() - start;
    println!("Total time taken for rt: {}.{}s", elapsed.as_secs(), elapsed.as_millis() % 1000);

    image.lock().unwrap().save("output.png").expect("Error saving image!");
    //oidn(&image);

    // Run rt again with double the sample count
    params.samples_per_pixel *= 2;
    run_rt(params, scene, image);
}