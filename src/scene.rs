use std::sync::Arc;
use image::io::Reader;
use image::RgbaImage;
use noise::Turbulence;
use rand::Rng;
use crate::{Camera, Color, HitList, Hittable, Materials, Point3, Sphere, Vec3};
use crate::bvh::BvhNode;
use crate::texture::Texture;
use crate::texture::Texture::{Checker, SolidColor};
use crate::triangle::Triangle;

use std::fs::File;
use std::io::BufReader;
use itertools::Itertools;
use tobj::{load_obj, LoadOptions};

pub struct Scene {
    pub hit_list: HitList,
    pub camera: Camera,
    pub sky_color: Color
}

pub fn random_scene() -> Scene {
    let mut world = HitList::new();

    let mat_ground: Materials = Materials::Lambertian {
        albedo: Checker {
            texture_odd: Arc::from(Texture::SolidColor {
                color_value: Color::new(0.2, 0.3, 0.1)
            }),
            texture_even: Arc::from(Texture::SolidColor {
                color_value: Color::new(0.9, 0.9, 0.9)
            }),
        }
    };

    world.add(Arc::new(Sphere {
        center: Point3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: mat_ground.clone(),
    }));

    let mut rng = rand::thread_rng();

    for a in -33..33 {
        for b in -33..33 {
            let mat_rand = rng.gen::<f64>();

            let origin = Point3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>()
            );

            if (origin - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let mat = if mat_rand < 0.8 {
                    let albedo = Color::random() * Color::random();
                    Materials::Lambertian { albedo: SolidColor {color_value: albedo.clone()}}
                }
                else if mat_rand < 0.95 {
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    Materials::Metal { albedo, fuzz }
                }
                else {
                    Materials::DiElectric {
                        ir: 1.5
                    }
                };

                world.add(Arc::new(Sphere {
                    center: origin,
                    radius: 0.2,
                    material: mat,
                }));
            }
        }
    }

    let mat_center = Materials::Lambertian {
        albedo: Texture::Image {
            image: Arc::from(Reader::open("earthmap.jpg").unwrap().decode().unwrap().to_rgba8())
        }
        /*albedo: Texture::Perlin {
            turbulence: Turbulence::new(noise::Perlin::new())
        }*/
    };

    let mat_left = Materials::DiElectric {
        ir: 1.5

    };
    let mat_right = Materials::Metal {
        albedo: Color::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    };

    let mat_left = Materials::DiffuseLight {
        tex: Texture::SolidColor {color_value: Color::new(4,4,4)}
    };

    {
        let mut options = LoadOptions::default();
        options.triangulate = true;
        let (model, _) = load_obj("xyzrgb_dragon.obj", &options)
            .expect("Error loading OBJ file.");

        let factor = *model[0].mesh.positions.iter().reduce(|a, b| {
            if a > b { a } else { b }
        }).unwrap_or(&1f32) as f64;
        let factor = 0.5;

        let mut mdl_bvh = HitList::new();

        for mut chunk in &model[0].mesh.indices.iter().chunks(3) {
            let i0 = *chunk.next().unwrap() as usize;
            let i1 = *chunk.next().unwrap() as usize;
            let i2 = *chunk.next().unwrap() as usize;

            let v0 = Point3::new(
                model[0].mesh.positions[i0 * 3],
                model[0].mesh.positions[i0 * 3 + 1]* -1f32,
                model[0].mesh.positions[i0 * 3 + 2]
            ) / factor * -1f64;

            let v1 = Point3::new(
                model[0].mesh.positions[i1 * 3],
                model[0].mesh.positions[i1 * 3 + 1]* -1f32,
                model[0].mesh.positions[i1 * 3 + 2]
            ) / factor * -1f64;

            let v2 = Point3::new(
                model[0].mesh.positions[i2 * 3],
                model[0].mesh.positions[i2 * 3 + 1]* -1f32,
                model[0].mesh.positions[i2 * 3 + 2]
            ) / factor * -1f64;

            mdl_bvh.add(Arc::new(Triangle::new_with(
                v0,
                v1,
                v2,
                Materials::Lambertian {
                    albedo: Texture::SolidColor {
                        color_value: Color::new(0.8, 0.8, 0.8)
                    }
                }
            )));
        }

        println!("{} {}", factor, model[0].mesh.indices.len());

        world.add(Arc::new(BvhNode::new_from_hitlist(&mdl_bvh, 0f64, 1f64)));
    }

    /*world.add(Arc::new(Sphere {
        center: Point3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: mat_left.clone(),
    }));*/

    world.add(Arc::new(Sphere {
        center: Point3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: mat_center.clone(),
    }));

    world.add(Arc::new(Sphere {
        center: Point3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: mat_left.clone(),
    }));

    world = HitList::new_with(Arc::new(BvhNode::new_from_hitlist(&world, 0.0, 1.0)));

    let cam = Camera::new(
        Point3::new(0, 3, -5),
        Point3::new(0, 0, 0),
        Vec3::new(0, 1, 0),
        70.0,
        16.0 / 9.0,
        0.00001,
        10.0);

    Scene {
        hit_list: world,
        camera: cam,
        sky_color: Color::new(0.7,0.8,1.0)
    }
}

pub fn tri_test() -> Scene {
    let mut world = HitList::new();

    let mat_ground: Materials = Materials::Lambertian {
        albedo: Texture::SolidColor {
            color_value: Color::new(0.8, 0.8, 0.8)
        }
    };

    world.add(Arc::new(Sphere {
        center: Point3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: mat_ground.clone(),
    }));

    world.add(Arc::new(Triangle::new_with(
        Point3::new(-5, 0, 0),
        Point3::new(-3, 0, 2),
        Point3::new(-4, 2, 1),
        Materials::DiffuseLight {
            tex: Texture::SolidColor {color_value: Color::new(4,4,4)}
        }
    )));

    // Load sample mesh
    let mut options = LoadOptions::default();
    options.triangulate = true;
    let (model, _) = load_obj("pumpkin_tall_10k.obj", &options)
        .expect("Error loading OBJ file.");

    for mut chunk in &model[0].mesh.indices.iter().chunks(3) {
        let i0 = *chunk.next().unwrap() as usize;
        let i1 = *chunk.next().unwrap() as usize;
        let i2 = *chunk.next().unwrap() as usize;

        let v0 = Point3::new(
            model[0].mesh.positions[i0 * 3],
            model[0].mesh.positions[i0 * 3 + 1],
            model[0].mesh.positions[i0 * 3 + 2]
        ) * 0.025;

        let v1 = Point3::new(
            model[0].mesh.positions[i1 * 3],
            model[0].mesh.positions[i1 * 3 + 1],
            model[0].mesh.positions[i1 * 3 + 2]
        ) * 0.025;

        let v2 = Point3::new(
            model[0].mesh.positions[i2 * 3],
            model[0].mesh.positions[i2 * 3 + 1],
            model[0].mesh.positions[i2 * 3 + 2]
        ) * 0.025;

        world.add(Arc::new(Triangle::new_with(
            v0,
            v1,
            v2,
            Materials::Lambertian {
                albedo: Texture::SolidColor {
                    color_value: Color::new(0.8, 0.8, 0.8)
                }
            }
        )));
    }

    println!("{}", model[0].mesh.indices.len());

    world = HitList::new_with(Arc::new(BvhNode::new_from_hitlist(&world, 0.0, 1.0)));

    let cam = Camera::new(
        Point3::new(0, 3, -5),
        Point3::new(0, 0, 0),
        Vec3::new(0, 1, 0),
        70.0,
        16.0 / 9.0,
        0.00001,
        10.0);

    Scene {
        hit_list: world,
        camera: cam,
        sky_color: Color::new(0.7,0.8,1)
    }
}