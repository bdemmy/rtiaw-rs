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
        //let mut t1 = (self.min().e[0] - r.origin().e[0]) * r.inv_dir().e[0];
        //let mut t2 = (self.max().e[0] - r.origin().e[0]) * r.inv_dir().e[0];

        let mut t_min = t_min; // t1.min(t2);
        let mut t_max = t_max; // t1.max(t2);

        let mut t1;
        let mut t2;

        for i in 0..3 {
            t1 = (self.min().e[i] - r.origin().e[i]) * r.inv_dir().e[i];
            t2 = (self.max().e[i] - r.origin().e[i]) * r.inv_dir().e[i];

            t_min = t_min.max(t1.min(t2));
            t_max = t_max.min(t1.max(t2));
        }

        t_max > t_min.max(0.0)
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