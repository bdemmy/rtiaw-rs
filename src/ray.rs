use crate::vec3::{Point3, Vec3};

#[derive(Copy, Clone)]
pub struct Ray {
    origin: Point3,
    dir: Vec3
}

impl Ray {
    pub fn new_empty() -> Ray {
        Ray {
            origin: Point3::new_empty(),
            dir: Vec3::new_empty()
        }
    }

    pub fn new(origin: Point3, dir: Vec3) -> Ray {
        Ray {
            origin,
            dir
        }
    }

    #[inline(always)]
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.dir * t
    }

    #[inline(always)]
    pub fn origin(&self) -> &Point3 {
        &self.origin
    }

    #[inline(always)]
    pub fn dir(&self) -> &Vec3 {
        &self.dir
    }
}