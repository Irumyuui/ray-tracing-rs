use std::sync::Arc;

use crate::{
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{Point3, Vector3},
};

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vector3,
    pub t: f32,

    pub front_face: bool,

    pub mat: Arc<dyn Material + 'static>,
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
    fn hit(&self, r: &Ray, interval: &Interval) -> Option<HitRecord>;
}

#[derive(Default, Clone)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable + Send + Sync + 'static>>,
}

impl HittableList {
    pub fn new(objects: Vec<Arc<dyn Hittable + Send + Sync + 'static>>) -> Self {
        Self { objects }
    }

    pub fn add(&mut self, object: Arc<dyn Hittable + Send + Sync + 'static>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, interval: &Interval) -> Option<HitRecord> {
        let mut closet_so_far = interval.max;
        let mut res = None;

        for obj in self.objects.iter() {
            if let Some(record) = obj.hit(r, &Interval::new(interval.min, closet_so_far)) {
                closet_so_far = record.t;
                res.replace(record);
            }
        }

        res
    }
}
