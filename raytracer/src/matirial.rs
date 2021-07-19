pub use crate::camera::{random_double_0_1, Camera};
pub use crate::hittable_list::HittableList;
pub use crate::rtweekend::clamp;
use crate::rtweekend::schlick;
use crate::texture::{SolidColor, Texture};
use crate::Vec3;
pub use crate::RAY::Sphere;
use crate::RAY::{HitRecord, Ray};
use std::alloc::handle_alloc_error;
use std::collections::hash_map::Entry::Vacant;
use std::ops::{Add, Mul};
use std::sync::Arc;

//unit_direction
pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;

    fn emitted(&self, u: f64, v: f64, p: &Vec3) -> Vec3;
}

pub struct Lambertian {
    pub albedo: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(a: &Vec3) -> Lambertian {
        Lambertian {
            albedo: Arc::new(SolidColor::new(a.clone())),
        }
    }

    pub fn new1(a: Arc<dyn Texture>) -> Lambertian {
        Lambertian { albedo: a }
    }
}

impl Lambertian {
    pub fn new_zero() -> Lambertian {
        Lambertian {
            albedo: Arc::new(SolidColor::new(Vec3::zero())),
        }
    }

    pub fn cao() -> f64 {
        5.0
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        mut scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal.add(Vec3::random_unit_vector());
        //todo
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal.clone();
        }
        let mut pig = Ray::new2(&rec.p, &scatter_direction, r_in.time);
        scattered.time = pig.time;
        scattered.orig = pig.orig.clone();
        scattered.dire = pig.dire.clone();
        let dog = self.albedo.value(rec.u, rec.v, &rec.p);
        attenuation.x = dog.x;
        attenuation.y = dog.y;
        attenuation.z = dog.z;
        true
    }

    fn emitted(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        //todo
        Vec3::zero()
    }
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(a: &Vec3, f: f64) -> Metal {
        let mut pig: f64;
        if f < 1.0 {
            pig = f
        } else {
            pig = 1.0
        };
        Metal {
            albedo: Vec3 {
                x: a.x,
                y: a.y,
                z: a.z,
            },
            fuzz: pig,
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let unit_v = Vec3::unit_vector(&(r_in.dire));
        let reflected = Vec3::reflect(&unit_v, &rec.normal);
        let r_mid = Ray::new2(
            &(rec.p),
            &(reflected.add((Vec3::random_in_unit_sphere()).mul(self.fuzz.clone()))),
            r_in.time,
        );
        scattered.dire = r_mid.dire.clone();
        scattered.orig = r_mid.orig.clone();
        attenuation.x = self.albedo.x;
        attenuation.y = self.albedo.y;
        attenuation.z = self.albedo.z;
        scattered.dire.dot(&(rec.normal)) > 0.0
    }

    fn emitted(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        Vec3::zero()
    }
}

pub struct Dielectric {
    pub ref_idx: f64,
}

impl Dielectric {
    pub fn new(a: f64) -> Dielectric {
        Dielectric { ref_idx: a }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        attenuation.x = 1.0;
        attenuation.y = 1.0;
        attenuation.z = 1.0;
        let etai: f64;
        if rec.front_face {
            etai = 1.0 / self.ref_idx;
        } else {
            etai = self.ref_idx;
        }
        let unit_v = Vec3::unit_vector(&(r_in.dire));
        let cos_theta: f64;
        let hap = unit_v.mul(-1.0).dot(&rec.normal);
        if hap < 1.0 {
            cos_theta = hap;
        } else {
            cos_theta = 1.0;
        }
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        if etai * sin_theta > 1.0 {
            let reflected = Vec3::reflect(&(unit_v), &rec.normal);
            let r_mid = Ray::new2(&rec.p, &reflected, r_in.time);
            scattered.dire = r_mid.dire.clone();
            scattered.orig = r_mid.orig.clone();
            return true;
        }

        let reflect_prob = schlick(cos_theta, etai);
        if random_double_0_1() < reflect_prob {
            let reflected = Vec3::reflect(&(unit_v), &rec.normal);
            let r_mid = Ray::new2(&rec.p, &reflected, r_in.time);
            scattered.dire = r_mid.dire.clone();
            scattered.orig = r_mid.orig.clone();
            return true;
        }

        let refracted = Vec3::refract(&(unit_v), &rec.normal, etai);
        let r_mid = Ray::new2(&rec.p, &refracted, r_in.time);
        scattered.dire = r_mid.dire.clone();
        scattered.orig = r_mid.orig.clone();
        true
    }

    fn emitted(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        Vec3::zero()
    }
}

pub struct DiffuseLight {
    pub emit: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(a: Arc<dyn Texture>) -> DiffuseLight {
        DiffuseLight { emit: a }
    }

    pub fn new2(c: Vec3) -> DiffuseLight {
        DiffuseLight {
            emit: Arc::new(SolidColor::new(c)),
        }
    }

    // pub fn emitted(&self,u:f64,v:f64,p:&Vec3)->Vec3{
    //     self.emit.value(u,v,&p)
    // }
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        false
    }

    //todo
    fn emitted(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        self.emit.value(u, v, p)
    }
}
