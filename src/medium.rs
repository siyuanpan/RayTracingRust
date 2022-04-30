use rand::Rng;

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    material::{Isotropic, Scatter},
    ray::Ray,
    texture::Texture,
    vec3::Vec3,
};
use std::sync::Arc;

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    neg_inv_density: f32,
    phase_function: Arc<dyn Scatter>,
}

impl ConstantMedium {
    pub fn new(b: Arc<dyn Hittable>, d: f32, a: Arc<dyn Texture>) -> Self {
        Self {
            boundary: b,
            neg_inv_density: -1.0 / d,
            phase_function: Arc::new(Isotropic::new(a)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut rng = rand::thread_rng();
        if let Some(mut hit1) = self.boundary.hit(&r, -f32::MAX, f32::MAX) {
            if let Some(mut hit2) = self.boundary.hit(&r, hit1.t + 0.0001, f32::MAX) {
                if hit1.t < t_min {
                    hit1.t = t_min
                }
                if hit2.t > t_max {
                    hit2.t = t_max
                }
                if hit1.t < hit2.t {
                    let distance_inside_boundary = (hit2.t - hit1.t) * r.direction().length();
                    let hit_distance = self.neg_inv_density * rng.gen::<f32>().ln();
                    if hit_distance < distance_inside_boundary {
                        let t = hit1.t + hit_distance / r.direction().length();
                        return Some(HitRecord {
                            t,
                            u: 0.0,
                            v: 0.0,
                            p: r.at(t),
                            normal: Vec3::new(1.0, 0.0, 0.0),
                            front_face: true,
                            mat: self.phase_function.clone(),
                        });
                    }
                }
            }
        }
        None
    }

    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        self.boundary.bounding_box(time0, time1)
    }
}
