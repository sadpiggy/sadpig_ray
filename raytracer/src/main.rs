mod BOX_H;
mod RAY;
mod aabb;
mod aarect_h;
mod bvh;
mod camera;
mod constant_medium;
mod hittable_list;
mod matirial;
mod moving_sphere;
mod perlin;
mod rtweekend;
mod texture;
#[allow(clippy::float_cmp)]
mod vec3;

use crate::camera::{random_double_0_1, random_double_a_b, Camera};
use crate::hittable_list::HittableList;
use crate::matirial::{Dielectric, Lambertian, Material, Metal};
use crate::rtweekend::{
    clamp, cornell_box, cornell_smoke, earth, final_scene, random_secne, simple_light,
    two_perlin_spheres, two_spheres,
};
use crate::RAY::Sphere;
use core::fmt::Alignment::Center;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
//pub use ray::Ray;
use image::imageops::FilterType::Lanczos3;
use std::f64::consts::PI;
use std::ops::{Add, AddAssign, Div, Mul, Sub};
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::vec::Vec;
use threadpool::ThreadPool;
pub use vec3::Vec3;

fn main() {
    let mut aspect_ratio_ = 3.0 / 2.0;
    let mut image_width: u32 = 1200;
    let mut image_height: u32 = (((image_width) as f64) / aspect_ratio_) as u32;
    //渲染质量
    let mut samples_per_pixels: u32 = 500;
    let max_depth = 100;
    //world
    let R = (PI / 4.0).cos();
    let mut world: HittableList = HittableList::new_zero(); // HittableList { objects: vec![] };
    let mut vfov_ = 40.0;
    let mut aperture_ = 0.0;
    let mut look_from_: Vec3 = Vec3::zero(); // = (Vec3::new(12.0, 2.0, 3.0));
    let mut look_at_: Vec3 = Vec3::zero(); // = (Vec3::new(0.0, 0.0, 0.0));
    let mut background = Vec3::zero();

    let mut case = 7;
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
    if case == 5 {
        world = cornell_box();
        background = Vec3::new(0.0, 0.0, 0.0);
        look_from_ = Vec3::new(278.0, 278.0, -800.0);
        look_at_ = Vec3::new(278.0, 278.0, 0.0);
        vfov_ = 40.0;
        aspect_ratio_ = 1.0;
        image_width = 600;
        image_height = image_width;
    }
    if case == 6 {
        world = cornell_smoke();
        background = Vec3::new(0.0, 0.0, 0.0);
        look_from_ = Vec3::new(278.0, 278.0, -800.0);
        look_at_ = Vec3::new(278.0, 278.0, 0.0);
        vfov_ = 40.0;
        aspect_ratio_ = 1.0;
        image_width = 600;
        image_height = image_width;
    }
    if case == 7 {
        world = final_scene();
        background = Vec3::new(0.0, 0.0, 0.0);
        look_from_ = Vec3::new(478.0, 278.0, -600.0);
        look_at_ = Vec3::new(278.0, 278.0, 0.0);
        vfov_ = 40.0;
        aspect_ratio_ = 1.0;
        image_width = 800;
        image_height = image_width;
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

    let (tx, rx) = channel();
    let n_jobs = 32;
    let n_workers = 6;
    let pool = ThreadPool::new(n_workers);

    let mut results: RgbImage = ImageBuffer::new(image_width as u32, image_height as u32);
    let bar = ProgressBar::new(n_jobs as u64);

    for i in 0..n_jobs {
        let tx = tx.clone();
        let world_ = world.clone();
        //let lights_ptr = lights.clone();
        pool.execute(move || {
            let row_begin = image_height as usize * i as usize / n_jobs;
            let row_end = image_height as usize * (i as usize + 1) / n_jobs;
            let render_height = row_end - row_begin;
            let mut img: RgbImage = ImageBuffer::new(image_width as u32, render_height as u32);
            for x in 0..image_width {
                for (img_y, y) in (row_begin..row_end).enumerate() {
                    let y = y as u32;
                    let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
                    for _s in 0..samples_per_pixels {
                        let u: f64 =
                            ((x) as f64 + random_double_0_1()) / ((image_width - 1) as f64);
                        let v: f64 = ((image_height - y) as f64 + random_double_0_1())
                            / ((image_height - 1) as f64);
                        let r = cam.get_ray(u, v);
                        pixel_color.add_assign(r.ray_color(&background, &world_, max_depth));
                    }
                    let pixel = img.get_pixel_mut(x as u32, img_y as u32);
                    *pixel = image::Rgb([
                        pixel_color.get_u8_x(samples_per_pixels),
                        pixel_color.get_u8_y(samples_per_pixels),
                        pixel_color.get_u8_z(samples_per_pixels),
                    ]);
                }
            }
            tx.send((row_begin..row_end, img))
                .expect("failed to send result");
        });
    }

    for (rows, data) in rx.iter().take(n_jobs) {
        for (idx, row) in rows.enumerate() {
            for col in 0..image_width {
                let row = row as u32;
                let idx = idx as u32;
                *results.get_pixel_mut(col as u32, row) = *data.get_pixel(col as u32, idx);
            }
        }
        bar.inc(1);
    }

    results.save("output/test.png").unwrap();
    bar.finish();
}
