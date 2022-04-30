use super::aabb::AABB;
use super::hittable::{HitRecord, Hittable};
use super::material::Scatter;
use super::ray::Ray;
use super::vec3::{Point3, Vec3};
use std::f32;

use std::sync::Arc;

fn get_sphere_uv(p: Vec3) -> (f32, f32) {
    // let phi = (-p.z()).atan2(p.x()) + f32::consts::PI;
    let phi = f32::atan2(-p.z(), p.x()) + f32::consts::PI;
    let theta = (-p.y()).acos();

    let u = phi / (2.0 * f32::consts::PI);
    let v = theta / f32::consts::PI;

    // let phi = p.z().atan2(p.x());
    // let theta = p.y().acos();
    // let u = 1.0 - (phi + f32::consts::PI) / (2.0 * f32::consts::PI);
    // let v = (theta + f32::consts::FRAC_PI_2) / f32::consts::PI;
    (u, v)
}

#[derive(Clone)]
pub struct Sphere {
    center: Point3,
    radius: f32,
    mat: Arc<dyn Scatter>,
}

impl Sphere {
    pub fn new(cen: Point3, r: f32, m: Arc<dyn Scatter>) -> Self {
        Self {
            center: cen,
            radius: r,
            mat: m,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
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

        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        let (u, v) = get_sphere_uv(outward_normal);
        rec.u = u;
        rec.v = v;

        Some(rec)
    }

    fn bounding_box(&self, _time0: f32, _time1: f32) -> Option<AABB> {
        let output_box = AABB::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        );

        Some(output_box)
    }
}
