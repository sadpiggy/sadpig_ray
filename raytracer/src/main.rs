mod camera;
mod hittable_list;
mod ray;
mod rtweekend;
#[allow(clippy::float_cmp)]
mod vec3;

use crate::camera::{random_double_0_1, Camera};
use crate::hittable_list::{Hittable, HittableList};
use crate::ray::Sphere;
use crate::rtweekend::clamp;
use core::fmt::Alignment::Center;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
pub use ray::Ray;
use std::ops::{Add, AddAssign, Div, Mul, Sub};
use std::sync::Arc;
use std::vec;
pub use vec3::Vec3;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height: u32 = (((image_width) as f64) / aspect_ratio) as u32;
    let samples_per_pixels: u32 = 100;
    //world
    let mut world: HittableList = HittableList { objects: vec![] };
    let pig: Arc<dyn Hittable> = Arc::new(Sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
    });
    world.add(pig);
    let pig2: Arc<dyn Hittable> = Arc::new(Sphere {
        center: Vec3::new(0.0, -100.5, -1.0),
        radius: 100.0,
    });
    world.add(pig2);
    //camera
    let cam = Camera::new();
    //render
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);
    let bar = ProgressBar::new(1024);

    let mut j = (image_height);
    // for j in 0..image_height
    while j > 0 {
        j -= 1;
        for i in 0..image_width {
            let mut pixel_color = Vec3::zero();
            for s in 0..samples_per_pixels {
                let u: f64 = ((i) as f64 + random_double_0_1()) / ((image_width - 1) as f64);
                let v: f64 =
                    ((image_height - j) as f64 + random_double_0_1()) / ((image_height - 1) as f64);
                let r = cam.get_ray(u, v);
                pixel_color.add_assign(r.ray_color(&world));
            }
            //
            let pixel = img.get_pixel_mut(i, j);
            *pixel = image::Rgb([
                (pixel_color.get_u8_x(samples_per_pixels)),
                (pixel_color.get_u8_y(samples_per_pixels)),
                (pixel_color.get_u8_z(samples_per_pixels)),
            ]);
        }
        bar.inc(1);
    }

    img.save("output/test.png").unwrap();
    bar.finish();
}
