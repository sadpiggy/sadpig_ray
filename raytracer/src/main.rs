mod RAY;
mod aabb;
mod aarect_h;
mod bvh;
mod camera;
mod hittable_list;
mod matirial;
mod moving_sphere;
mod perlin;
mod rtweekend;
mod texture;
#[allow(clippy::float_cmp)]
mod vec3;

use crate::camera::{random_double_0_1, Camera};
use crate::hittable_list::HittableList;
use crate::matirial::{Dielectric, Lambertian, Material, Metal};
use crate::rtweekend::{clamp, earth, random_secne, simple_light, two_perlin_spheres, two_spheres};
use crate::RAY::Sphere;
use core::fmt::Alignment::Center;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
//pub use ray::Ray;
use image::imageops::FilterType::Lanczos3;
use std::f64::consts::PI;
use std::ops::{Add, AddAssign, Div, Mul, Sub};
use std::sync::Arc;
use std::vec::Vec;
pub use vec3::Vec3;

fn main() {
    let aspect_ratio_ = 3.0 / 2.0;
    let image_width: u32 = 1200;
    let image_height: u32 = (((image_width) as f64) / aspect_ratio_) as u32;
    //渲染质量
    let mut samples_per_pixels: u32 = 500;
    let max_depth = 50;
    //world
    let R = (PI / 4.0).cos();
    let mut world: HittableList; // HittableList { objects: vec![] };
    let mut world = random_secne();
    let mut vfov_ = 40.0;
    let mut aperture_ = 0.0;
    let mut look_from_: Vec3 = Vec3::zero(); // = (Vec3::new(12.0, 2.0, 3.0));
    let mut look_at_: Vec3 = Vec3::zero(); // = (Vec3::new(0.0, 0.0, 0.0));
    let mut background = Vec3::zero();

    let mut case = 4;
    if case == 0 {
        world = random_secne();
        background = Vec3::new(0.7, 0.8, 1.0);
        look_from_ = Vec3::new(13.0, 2.0, 3.0);
        look_at_ = Vec3::new(0.0, 0.0, 0.0);
        vfov_ = 20.0;
        aperture_ = 0.1;
    }
    if case == 1 {
        world = two_spheres();
        background = Vec3::new(0.7, 0.8, 1.0);
        look_from_ = Vec3::new(13.0, 2.0, 3.0);
        look_at_ = Vec3::new(0.0, 0.0, 0.0);
        vfov_ = 20.0;
        aperture_ = 0.1;
    }
    if case == 2 {
        world = two_perlin_spheres();
        background = Vec3::new(0.7, 0.8, 1.0);
        look_from_ = Vec3::new(13.0, 2.0, 3.0);
        look_at_ = Vec3::new(0.0, 0.0, 0.0);
        vfov_ = 20.0;
        aperture_ = 0.1;
    }
    if case == 3 {
        world = earth();
        background = Vec3::new(0.7, 0.8, 1.0);
        look_from_ = Vec3::new(13.0, 2.0, 3.0);
        look_at_ = Vec3::new(0.0, 0.0, 0.0);
        vfov_ = 20.0;
        aperture_ = 0.1;
    }
    if case == 4 {
        world = simple_light();
        background = Vec3::new(0.0, 0.0, 0.0);
        look_from_ = Vec3::new(26.0, 3.0, 6.0);
        look_at_ = Vec3::new(0.0, 2.0, 0.0);
        vfov_ = 20.0;
    }

    //camera
    let cam = Camera::new(
        &look_from_,
        &look_at_,
        &(Vec3::new(0.0, 1.0, 0.0)),
        vfov_,
        aspect_ratio_,
        aperture_,
        10.0,
        0.0,
        1.0,
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
                pixel_color.add_assign(r.ray_color(&background, &world, 20));
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
