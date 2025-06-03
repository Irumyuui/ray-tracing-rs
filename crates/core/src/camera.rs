#![allow(unused)]

use std::io::Write;

use indicatif::{ProgressBar, ProgressStyle};

use crate::{
    hit::Hittable,
    interval::Interval,
    ray::Ray,
    vec3::{Color, Point3, Vector3, Wrapper},
};

pub struct Camera {
    image_height: i32,

    pub aspect_ratio: f32,
    pub image_width: i32,

    center: Point3,
    pixel00_loc: Point3,

    pixel_delta_u: Point3,
    pixel_delta_v: Point3,
}

impl Camera {
    pub fn render<H: Hittable>(&mut self, world: &H) -> anyhow::Result<()> {
        let pb = ProgressBar::new(self.image_height as u64);
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

        writeln!(writer, "P3")?;
        writeln!(writer, "{} {}", self.image_width, self.image_height)?;
        writeln!(writer, "255")?;

        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let pixel_center = self.pixel00_loc
                    + (i as f32 * self.pixel_delta_u)
                    + (j as f32 * self.pixel_delta_v);
                let ray_direction = pixel_center - self.center;
                let r = Ray::new(self.center, ray_direction);

                let pixel_color = Self::ray_color(&r, world);
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

    fn ray_color<H: Hittable>(r: &Ray, world: &H) -> Color {
        if let Some(rec) = world.hit(r, &Interval::new(0.0, f32::INFINITY)) {
            return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
        }

        let unit_direction = r.direction().unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}

pub struct CameraBuilder {
    pub aspect_ratio: f32,
    pub image_width: i32,
}

impl Default for CameraBuilder {
    fn default() -> Self {
        Self {
            aspect_ratio: 16.0 / 9.0,
            image_width: 400,
        }
    }
}

impl CameraBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Camera {
        let aspect_ratio = self.aspect_ratio;
        let image_width = self.image_width;
        let image_height = match (image_width as f32 / aspect_ratio) as i32 {
            n if n < 1 => 1,
            n => n,
        };

        let center = Point3::new(0.0, 0.0, 0.0);

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f32 / image_height as f32);

        let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / image_width as f32;
        let pixel_delta_v = viewport_v / image_height as f32;

        let viewport_upper_left =
            center - Vector3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Camera {
            image_height,
            aspect_ratio,
            image_width,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }
}
