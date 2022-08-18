use std::sync::Arc;
use crate::{Color, Materials, Point3, Ray, Vec3};
use crate::aabb::AABB;
use crate::texture::Texture::SolidColor;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
    pub material: Materials
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = ray.dir().dot(outward_normal) < 0.0;
        self.normal = if self.front_face { *outward_normal } else { *outward_normal * -1.0 }
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        HitRecord {
            p: Point3::new_empty(),
            normal: Vec3::new_empty(),
            t: 0.0,
            u: 0.0,
            v: 0.0,
            front_face: false,
            material: Materials::Lambertian {albedo: SolidColor{color_value: Color::new_empty()}}
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool;
}