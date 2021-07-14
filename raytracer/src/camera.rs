pub use crate::ray::Ray;
pub use crate::rtweekend::random_double_0_1;
pub use crate::rtweekend::random_double_a_b;
pub use crate::vec3::Vec3;
use std::ops::{Add, Div, Mul, Sub};
#[derive(Clone, Debug)]
pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        let aspect_ration: f64 = 16.0 / 9.0;
        let v_h = 2.0;
        let v_w = aspect_ration * v_h;
        let focal_length = 1.0;
        let origin_ = Vec3::new(0.0, 0.0, 0.0);
        let horizontal_ = Vec3::new(v_w, 0.0, 0.0);
        let vertical_ = Vec3::new(0.0, v_h, 0.0);
        Camera {
            origin: Vec3::new(0.0, 0.0, 0.0),
            horizontal: Vec3::new(v_w, 0.0, 0.0),
            vertical: Vec3::new(0.0, v_h, 0.0),
            lower_left_corner: origin_
                .sub(horizontal_.div(2.0))
                .sub(vertical_.div(2.0))
                .sub(Vec3::new(0.0, 0.0, focal_length)),
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            orig: self.origin,
            dire: self
                .lower_left_corner
                .add(self.horizontal.mul(u))
                .add(self.vertical.mul(v))
                .sub(self.origin),
        }
    }
}
