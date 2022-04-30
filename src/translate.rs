use super::aabb::AABB;
use super::hittable::{HitRecord, Hittable};
use super::ray::Ray;
use super::vec3::Vec3;
use std::sync::Arc;

pub struct Translate {
    hitable: Arc<dyn Hittable>,
    offset: Vec3,
}

impl Translate {
    pub fn new(hitable: Arc<dyn Hittable>, offset: Vec3) -> Self {
        Self { hitable, offset }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let moved_ray = Ray::new(r.origin() - self.offset, r.direction(), r.time());
        self.hitable.hit(&moved_ray, t_min, t_max).map(|mut hit| {
            hit.p += self.offset;
            hit
        })
    }

    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        self.hitable
            .bounding_box(time0, time1)
            .map(|b| AABB::new(b.min() + self.offset, b.max() + self.offset))
    }
}
