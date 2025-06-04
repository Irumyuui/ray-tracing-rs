use std::sync::Arc;

use rt_core::{
    camera::CameraBuilder,
    hit::HittableList,
    material::{Lambertian, Metal},
    sphere::Sphere,
    vec3::{Color, Point3},
};

fn main() -> anyhow::Result<()> {
    let mut world = HittableList::default();

    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -100.6, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    CameraBuilder {
        aspect_ratio: 16.0 / 9.0,
        image_width: 400,
        samples_per_pixel: 100,
        max_depth: 50,
    }
    .build()
    .render(&world)?;

    Ok(())
}
