use crate::{Point3, Ray, Vec3};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64
}

impl Camera {
    pub fn new(look_from: Point3, look_at: Point3, vup: Vec3, fov: f64, aspect_ratio: f64, aperture: f64, focus_dist: f64) -> Camera {
        let theta = fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).normalized();
        let u = vup.cross(&w).normalized();
        let v = w.cross(&u);

        let mut cam = Camera {
            origin: look_from,
            horizontal: u * viewport_width * focus_dist,
            vertical: v * viewport_height * focus_dist,
            lower_left_corner: Point3::new_empty(),
            u, v, w,
            lens_radius: aperture / 2.0
        };

        cam.lower_left_corner = cam.origin - cam.horizontal / 2.0 - cam.vertical / 2.0 - w * focus_dist;
        cam
    }

    pub fn ray(&self, s: f64, t: f64) -> Ray {
        let rd = Vec3::random_in_unit_disk() * self.lens_radius;
        let offset =  self.u * rd.x() + self.v * rd.y();
        Ray::new(self.origin + offset, self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset)
    }
}