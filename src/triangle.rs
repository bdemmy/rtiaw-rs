use std::borrow::Borrow;
use crate::{HitRecord, Hittable, Materials, Point3, Ray, Vec3};
use crate::aabb::AABB;

pub struct Triangle {
    v1: Point3,
    v2: Point3,
    v3: Point3,
    n: Vec3,
    material: Materials
}

impl Triangle {
    pub fn new_with(v1: Point3, v2: Point3, v3: Point3, material: Materials) -> Triangle {
        Triangle {
            v1, v2, v3, material,
            n: (v2 - v1).cross(&(v3 - v1)).normalized()
        }
    }
}

impl Hittable for Triangle {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        if ray.dir().dot(&self.n) > 0.0 {
            return false;
        }

        let area2 = self.n.length();

        let n_dot_ray_dir = self.n.dot(&ray.dir());
        if n_dot_ray_dir.abs() < f64::EPSILON {
            return false;
        }

        let d = (-self.n).dot(&self.v1);

        let t = -(self.n.dot(&ray.origin()) + d) / n_dot_ray_dir;
        if t < t_min || t > t_max {
            return false;
        }

        let p = *ray.origin() + *ray.dir() * t;

        let edge0 = self.v2 - self.v1;
        let vp0 = p - self.v1;
        let c = edge0.cross(&vp0);
        if self.n.dot(&c) < 0.0 {
            return false;
        }

        let edge1 = self.v3 - self.v2;
        let vp1 = p - self.v2;
        let c = edge1.cross(&vp1);
        if self.n.dot(&c) < 0.0 {
            return false;
        }

        let edge2 = self.v1 - self.v3;
        let vp2 = p - self.v3;
        let c = edge2.cross(&vp2);
        if self.n.dot(&c) < 0.0 {
            return false;
        }

        rec.t = t;
        rec.set_face_normal(ray, &self.n);
        rec.p = ray.at(rec.t);
        rec.material = self.material.clone();
        true
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        let x_min = self.v1.x().min(self.v2.x().min(self.v3.x()));
        let x_max = self.v1.x().max(self.v2.x().max(self.v3.x()));

        let y_min = self.v1.y().min(self.v2.y().min(self.v3.y()));
        let y_max = self.v1.y().max(self.v2.y().max(self.v3.y()));

        let z_min = self.v1.z().min(self.v2.z().min(self.v3.z()));
        let z_max = self.v1.z().max(self.v2.z().max(self.v3.z()));

        *output_box = AABB::new(
            (Vec3::new(x_min, y_min, z_min)).borrow(),
            (Vec3::new(x_max, y_max, z_max)).borrow()
        );

        true
    }
}