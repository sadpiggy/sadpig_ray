use crate::rtweekend;
use crate::{vec3, Vec3};
use std::ops::Mul;

#[derive(Clone)]
pub struct Onb {
    pub axis: [Vec3; 3],
}
impl Onb {
    pub fn new_zero() -> Onb {
        Onb {
            axis: [Vec3::zero(); 3],
        }
    }

    pub fn get_vec3(&self, index: usize) -> Vec3 {
        self.axis.get(index).unwrap().clone()
    }

    pub fn u(&self) -> Vec3 {
        self.axis.get(0).unwrap().clone()
    }

    pub fn v(&self) -> Vec3 {
        self.axis.get(1).unwrap().clone()
    }

    pub fn w(&self) -> Vec3 {
        self.axis.get(2).unwrap().clone()
    }

    pub fn local_1(&self, a: f64, b: f64, c: f64) -> Vec3 {
        self.u().mul(a) + self.v().mul(b) + self.w().mul(c)
    }

    pub fn local_2(&self, a: &Vec3) -> Vec3 {
        self.u().mul(a.x) + self.v().mul(a.y) + self.w().mul(a.z)
    }

    pub fn build_from_w(&mut self, n: &Vec3) {
        *(self.axis.get_mut(2).unwrap()) = Vec3::unit_vector(n);
        let a: Vec3 = if (self.w().x).abs() > 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };
        *(self.axis.get_mut(1).unwrap()) = Vec3::unit_vector(&(self.w().cross(&a)));
        *(self.axis.get_mut(0).unwrap()) = self.w().cross(&self.v())
    }
}
