use super::ray::Ray;
use super::vec3::{Point3, Vec3};

#[derive(Clone, Copy)]
pub struct AABB {
    minimum: Point3,
    maximum: Point3,
}

impl AABB {
    pub fn new(a: Point3, b: Point3) -> Self {
        Self {
            minimum: a,
            maximum: b,
        }
    }

    pub fn min(&self) -> Point3 {
        self.minimum
    }

    pub fn max(&self) -> Point3 {
        self.maximum
    }

    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> bool {
        let mut tmin = t_min;
        let mut tmax = t_max;
        for a in 0..3 {
            let inv_d = 1.0 / r.direction()[a];
            let mut t0 = (self.min()[a] - r.origin()[a]) * inv_d;
            let mut t1 = (self.max()[a] - r.origin()[a]) * inv_d;
            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            tmin = if t0 > tmin { t0 } else { tmin };
            tmax = if t1 < tmax { t1 } else { tmax };
            if tmax <= tmin {
                return false;
            }
        }

        true
    }

    pub fn surrounding_box(box0: &Self, box1: &Self) -> Self {
        let min = Vec3::new(
            f32::min(box0.min().x(), box1.min().x()),
            f32::min(box0.min().y(), box1.min().y()),
            f32::min(box0.min().z(), box1.min().z()),
        );

        let max = Vec3::new(
            f32::max(box0.max().x(), box1.max().x()),
            f32::max(box0.max().y(), box1.max().y()),
            f32::max(box0.max().z(), box1.max().z()),
        );

        Self::new(min, max)
    }
}
