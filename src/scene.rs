use std::sync::Arc;
use rand::Rng;
use crate::{Color, HitList, Materials, Point3, Sphere};
use crate::bvh::BvhNode;

pub fn random_scene() -> HitList {
    let mut world = HitList::new();

    let mat_ground: Materials = Materials::Lambertian {
        albedo: Color::new(0.5, 0.5, 0.5)
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
                    Materials::Lambertian { albedo }
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
        albedo: Color::new(0.4, 0.2, 0.1)
    };

    let mat_left = Materials::DiElectric {
        ir: 1.5
    };

    let mat_right = Materials::Metal {
        albedo: Color::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    };

    world.add(Arc::new(Sphere {
        center: Point3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: mat_left.clone(),
    }));

    world.add(Arc::new(Sphere {
        center: Point3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: mat_center.clone(),
    }));

    world.add(Arc::new(Sphere {
        center: Point3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: mat_right.clone(),
    }));

    world = HitList::new_with(Arc::new(BvhNode::new_from_hitlist(&world, 0.0, 1.0)));

    world
}