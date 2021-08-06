use crate::rtweekend::{f_max, f_min};
use crate::Vec3;
use crate::RAY::Ray;

pub struct Aabb {
    pub minimum: Vec3,
    pub maximum: Vec3,
}

impl Aabb {
    pub fn new(mini: Vec3, maxi: Vec3) -> Aabb {
        Aabb {
            minimum: mini,
            maximum: maxi,
        }
    }
    pub fn new_zero() -> Aabb {
        Aabb {
            minimum: Vec3::zero(),
            maximum: Vec3::zero(),
        }
    }
}

impl Aabb {
    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        for a in 0..3 {
            let invD = 1.0 / r.dire.get_xyz(a);
            let mut t0 = (self.minimum.get_xyz(a) - r.orig.get_xyz(a)) * invD;
            let mut t1 = (self.maximum.get_xyz(a) - r.orig.get_xyz(a)) * invD;

            if invD < 0.0 {
                let mut mid = t0;
                t0 = t1;
                t1 = mid;
            }
            let pig1 = f_max(t0, t_min);
            let pig2 = f_min(t1, t_max);
            if pig2 <= pig1 {
                return false;
            }
        }
        true
    }
}
