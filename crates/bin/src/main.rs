use rt_core::{
    hit::{Hittable, HittableList},
    interval::Interval,
    ray::Ray,
    sphere::Sphere,
    vec3::{Color, Point3, Vector3, Wrapper},
};
use std::{error::Error, io::Write, sync::Arc};

use indicatif::{ProgressBar, ProgressStyle};

fn main() -> Result<(), Box<dyn Error>> {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = match (image_width as f32 / aspect_ratio) as i32 {
        n if n < 1 => 1,
        n => n,
    };

    // World
    let mut world = HittableList::default();
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    // Right-handed coordinates.
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f32 / image_height as f32);
    let camera_center = Point3::new(0., 0., 0.);

    let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / image_width as f32;
    let pixel_delta_v = viewport_v / image_height as f32;

    let viewport_upper_left =
        camera_center - Vector3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let pb = ProgressBar::new(image_height as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len}")?
            .progress_chars("#>-"),
    );

    let image = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open("image.ppm")?;
    let mut writer = std::io::BufWriter::new(image);

    // Render
    writeln!(writer, "P3")?;
    writeln!(writer, "{} {}", image_width, image_height)?;
    writeln!(writer, "255")?;

    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (i as f32 * pixel_delta_u) + (j as f32 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&r, &world);
            writeln!(writer, "{}", Wrapper::new(&pixel_color))?;
        }
        pb.inc(1);
    }
    writer.flush()?;
    drop(writer);

    pb.finish_and_clear();
    println!("Image rendered to 'image.ppm'");

    Ok(())
}

fn ray_color(r: &Ray, world: &HittableList) -> Color {
    if let Some(rec) = world.hit(r, &Interval::new(0.0, f32::INFINITY)) {
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
    }

    let unit_direction = r.direction().unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}
