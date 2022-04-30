use super::aabb::AABB;
use super::hittable::{HitRecord, Hittable};
use super::ray::Ray;

pub type World = Vec<Box<dyn Hittable>>;

pub type HitableList = World;

impl Hittable for World {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut tmp_rec = None;
        let mut closest_so_far = t_max;

        for object in self {
            if let Some(rec) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = rec.t;
                tmp_rec = Some(rec);
            }
        }

        tmp_rec
    }

    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        match self.first() {
            Some(first) => match first.bounding_box(time0, time1) {
                Some(bbox) => self.iter().skip(1).try_fold(bbox, |acc, hitable| {
                    match hitable.bounding_box(time0, time1) {
                        Some(bbox) => Some(AABB::surrounding_box(&acc, &bbox)),
                        _ => None,
                    }
                }),
                _ => None,
            },
            _ => None,
        }
    }
}
