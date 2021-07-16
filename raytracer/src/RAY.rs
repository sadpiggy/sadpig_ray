use crate::matirial::Lambertian;
pub use crate::matirial::Material;
pub use crate::vec3::Vec3;
use std::f64::INFINITY;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use std::sync::Arc;

#[derive(Clone)]
//pub use crate::vec3::Ray;

pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat_ptr: Arc<dyn Material>,
}
//设置为mut
impl HitRecord {
    pub fn new(
        p_: Vec3,
        normal_: Vec3,
        t_: f64,
        front_face_: bool,
        mat_ptr_: Arc<dyn Material>,
    ) -> HitRecord {
        HitRecord {
            p: p_,
            normal: normal_,
            t: t_,
            front_face: front_face_,
            mat_ptr: mat_ptr_,
        }
    }

    pub fn new_blank() -> HitRecord {
        let pig = Lambertian::new(&Vec3::zero());
        HitRecord {
            p: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            normal: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            t: 0.0,
            front_face: false,
            mat_ptr: Arc::new(pig),
        }
    }

    //这里·可能·有bug
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = (r.dire.dot(&outward_normal.clone()) < 0.0);
        if self.front_face {
            //println!("niamisisi");
            self.normal = (outward_normal).clone();
        } else {
            // println!("llllll");
            self.normal = (outward_normal).clone().mul(-1.0);
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub mat_ptr: Arc<dyn Material>,
}

pub struct Ray {
    pub orig: Vec3,
    pub dire: Vec3,
    pub time: f64,
}

impl Ray {
    pub fn new(x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64, t: f64) -> Self {
        Self {
            orig: Vec3 {
                x: x1,
                y: y1,
                z: z1,
            },
            dire: Vec3 {
                x: x2,
                y: y2,
                z: z2,
            },
            time: t,
        }
    }

    pub fn new2(orig_: &Vec3, dire_: &Vec3, t: f64) -> Self {
        Self {
            orig: Vec3 {
                x: orig_.x,
                y: orig_.y,
                z: orig_.z,
            },
            dire: Vec3 {
                x: dire_.x,
                y: dire_.y,
                z: dire_.z,
            },
            time: t,
        }
    }

    pub fn get_orig(&self) -> &Vec3 {
        &self.orig
    }

    pub fn get_dire(&self) -> &Vec3 {
        &self.dire
    }

    pub fn at(&self, t: f64) -> Vec3 {
        Vec3 {
            x: self.orig.x + self.dire.x * t,
            y: self.orig.y + self.dire.y * t,
            z: self.orig.z + self.dire.z * t,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = r.orig.sub(self.center);
        let a: f64 = r.dire.squared_length();
        let half_b: f64 = r.dire.dot(&oc);
        let c: f64 = oc.squared_length() - self.radius * self.radius;
        let pan: f64 = half_b * half_b - a * c;
        if pan <= 0.0 {
            return false;
        };

        let root: f64 = pan.sqrt();
        let t: f64 = (-half_b - root) / a;
        if t > t_min && t < t_max {
            rec.t = t;
            rec.p = r.at(t);
            rec.set_face_normal(&r, &((rec.p.sub(self.center.clone())).div(self.radius)));
            rec.mat_ptr = self.mat_ptr.clone();
            return true;
        }
        let t: f64 = (-half_b + root) / a;
        if t > t_min && t < t_max {
            rec.t = t;
            rec.p = r.at(t);
            rec.set_face_normal(&r, &((rec.p.sub(self.center.clone())).div(self.radius)));
            rec.mat_ptr = self.mat_ptr.clone();
            return true;
        }
        return false;
    }
}

impl Ray {
    pub fn ray_color(&self, world: &dyn Hittable, depth: i32) -> Vec3 {
        let mut rec: HitRecord = HitRecord::new_blank();
        if depth <= 0 {
            return Vec3::zero();
        }
        let inf = INFINITY;
        if world.hit(&self, 0.001, inf, &mut rec) {
            let mut scattered = Ray::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
            let mut attenuation = Vec3::zero();
            if rec
                .mat_ptr
                .scatter(&self, &rec, &mut attenuation, &mut scattered)
            {
                return scattered.ray_color(world, depth - 1).mul(attenuation);
            }
            return Vec3::zero();
        }
        // let unit_dire = self.dire.clone();
        let t: f64 = (self.dire.y + 1.0) * 0.5;
        let v1: Vec3 = Vec3::new(1.0, 1.0, 1.0).mul(1.0 - t);
        v1.add((Vec3::new(0.5, 0.7, 1.0).mul(t)))
    }
    // pub fn ray_color(&self) -> Vec3 {
    //     let mid: Vec3 = Vec3::new(0.0, 0.0, -1.0);
    //     let t = self.hit_sphere(mid, 0.5);
    //     if t > 0.0 {
    //         let N: Vec3 = self.at(t).sub(Vec3::new(0.0, 0.0, -1.0));
    //         return Vec3::new(N.x + 1.0, N.y + 1.0, N.z + 1.0).mul(0.5);
    //     }
    //
    //     let t = 0.5 * (self.dire.y + 1.0);
    //     let v1 = Vec3::new(1.0, 1.0, 1.0);
    //     let v2 = Vec3::new(0.5, 0.7, 1.0);
    //     let v1 = (v1).mul((1.0 - t));
    //     let v2 = v2.mul(t);
    //     v1.add(v2)
    // }
}
