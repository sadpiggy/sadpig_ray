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

use crate::hittable_list::HittableList;
use crate::matirial::Material;

//pub use ray::Ray;

use crate::run::Runstatic;

pub use vec3::Vec3;

fn main() {
    //Run();
    Runstatic();
}
