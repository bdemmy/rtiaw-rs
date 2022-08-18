use std::borrow::Borrow;
use std::f64::consts::PI;
use crate::hittable::{HitRecord, Hittable};
use crate::{Materials, Point3, Vec3};
use crate::aabb::AABB;
use crate::ray::Ray;

#[derive(Clone)]
pub struct Sphere {
    pub(crate) center: Point3,
    pub(crate) radius: f64,
    pub material: Materials
}

impl Sphere {
    fn get_sphere_uv(&self, p: &Point3, u: &mut f64, v: &mut f64) {
        let theta = (-p.y()).acos();
        let phi = (-p.z().atan2(p.x())) + PI;

        *u = phi / (2.0 * PI);
        *v = theta / PI;
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = *ray.origin() - self.center;
        let a = ray.dir().length_squared();
        let half_b = oc.dot(ray.dir());
        let c = oc.length_squared() - self.radius * self.radius;
        let disc = half_b * half_b - a * c;

        if disc < 0.0 {
            return false;
        }

        let sqrt_d = disc.sqrt();
        let mut root = (-half_b - sqrt_d) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrt_d) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = ray.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(ray, &outward_normal);
        self.get_sphere_uv(&outward_normal, &mut rec.u, &mut rec.v);
        rec.material = self.material.clone();

        true
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(
            (self.center - Vec3::new(self.radius, self.radius, self.radius)).borrow(),
            (self.center + Vec3::new(self.radius, self.radius, self.radius)).borrow()
        );

        true
    }
}