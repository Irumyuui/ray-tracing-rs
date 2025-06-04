use crate::{
    hit::HitRecord,
    ray::Ray,
    vec3::{Color, Vector3},
};

pub trait Material {
    /// Return attenuation and scattered ray if the ray is scattered.
    fn scatter(&self, _ray_in: &Ray, _rec: &HitRecord) -> Option<(Color, Ray)> {
        None
    }
}

#[derive(Clone)]
pub struct Lambertian {
    albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + Vector3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new(rec.p, scatter_direction);
        let attenuation = self.albedo;
        Some((attenuation, scattered))
    }
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

#[derive(Clone)]
pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = ray_in.direction().reflect(&rec.normal);
        let scattered = Ray::new(rec.p, reflected);
        Some((self.albedo, scattered))
    }
}
