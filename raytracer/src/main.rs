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
mod onb;
mod pdf;
mod perlin;
mod rtweekend;
mod run;
mod texture;
#[allow(clippy::float_cmp)]
mod vec3;

use crate::camera::{random_double_0_1, random_double_a_b, Camera};
use crate::hittable_list::{HittableList, HittableListstatic};
use crate::matirial::{Dielectric, Lambertian, Material, Metal};
use crate::rtweekend::{
    clamp, cornell_box, cornell_smoke, earth, final_scene, random_secne, simple_light,
    two_perlin_spheres, two_spheres, two_spheres_static,
};
use crate::RAY::{Hittable, Sphere};
use core::fmt::Alignment::Center;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
//pub use ray::Ray;
use crate::aarect_h::XzRect;
use crate::run::{Run, Runstatic};
use image::imageops::FilterType::Lanczos3;
use std::f64::consts::PI;
use std::ops::{Add, AddAssign, Div, Mul, Sub};
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::vec::Vec;
use threadpool::ThreadPool;
pub use vec3::Vec3;

fn main() {
    //Run();
    Runstatic();
}
