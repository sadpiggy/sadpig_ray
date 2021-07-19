use crate::aabb::Aabb;
use crate::matirial::Lambertian;
use crate::RAY;
use crate::RAY::{HitRecord, Hittable, Material, Ray};
use crate::{rtweekend, Vec3};
use std::sync::Arc;

pub struct XyRect {
    mp: Arc<dyn Material>,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
}

impl XyRect {
    pub fn new(x0_: f64, x1_: f64, y0_: f64, y1_: f64, k_: f64, mp_: Arc<dyn Material>) -> XyRect {
        XyRect {
            mp: mp_,
            x0: x0_,
            x1: x1_,
            y0: y0_,
            y1: y1_,
            k: k_,
        }
    }

    pub fn new_zero() -> XyRect {
        XyRect {
            mp: Arc::new(Lambertian::new_zero()),
            x0: 0.0,
            x1: 0.0,
            y0: 0.0,
            y1: 0.0,
            k: 0.0,
        }
    }
}

impl Hittable for XyRect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = (self.k - r.orig.z) / (r.dire.z);
        if t < t_min || t > t_max {
            return false;
        }
        let x = r.orig.x + t * r.dire.x;
        let y = r.orig.y + t * r.dire.y;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return false;
        }
        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (y - self.y0) / (self.y1 - self.y0);
        rec.t = t;
        let outward_normal = Vec3::new(0.0, 0.0, 1.0);
        rec.p = r.at(t);
        rec.set_face_normal(r, &outward_normal);
        rec.mat_ptr = self.mp.clone();
        true
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        output_box.minimum = Vec3::new(self.x0, self.y0, self.k - 0.0001);
        output_box.maximum = Vec3::new(self.x1, self.y1, self.k + 0.0001);
        true
    }
}
