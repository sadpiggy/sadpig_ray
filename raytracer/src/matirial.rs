pub use crate::camera::{random_double_0_1, Camera};
pub use crate::hittable_list::HittableList;
use crate::onb;
use crate::onb::Onb;
use crate::pdf::{CosinePdf, HittablePdf, PDF};
pub use crate::rtweekend::clamp;
use crate::rtweekend::schlick;
use crate::texture::{SolidColor, Texture};
use crate::Vec3;
pub use crate::RAY::Sphere;
use crate::RAY::{HitRecord, Ray};
use std::alloc::handle_alloc_error;
use std::collections::hash_map::Entry::Vacant;
use std::f64::consts::PI;
use std::ops::{Add, Mul};
use std::sync::Arc;
use std::thread::sleep;

//unit_direction

pub struct ScatterRecord {
    pub specular_ray: Ray,
    pub is_specular: bool,
    pub attenuation: Vec3,
    pub pdf_ptr: Arc<dyn PDF>,
}

impl ScatterRecord {
    pub fn new_zero() -> ScatterRecord {
        ScatterRecord {
            specular_ray: Ray::new2(&Vec3::zero(), &Vec3::zero(), 0.0),
            is_specular: false,
            attenuation: Vec3::new2(&Vec3::zero()),
            pdf_ptr: Arc::new(HittablePdf::new_zero()),
        }
    }
    pub fn new(
        specular_ray_: Ray,
        is_specular_: bool,
        attenuation_: Vec3,
        pdf_ptr_: Arc<dyn PDF>,
    ) -> ScatterRecord {
        ScatterRecord {
            specular_ray: specular_ray_,
            is_specular: is_specular_,
            attenuation: attenuation_,
            pdf_ptr: pdf_ptr_,
        }
    }
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool;

    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &mut Ray) -> f64;

    fn emitted(&self, r_in: &Ray, rec: &HitRecord, u: f64, v: f64, p: &Vec3) -> Vec3;
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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        srec.is_specular = false;
        srec.attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        srec.pdf_ptr = Arc::new(CosinePdf::new(&rec.normal));
        true
        // let mut uvw = Onb::new_zero();
        // uvw.build_from_w(&rec.normal);
        // let dire = uvw.local_2(&Vec3::random_cosine_dire());
        //
        // // let mut scatter_direction = rec.normal.add(Vec3::random_unit_vector());
        // //todo
        // // if scatter_direction.near_zero() {
        // //     scatter_direction = rec.normal.clone();
        // // }
        // let mut pig = Ray::new2(&rec.p, &Vec3::unit_vector(&dire), r_in.time);
        // scattered.time = pig.time;
        // scattered.orig = pig.orig.clone();
        // scattered.dire = pig.dire.clone();
        // let dog = self.albedo.value(rec.u, rec.v, &rec.p);
        // attenuation.x = dog.x;
        // attenuation.y = dog.y;
        // attenuation.z = dog.z;
        //
        // // println!("zhiqiande{}", pdf);
        // *pdf = (uvw.w().dot(&scattered.dire) / PI);
        //
        // //println!("{}", pdf);
        //
        // true
    }

    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &mut Ray) -> f64 {
        let cosine = rec.normal.dot(&Vec3::unit_vector(&scattered.dire));
        if cosine < 0.0 {
            return 0.0;
        }
        cosine / PI
    }

    fn emitted(&self, r_in: &Ray, rec: &HitRecord, u: f64, v: f64, p: &Vec3) -> Vec3 {
        //todo  作者在抽什么风？
        //Vec3::zero()
        // if rec.front_face {
        //     return self.albedo.value(u, v, p);
        // }
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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, serc: &mut ScatterRecord) -> bool {
        let reflected = Vec3::reflect(&Vec3::unit_vector(&r_in.dire), &rec.normal);
        serc.specular_ray = Ray::new2(
            &rec.p,
            &(reflected + (Vec3::random_in_unit_sphere()) * self.fuzz),
            0.0,
        ); //time默认值是多少来着
        serc.attenuation = self.albedo.clone();
        serc.is_specular = true;
        serc.pdf_ptr = Arc::new(CosinePdf::new(&rec.normal));
        true
        // let unit_v = Vec3::unit_vector(&(r_in.dire));
        // let reflected = Vec3::reflect(&unit_v, &rec.normal);
        // let r_mid = Ray::new2(
        //     &(rec.p),
        //     &(reflected.add((Vec3::random_in_unit_sphere()).mul(self.fuzz.clone()))),
        //     r_in.time,
        // );
        // scattered.dire = r_mid.dire.clone();
        // scattered.orig = r_mid.orig.clone();
        // attenuation.x = self.albedo.x;
        // attenuation.y = self.albedo.y;
        // attenuation.z = self.albedo.z;
        // scattered.dire.dot(&(rec.normal)) > 0.0
    }

    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &mut Ray) -> f64 {
        let cosine = rec.normal.dot(&Vec3::unit_vector(&scattered.dire));
        if cosine < 0.0 {
            return 0.0;
        }
        cosine / PI
    }

    fn emitted(&self, r_in: &Ray, rec: &HitRecord, u: f64, v: f64, p: &Vec3) -> Vec3 {
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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, serc: &mut ScatterRecord) -> bool {
        serc.is_specular = true;
        serc.pdf_ptr = Arc::new(CosinePdf::new(&rec.normal));
        serc.attenuation = Vec3::new(1.0, 1.0, 1.0);

        // attenuation.x = 1.0;
        // attenuation.y = 1.0;
        // attenuation.z = 1.0;
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
            serc.specular_ray = Ray::new2(&rec.p, &reflected, r_in.time);
            return true;
        }

        let reflect_prob = schlick(cos_theta, etai);
        if random_double_0_1() < reflect_prob {
            let reflected = Vec3::reflect(&(unit_v), &rec.normal);
            serc.specular_ray = Ray::new2(&rec.p, &reflected, r_in.time);
            return true;
        }

        let refracted = Vec3::refract(&(unit_v), &rec.normal, etai);
        //let r_mid = Ray::new2(&rec.p, &refracted, r_in.time);
        serc.specular_ray = Ray::new2(&rec.p, &refracted, r_in.time);
        true
    }

    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &mut Ray) -> f64 {
        let cosine = rec.normal.dot(&Vec3::unit_vector(&scattered.dire));
        if cosine < 0.0 {
            return 0.0;
        }
        cosine / PI
    }

    fn emitted(&self, r_in: &Ray, rec: &HitRecord, u: f64, v: f64, p: &Vec3) -> Vec3 {
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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        false
    }

    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &mut Ray) -> f64 {
        let cosine = rec.normal.dot(&Vec3::unit_vector(&scattered.dire));
        if cosine < 0.0 {
            return 0.0;
        }
        cosine / PI
    }

    //todo
    fn emitted(&self, r_in: &Ray, rec: &HitRecord, u: f64, v: f64, p: &Vec3) -> Vec3 {
        self.emit.value(u, v, p)
    }
}

pub struct Iostropic {
    pub albedo: Arc<dyn Texture>,
}

impl Iostropic {
    pub fn new(c: Vec3) -> Iostropic {
        Iostropic {
            albedo: Arc::new(SolidColor::new(c)),
        }
    }

    pub fn new2(a: Arc<dyn Texture>) -> Iostropic {
        Iostropic { albedo: a }
    }
}

impl Material for Iostropic {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        // let pig = Ray::new2(&rec.p, &Vec3::random_in_unit_sphere(), r_in.time);
        // // scattered.time = pig.time;
        // // scattered.dire = pig.dire.clone();
        // // scattered.orig = pig.orig.clone();
        // srec.specular_ray = pig;
        // let dog = self.albedo.value(rec.u, rec.v, &rec.p);
        // // attenuation.x = dog.x;
        // // attenuation.y = dog.y;
        // // attenuation.z = dog.z;
        // srec.attenuation = dog;
        // true
        false
    }

    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &mut Ray) -> f64 {
        let cosine = rec.normal.dot(&Vec3::unit_vector(&scattered.dire));
        if cosine < 0.0 {
            return 0.0;
        }
        cosine / PI
    }

    fn emitted(&self, r_in: &Ray, rec: &HitRecord, u: f64, v: f64, p: &Vec3) -> Vec3 {
        Vec3::zero()
    }
}
