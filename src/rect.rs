use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    material::Scatter,
    ray::Ray,
    vec3::Vec3,
};
use std::sync::Arc;

pub enum Plane {
    YZ,
    ZX,
    XY,
}

pub struct Rect {
    plane: Plane,
    a0: f32,
    a1: f32,
    b0: f32,
    b1: f32,
    k: f32,
    mat: Arc<dyn Scatter>,
}

impl Rect {
    pub fn new(
        plane: Plane,
        a0: f32,
        a1: f32,
        b0: f32,
        b1: f32,
        k: f32,
        material: Arc<dyn Scatter>,
    ) -> Self {
        Self {
            plane,
            a0,
            a1,
            b0,
            b1,
            k,
            mat: material,
        }
    }
}

impl Hittable for Rect {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let (k_axis, a_axis, b_axis) = match &self.plane {
            Plane::YZ => (0, 1, 2),
            Plane::ZX => (1, 2, 0),
            Plane::XY => (2, 0, 1),
        };

        let t = (self.k - r.origin()[k_axis]) / r.direction()[k_axis];
        if t < t_min || t > t_max {
            None
        } else {
            let a = r.origin()[a_axis] + t * r.direction()[a_axis];
            let b = r.origin()[b_axis] + t * r.direction()[b_axis];
            if a < self.a0 || a > self.a1 || b < self.b0 || b > self.b1 {
                None
            } else {
                let u = (a - self.a0) / (self.a1 - self.a0);
                let v = (b - self.b0) / (self.b1 - self.b0);
                let p = r.at(t);
                let mut normal = Vec3::new(0.0, 0.0, 0.0);
                normal[k_axis] = 1.0;
                let mut rec = HitRecord {
                    t,
                    p,
                    normal,
                    u,
                    v,
                    mat: self.mat.clone(),
                    front_face: false,
                };
                rec.set_face_normal(r, normal);

                Some(rec)
            }
        }
    }

    fn bounding_box(&self, _time0: f32, _time1: f32) -> Option<AABB> {
        let (k_axis, a_axis, b_axis) = match &self.plane {
            Plane::YZ => (0, 1, 2),
            Plane::ZX => (1, 2, 0),
            Plane::XY => (2, 0, 1),
        };
        let mut min = Vec3::new(0.0, 0.0, 0.0);
        let mut max = Vec3::new(0.0, 0.0, 0.0);
        min[k_axis] = self.k - 0.0001;
        min[a_axis] = self.a0;
        min[b_axis] = self.b0;
        max[k_axis] = self.k + 0.0001;
        max[a_axis] = self.a1;
        max[b_axis] = self.b1;

        Some(AABB::new(min, max))
    }
}
