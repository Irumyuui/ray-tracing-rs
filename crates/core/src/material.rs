use std::ops::Neg;

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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Self {
        Self {
            albedo,
            fuzz: fuzz.clamp(0.0, 1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = ray_in.direction().reflect(&rec.normal).unit_vector()
            + self.fuzz * Vector3::random_unit_vector();
        let scattered = Ray::new(rec.p, reflected);

        if scattered.direction().dot(&rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct Dielectric {
    refraction_index: f32,
}

impl Dielectric {
    pub fn new(refraction_index: f32) -> Self {
        Self { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray_in.direction().unit_vector();
        let cos_theta = unit_direction.neg().dot(&rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;
        let direction =
            if cannot_refract || reflectance(cos_theta, ri) > rand::random_range(0.0..=1.0) {
                unit_direction.reflect(&rec.normal)
            } else {
                unit_direction.refract(&rec.normal, ri)
            };

        let scattered = Ray::new(rec.p, direction);
        Some((attenuation, scattered))
    }
}

#[inline(always)]
fn reflectance(cosine: f32, refraction_index: f32) -> f32 {
    let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}
