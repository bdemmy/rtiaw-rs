use std::sync::Arc;
use crate::hittable::{HitRecord, Hittable};
use crate::{Ray};
use crate::aabb::AABB;

pub struct HitList {
    pub objects: Vec<Arc<dyn Hittable>>
}

unsafe impl Sync for HitList {}

impl Hittable for HitList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t.clone();
                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        if self.objects.is_empty() {
            return false;
        }

        let mut temp_box = AABB::new_empty();
        let mut first_box = true;

        for object in &self.objects {
            if !object.bounding_box(time0, time1, &mut temp_box) {
                return false;
            }

            *output_box = if first_box { temp_box.clone() } else {AABB::surrounding_box(output_box, &temp_box)};
            first_box = false;
        }

        true
    }
}

impl HitList {
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn new_with(object: Arc<dyn Hittable>) -> Self {
        HitList {
            objects: vec![object]
        }
    }

    pub fn new() -> Self {
        HitList {
            objects: Vec::new()
        }
    }
}
