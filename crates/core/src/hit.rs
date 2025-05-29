use std::sync::Arc;

use crate::{
    ray::Ray,
    vec3::{Point3, Vector3},
};

#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vector3,
    pub t: f32,

    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vector3) {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -(*outward_normal)
        };
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_tmin: f32, ray_tmax: f32) -> Option<HitRecord>;
}

#[derive(Default, Clone)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new(objects: Vec<Arc<dyn Hittable>>) -> Self {
        Self { objects }
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_tmin: f32, ray_tmax: f32) -> Option<HitRecord> {
        let mut closet_so_far = ray_tmax;
        let mut res = None;

        for obj in self.objects.iter() {
            if let Some(record) = obj.hit(r, ray_tmin, closet_so_far) {
                closet_so_far = record.t;
                res.replace(record);
            }
        }

        res
    }
}
