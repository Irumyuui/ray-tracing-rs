use std::sync::Arc;

use rt_core::{camera::CameraBuilder, hit::HittableList, sphere::Sphere, vec3::Point3};

fn main() -> anyhow::Result<()> {
    let mut world = HittableList::default();
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    CameraBuilder {
        aspect_ratio: 16.0 / 9.0,
        image_width: 400,
        samples_per_pixel: 100,
    }
    .build()
    .render(&world)?;

    Ok(())
}
