use crate::rtweekend::degrees_to_radians;
pub use crate::rtweekend::random_double_0_1;
pub use crate::rtweekend::random_double_a_b;
pub use crate::vec3::Vec3;
pub use crate::RAY::Ray;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Debug)]
pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f64,
    pub time0: f64,
    pub time1: f64,
}

impl Camera {
    pub fn new(
        look_from: &Vec3,
        look_at: &Vec3,
        v_up: &Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
        t0: f64,
        t1: f64,
    ) -> Camera {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let v_h = 2.0 * h;
        let v_w = aspect_ratio * v_h;

        let w_ = Vec3::unit_vector(&(look_from.sub(look_at.clone())));
        let u_ = Vec3::unit_vector(&(v_up.cross(&w_)));
        let v_ = w_.cross(&u_);

        let origin_ = look_from.clone();
        let horizontal_ = u_.mul(v_w).mul(focus_dist);
        let vertical_ = v_.mul(v_h).mul(focus_dist);
        Camera {
            origin: origin_.clone(),
            horizontal: horizontal_.clone(),
            vertical: vertical_.clone(),
            lower_left_corner: origin_
                .sub(horizontal_.div(2.0))
                .sub(vertical_.div(2.0))
                .sub(w_.mul(focus_dist)),
            lens_radius: aperture / 2.0,
            w: w_,
            v: v_,
            u: u_,
            time0: t0,
            time1: t1,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = Vec3::random_in_unit_disk().mul(self.lens_radius);
        let offset = (self.u.mul(rd.x)).add(self.v.mul(rd.y));
        Ray {
            orig: self.origin.add(offset.clone()),
            dire: self
                .lower_left_corner
                .add(self.horizontal.mul(s))
                .add(self.vertical.mul(t))
                .sub(self.origin.add(offset)),
            time: random_double_a_b(self.time0, self.time1),
        }
    }
}
