use std::mem::swap;
use crate::{Point3, Ray};

#[derive(Copy, Clone, Debug)]
pub struct AABB {
    minimum: Point3,
    maximum: Point3,
}

unsafe impl Sync for AABB {}

impl AABB {
    pub fn new(a: &Point3, b: &Point3) -> AABB {
        AABB {
            minimum: a.clone(),
            maximum: b.clone()
        }
    }

    pub fn new_empty() -> AABB {
        AABB {
            minimum: Point3::new_empty(),
            maximum: Point3::new_empty()
        }
    }

    #[inline(always)]
    pub fn max(&self) -> Point3 {
        self.maximum
    }

    #[inline(always)]
    pub fn min(&self) -> Point3 {
        self.minimum
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        for i in 0..3 {
            let inv_d = 1.0 / r.dir().e[i];
            let mut t0 = (self.min().e[i] - r.origin().e[i]) * inv_d;
            let mut t1 = (self.max().e[i] - r.origin().e[i]) * inv_d;
            if inv_d < 0.0 {
                swap(&mut t0, &mut t1);
            }
            let t_min2 = if t0 > t_min { t0 } else { t_min };
            let t_max2 = if t1 < t_max { t1 } else { t_max };
            if t_max2 <= t_min2 {
                return false;
            }
        }

        true
    }

    pub fn surrounding_box(left: &AABB, right: &AABB) -> AABB {
        let small = Point3::new(left.min().x().min(right.min().x()),
                                left.min().y().min(right.min().y()),
                                left.min().z().min(right.min().z()));
        let big = Point3::new(left.max().x().max(right.max().x()),
                                left.max().y().max(right.max().y()),
                                left.max().z().max(right.max().z()));

        AABB::new(&small, &big)
    }
}