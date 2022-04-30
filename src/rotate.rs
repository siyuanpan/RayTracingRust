use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    ray::Ray,
    vec3::Vec3,
};
use std::{f32, sync::Arc};

#[allow(dead_code)]
pub enum Axis {
    X,
    Y,
    Z,
}

fn get_axis(axis: &Axis) -> (usize, usize, usize) {
    match axis {
        Axis::X => (0, 1, 2),
        Axis::Y => (1, 2, 0),
        Axis::Z => (2, 0, 1),
    }
}

pub struct Rotate {
    axis: Axis,
    sin_theta: f32,
    cos_theta: f32,
    hitable: Arc<dyn Hittable>,
    bbox: Option<AABB>,
}

impl Rotate {
    pub fn new(axis: Axis, hitable: Arc<dyn Hittable>, angle: f32) -> Self {
        let (r_axis, a_axis, b_axis) = get_axis(&axis);
        let radians = (f32::consts::PI / 180.0) * angle;
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = hitable.bounding_box(0.0, 1.0).map(|b| {
            let mut min = Vec3::new(f32::MAX, f32::MAX, f32::MAX);
            let mut max = Vec3::new(-f32::MAX, -f32::MAX, -f32::MAX);
            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        let r = k as f32 * b.max()[r_axis] + (1 - k) as f32 * b.min()[r_axis];
                        let a = i as f32 * b.max()[a_axis] + (1 - i) as f32 * b.min()[a_axis];
                        let b = j as f32 * b.max()[b_axis] + (1 - j) as f32 * b.min()[b_axis];
                        let new_a = cos_theta * a - sin_theta * b;
                        let new_b = sin_theta * a + cos_theta * b;

                        if new_a < min[a_axis] {
                            min[a_axis] = new_a
                        }
                        if new_b < min[b_axis] {
                            min[b_axis] = new_b
                        }
                        if r < min[r_axis] {
                            min[r_axis] = r
                        }

                        if new_a > max[a_axis] {
                            max[a_axis] = new_a
                        }
                        if new_b > max[b_axis] {
                            max[b_axis] = new_b
                        }
                        if r > max[r_axis] {
                            max[r_axis] = r
                        }
                    }
                }
            }

            AABB::new(min, max)
        });
        Self {
            axis,
            sin_theta,
            cos_theta,
            hitable,
            bbox,
        }
    }
}

impl Hittable for Rotate {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let (_, a_axis, b_axis) = get_axis(&self.axis);
        let mut origin = r.origin();
        let mut direction = r.direction();
        origin[a_axis] = self.cos_theta * r.origin()[a_axis] + self.sin_theta * r.origin()[b_axis];
        origin[b_axis] = -self.sin_theta * r.origin()[a_axis] + self.cos_theta * r.origin()[b_axis];
        direction[a_axis] =
            self.cos_theta * r.direction()[a_axis] + self.sin_theta * r.direction()[b_axis];
        direction[b_axis] =
            -self.sin_theta * r.direction()[a_axis] + self.cos_theta * r.direction()[b_axis];
        let rotated_ray = Ray::new(origin, direction, r.time());
        self.hitable.hit(&rotated_ray, t_min, t_max).map(|mut hit| {
            let mut p = hit.p;
            let mut normal = hit.normal;
            p[a_axis] = self.cos_theta * hit.p[a_axis] - self.sin_theta * hit.p[b_axis];
            p[b_axis] = self.sin_theta * hit.p[a_axis] + self.cos_theta * hit.p[b_axis];
            normal[a_axis] =
                self.cos_theta * hit.normal[a_axis] - self.sin_theta * hit.normal[b_axis];
            normal[b_axis] =
                self.sin_theta * hit.normal[a_axis] + self.cos_theta * hit.normal[b_axis];
            hit.p = p;
            hit.set_face_normal(&rotated_ray, normal);
            hit
        })
    }

    fn bounding_box(&self, _time0: f32, _time1: f32) -> Option<AABB> {
        self.bbox.clone()
    }
}
