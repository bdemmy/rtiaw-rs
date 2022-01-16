use std::cmp::Ordering;
use std::sync::Arc;
use rand::Rng;
use crate::aabb::AABB;
use crate::{HitList, Hittable, Sphere};
use crate::hittable::HitRecord;
use crate::ray::Ray;

type ChildNode = Option<Arc<dyn Hittable>>;

#[derive(Clone)]
pub struct BvhNode {
    left: ChildNode,
    right: ChildNode,
    bounding_box: AABB
}

impl BvhNode {
    pub fn new_empty() -> BvhNode {
        BvhNode {
            left: None,
            right: None,
            bounding_box: AABB::new_empty()
        }
    }

    pub fn new_from_hitlist(list: &HitList, time0: f64, time1: f64) -> BvhNode {
        BvhNode::new(&list.objects, 0, list.objects.len(), time0, time1)
    }

    pub fn new(src_objects: &Vec<Arc<dyn Hittable>>, start: usize, end: usize, time0: f64, time1: f64) -> BvhNode {
        let mut objects = src_objects.clone();

        let axis = rand::thread_rng().gen_range((0usize..2usize));
        let comparator = |a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>| -> std::cmp::Ordering {
            let box_compare = |a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis: usize| -> Ordering {
                let mut box_a = AABB::new_empty();
                let mut box_b = AABB::new_empty();

                if !a.bounding_box(0.0, 0.0, &mut box_a) ||
                    !b.bounding_box(0.0, 0.0, &mut box_b) {
                    eprintln!("No bounding box in BvhNode constructor");
                }

                return if box_a.min().e[axis] < box_b.min().e[axis] {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            };

            box_compare(a, b, axis)
        };

        let mut new = BvhNode::new_empty();

        let object_span = end - start;

        println!("{}", object_span);

        if object_span == 1 {
            new.left = Some(objects[start].clone());
            new.right = new.left.clone();
        }
        else if object_span == 2 {
            if comparator(&objects[start], &objects[start + 1]) == Ordering::Less {
                new.left = Some(objects[start].clone());
                new.right = Some(objects[start + 1].clone());
            }
            else {
                new.right = Some(objects[start].clone());
                new.left = Some(objects[start + 1].clone());
            }
        }
        else {
            objects[start..end].sort_by(comparator);
            let mid = start + (object_span / 2);
            new.left = Some(Arc::new(BvhNode::new(&objects, start, mid, time0, time1)));
            new.right = Some(Arc::new(BvhNode::new(&objects, mid, end, time0, time1)));
        }

        let mut box_left = AABB::new_empty();
        let mut box_right = AABB::new_empty();

        let left = match &new.left {
            Some(left) => {
                left.bounding_box(time0, time1, &mut box_left)
            },
            None => false
        };

        let right = match &new.right {
            Some(right) => {
                right.bounding_box(time0, time1, &mut box_right)
            },
            None => false
        };

        if !left || !right {
            eprintln!("No bounding box in BvhNode constructor.");
        }

        new.bounding_box = AABB::surrounding_box(&box_left, &box_right);

        new
    }
}

impl Hittable for BvhNode {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        if !self.bounding_box.hit(ray, t_min, t_max) {
            return false;
        }

        let left_hit = self.left.as_ref().unwrap().hit(ray, t_min, t_max, rec);
        let right_hit = if left_hit {
            self.right.as_ref().unwrap().hit(ray, t_min, rec.t, rec)
        } else {
            self.right.as_ref().unwrap().hit(ray, t_min, t_max, rec)
        };

        left_hit || right_hit
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        *output_box = self.bounding_box;
        true
    }
}