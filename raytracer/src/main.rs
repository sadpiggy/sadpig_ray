mod camera;
mod hittable_list;
mod matirial;
mod RAY;
mod rtweekend;
#[allow(clippy::float_cmp)]
mod vec3;

use crate::camera::{random_double_0_1, Camera};
use crate::hittable_list::{Hittable, HittableList};
use crate::matirial::{Dielectric, Lambertian, Material, Metal};
use crate::ray::Sphere;
use crate::rtweekend::{clamp, random_secne};
use core::fmt::Alignment::Center;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
pub use ray::Ray;
use std::f64::consts::PI;
use std::ops::{Add, AddAssign, Div, Mul, Sub};
use std::sync::Arc;
use std::vec;
pub use vec3::Vec3;

fn main() {
    let aspect_ratio_ = 3.0 / 2.0;
    let image_width: u32 = 1200;
    let image_height: u32 = (((image_width) as f64) / aspect_ratio_) as u32;
    let samples_per_pixels: u32 = 300;
    let max_depth = 100;
    //world
    let R = (PI / 4.0).cos();
    //let mut world: HittableList = HittableList { objects: vec![] };
    let mut world = random_secne();

    //camera
    let look_from_ = (Vec3::new(12.0, 2.0, 3.0));
    let look_at_ = (Vec3::new(0.0, 0.0, 0.0));
    let cam = Camera::new(
        &look_from_,
        &look_at_,
        &(Vec3::new(0.0, 1.0, 0.0)),
        20.0,
        aspect_ratio_,
        0.1,
        10.0,
    );
    //render
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);
    let bar = ProgressBar::new(1024);
    let mut j = (image_height);
    while j > 0 {
        j -= 1;
        for i in 0..image_width {
            let mut pixel_color = Vec3::zero();
            for s in 0..samples_per_pixels {
                let u: f64 = ((i) as f64 + random_double_0_1()) / ((image_width - 1) as f64);
                let v: f64 =
                    ((image_height - j) as f64 + random_double_0_1()) / ((image_height - 1) as f64);
                let r = cam.get_ray(u, v);
                pixel_color.add_assign(r.ray_color(&world, 20));
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
