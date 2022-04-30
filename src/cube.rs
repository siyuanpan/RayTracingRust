use crate::hittable::{HitRecord, Hittable};
use crate::material::Scatter;

use super::aabb::AABB;
use super::ray::Ray;
use super::rect::{Plane, Rect};
use super::vec3::Vec3;
use super::world::HitableList;
use std::sync::Arc;

pub struct Cube {
    box_min: Vec3,
    box_max: Vec3,
    sides: HitableList,
}

impl Cube {
    pub fn new(p0: Vec3, p1: Vec3, mat: Arc<dyn Scatter>) -> Self {
        let mut sides = HitableList::new();

        sides.push(Box::new(Rect::new(
            Plane::XY,
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p1.z(),
            mat.clone(),
        )));
        sides.push(Box::new(Rect::new(
            Plane::XY,
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p0.z(),
            mat.clone(),
        )));

        sides.push(Box::new(Rect::new(
            Plane::ZX,
            p0.z(),
            p1.z(),
            p0.x(),
            p1.x(),
            p1.y(),
            mat.clone(),
        )));
        sides.push(Box::new(Rect::new(
            Plane::ZX,
            p0.z(),
            p1.z(),
            p0.x(),
            p1.x(),
            p0.y(),
            mat.clone(),
        )));

        sides.push(Box::new(Rect::new(
            Plane::YZ,
            p0.y(),
            p1.y(),
            p0.z(),
            p1.z(),
            p1.x(),
            mat.clone(),
        )));
        sides.push(Box::new(Rect::new(
            Plane::YZ,
            p0.y(),
            p1.y(),
            p0.z(),
            p1.z(),
            p0.x(),
            mat,
        )));

        Self {
            box_min: p0,
            box_max: p1,
            sides,
        }
    }
}

impl Hittable for Cube {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.sides.hit(r, t_min, t_max)
    }

    fn bounding_box(&self, _time0: f32, _time1: f32) -> Option<AABB> {
        Some(AABB::new(self.box_min, self.box_max))
    }
}
