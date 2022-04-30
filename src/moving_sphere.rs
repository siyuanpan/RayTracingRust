use super::aabb::AABB;
use super::hittable::{HitRecord, Hittable};
use super::material::Scatter;
use super::ray::Ray;
use super::vec3::{Point3, Vec3};

use std::f32;
use std::sync::Arc;

fn get_sphere_uv(p: Vec3) -> (f32, f32) {
    let phi = (-p.z()).atan2(p.x()) + f32::consts::PI;
    let theta = (-p.y()).acos();

    let u = phi / (2.0 * f32::consts::PI);
    let v = theta / f32::consts::PI;
    (u, v)
}

pub struct MovingSphere {
    center0: Point3,
    center1: Point3,
    time0: f32,
    time1: f32,
    radius: f32,
    mat: Arc<dyn Scatter>,
}

impl MovingSphere {
    pub fn new(
        cen0: Point3,
        cen1: Point3,
        time0: f32,
        time1: f32,
        r: f32,
        m: Arc<dyn Scatter>,
    ) -> Self {
        Self {
            center0: cen0,
            center1: cen1,
            time0,
            time1,
            radius: r,
            mat: m,
        }
    }

    pub fn center(&self, time: f32) -> Point3 {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin() - self.center(r.time());
        let a = r.direction().dot(r.direction());
        let half_b = oc.dot(r.direction());
        let c = oc.dot(oc) - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let p = r.at(root);
        let mut rec = HitRecord {
            t: root,
            p: p,
            normal: Vec3::new(0.0, 0.0, 0.0),
            u: 0.0,
            v: 0.0,
            mat: self.mat.clone(),
            front_face: false,
        };

        let outward_normal = (rec.p - self.center(r.time())) / self.radius;
        rec.set_face_normal(r, outward_normal);
        let (u, v) = get_sphere_uv(outward_normal);
        rec.u = u;
        rec.v = v;

        Some(rec)
    }

    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        let box0 = AABB::new(
            self.center(time0) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(time0) + Vec3::new(self.radius, self.radius, self.radius),
        );

        let box1 = AABB::new(
            self.center(time1) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(time1) + Vec3::new(self.radius, self.radius, self.radius),
        );

        let output_box = AABB::surrounding_box(&box0, &box1);

        Some(output_box)
    }
}
