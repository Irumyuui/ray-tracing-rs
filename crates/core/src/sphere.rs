use crate::{
    hit::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
    vec3::Point3,
};

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    center: Point3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32) -> Self {
        Self {
            center,
            radius: radius.max(0.),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, interval: &Interval) -> Option<HitRecord> {
        let oc = self.center - r.origin();

        let a = r.direction().length_squared();
        let h = r.direction().dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();
        let mut root = (h - sqrt_d) / a;
        if !interval.surrounds(root) {
            root = (h + sqrt_d) / a;
            if !interval.surrounds(root) {
                return None;
            }
        }

        let t = root;
        let p = r.at(t);
        let n = (p - self.center) / self.radius;

        let outward_normal = (p - self.center) / self.radius;

        let mut rec = HitRecord {
            p,
            normal: n,
            t,
            front_face: false,
        };
        rec.set_face_normal(r, &outward_normal);

        Some(rec)
    }
}
