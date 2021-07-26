use crate::aabb::Aabb;
use crate::matirial::Lambertian;
use crate::rtweekend::random_double_a_b;
use crate::RAY;
use crate::RAY::{HitRecord, Hittable, Material, Ray};
use crate::{rtweekend, Vec3};
use std::f64::INFINITY;
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

    fn pdf_value(&self, o: &Vec3, v: &Vec3) -> f64 {
        0.0
    }

    fn random(&self, o: &Vec3) -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
    }
}

pub struct XzRect {
    mp: Arc<dyn Material>,
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
}

impl XzRect {
    pub fn new(x0_: f64, x1_: f64, z0_: f64, z1_: f64, k_: f64, mp_: Arc<dyn Material>) -> XzRect {
        XzRect {
            mp: mp_,
            x0: x0_,
            x1: x1_,
            z0: z0_,
            z1: z1_,
            k: k_,
        }
    }

    pub fn new_zero() -> XzRect {
        XzRect {
            mp: Arc::new(Lambertian::new_zero()),
            x0: 0.0,
            x1: 0.0,
            z0: 0.0,
            z1: 0.0,
            k: 0.0,
        }
    }
}

impl Hittable for XzRect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = (self.k - r.orig.y) / (r.dire.y);
        if t < t_min || t > t_max {
            return false;
        }
        let x = r.orig.x + t * r.dire.x;
        let z = r.orig.z + t * r.dire.z;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return false;
        }
        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.t = t;
        let outward_normal = Vec3::new(0.0, 1.0, 0.0);
        rec.p = r.at(t);
        rec.set_face_normal(r, &outward_normal);
        rec.mat_ptr = self.mp.clone();
        true
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        output_box.minimum = Vec3::new(self.x0, self.k - 0.0001, self.z0);
        output_box.maximum = Vec3::new(self.x1, self.k + 0.0001, self.z1);
        true
    }

    fn pdf_value(&self, o: &Vec3, v: &Vec3) -> f64 {
        let mut rec = HitRecord::new_blank(); //书上没有写ray 的time应该是多少 todo
        if !self.hit(&Ray::new2(o, v, 0.0), 0.001, INFINITY, &mut rec) {
            return 0.0;
        }
        let area = (self.x1 - self.x0) * (self.z1 - self.z0);
        let dist_squared = rec.t * rec.t * v.squared_length();
        let cosine = (v.dot(&rec.normal) / v.length()).abs();
        dist_squared / (cosine * area)
    }

    fn random(&self, o: &Vec3) -> Vec3 {
        let random_point = Vec3::new(
            random_double_a_b(self.x0, self.x1),
            self.k,
            random_double_a_b(self.z0, self.z1),
        );
        random_point - o.clone()
    }
}

pub struct YzRect {
    mp: Arc<dyn Material>,
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64,
}

impl YzRect {
    pub fn new(y0_: f64, y1_: f64, z0_: f64, z1_: f64, k_: f64, mp_: Arc<dyn Material>) -> YzRect {
        YzRect {
            mp: mp_,
            y0: y0_,
            y1: y1_,
            z0: z0_,
            z1: z1_,
            k: k_,
        }
    }

    pub fn new_zero() -> YzRect {
        YzRect {
            mp: Arc::new(Lambertian::new_zero()),
            y0: 0.0,
            y1: 0.0,
            z0: 0.0,
            z1: 0.0,
            k: 0.0,
        }
    }
}

impl Hittable for YzRect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = (self.k - r.orig.x) / (r.dire.x);
        if t < t_min || t > t_max {
            return false;
        }
        let y = r.orig.y + t * r.dire.y;
        let z = r.orig.z + t * r.dire.z;
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return false;
        }
        rec.u = (y - self.y0) / (self.y1 - self.y0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.t = t;
        let outward_normal = Vec3::new(1.0, 0.0, 0.0);
        rec.p = r.at(t);
        rec.set_face_normal(r, &outward_normal);
        rec.mat_ptr = self.mp.clone();
        true
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        output_box.minimum = Vec3::new(self.k - 0.0001, self.y0, self.z0);
        output_box.maximum = Vec3::new(self.k + 0.0001, self.y1, self.z1);
        true
    }

    fn pdf_value(&self, o: &Vec3, v: &Vec3) -> f64 {
        0.0
    }

    fn random(&self, o: &Vec3) -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
    }
}
