pub use crate::camera::{random_double_0_1, Camera};
pub use crate::hittable_list::HittableList;
use crate::onb;
use crate::onb::Onb;
use crate::pdf::{CosinePdf, CosinePdfstatic, HittablePdf, PDFstatic, PDF};
pub use crate::rtweekend::clamp;
use crate::rtweekend::schlick;
use crate::texture::{SolidColor, SolidColorstatic, Texture, Texturestatic};
use crate::Vec3;
pub use crate::RAY::Sphere;
use crate::RAY::{HitRecord, HitRecordstatic, Ray};
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

pub struct ScatterRecordstatic {
    pub specular_ray: Ray,
    pub is_specular: bool,
    pub attenuation: Vec3,
    pub pdf_ptr: CosinePdfstatic,
}

impl ScatterRecordstatic {
    pub fn new(
        specular_ray_: Ray,
        is_specular_: bool,
        attenuation_: Vec3,
        pdf_ptr_: CosinePdfstatic,
    ) -> ScatterRecordstatic {
        ScatterRecordstatic {
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

pub trait Materialstatic {
    fn scatter(&self, r_in: &Ray, rec: &HitRecordstatic) -> Option<ScatterRecordstatic>;

    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecordstatic, scattered: &mut Ray) -> f64;

    fn emitted(&self, r_in: &Ray, rec: &HitRecordstatic, u: f64, v: f64, p: &Vec3) -> Vec3;

    //fn fuzhi(&self)->Self;
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

pub struct Lambertianstatic<T: Texturestatic> {
    pub albedo: T,
}

impl<T: Texturestatic> Lambertianstatic<T> {
    pub fn new(a: &Vec3) -> Lambertianstatic<SolidColorstatic> {
        Lambertianstatic {
            albedo: SolidColorstatic::new(a.clone()),
        }
    }

    pub fn new1(a: T) -> Lambertianstatic<T> {
        Lambertianstatic { albedo: a }
    }
}

impl<T: Texturestatic> Lambertianstatic<T> {
    pub fn new_zero() -> Lambertianstatic<SolidColorstatic> {
        Lambertianstatic {
            albedo: SolidColorstatic::new(Vec3::zero()),
        }
    }

    pub fn cao() -> f64 {
        5.0
    }
}

impl<T: Texturestatic> Materialstatic for Lambertianstatic<T> {
    fn scatter(&self, r_in: &Ray, rec: &HitRecordstatic) -> Option<ScatterRecordstatic> {
        Option::from(ScatterRecordstatic {
            specular_ray: Ray::new2(&Vec3::zero(), &Vec3::zero(), 0.0),
            is_specular: false,
            attenuation: self.albedo.value(rec.u, rec.v, &rec.p),
            pdf_ptr: (CosinePdfstatic::new(&rec.normal)),
        })
    }

    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecordstatic, scattered: &mut Ray) -> f64 {
        let cosine = rec.normal.dot(&Vec3::unit_vector(&scattered.dire));
        if cosine < 0.0 {
            return 0.0;
        }
        cosine / PI
    }

    fn emitted(&self, r_in: &Ray, rec: &HitRecordstatic, u: f64, v: f64, p: &Vec3) -> Vec3 {
        Vec3::zero()
    }
}

impl<T: Texturestatic> Clone for Lambertianstatic<T> {
    fn clone(&self) -> Self {
        Self {
            albedo: self.albedo.clone(),
        }
    }
}

pub struct Metalstatic {
    pub albedo: Vec3,
    pub fuzz: f64,
}

impl Metalstatic {
    pub fn new(a: &Vec3, f: f64) -> Metalstatic {
        let mut pig: f64;
        if f < 1.0 {
            pig = f
        } else {
            pig = 1.0
        };
        Metalstatic {
            albedo: Vec3 {
                x: a.x,
                y: a.y,
                z: a.z,
            },
            fuzz: pig,
        }
    }
}

impl Materialstatic for Metalstatic {
    fn scatter(&self, r_in: &Ray, rec: &HitRecordstatic) -> Option<ScatterRecordstatic> {
        let reflected = Vec3::reflect(&Vec3::unit_vector(&r_in.dire), &rec.normal);
        let pipixia = Ray::new2(
            &rec.p,
            &(reflected + (Vec3::random_in_unit_sphere()) * self.fuzz),
            0.0,
        ); //time默认值是多少来着
        Option::from(ScatterRecordstatic {
            specular_ray: pipixia,
            is_specular: true,
            attenuation: self.albedo.clone(),
            pdf_ptr: CosinePdfstatic::new(&rec.normal),
        })
    }

    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecordstatic, scattered: &mut Ray) -> f64 {
        let cosine = rec.normal.dot(&Vec3::unit_vector(&scattered.dire));
        if cosine < 0.0 {
            return 0.0;
        }
        cosine / PI
    }

    fn emitted(&self, r_in: &Ray, rec: &HitRecordstatic, u: f64, v: f64, p: &Vec3) -> Vec3 {
        Vec3::zero()
    }
}

impl Clone for Metalstatic {
    fn clone(&self) -> Self {
        Self {
            albedo: self.albedo.clone(),
            fuzz: self.fuzz,
        }
    }
}

pub struct Dielectricstatic {
    pub ref_idx: f64,
}

impl Dielectricstatic {
    pub fn new(a: f64) -> Dielectricstatic {
        Dielectricstatic { ref_idx: a }
    }
}

impl Materialstatic for Dielectricstatic {
    fn scatter(&self, r_in: &Ray, rec: &HitRecordstatic) -> Option<ScatterRecordstatic> {
        let is_specular_ = true;
        let pdf_ptr_ = (CosinePdfstatic::new(&rec.normal));
        let attenuation_ = Vec3::new(1.0, 1.0, 1.0);

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
            //serc.specular_ray = Ray::new2(&rec.p, &reflected, r_in.time);
            return Option::from(ScatterRecordstatic {
                specular_ray: Ray::new2(&rec.p, &reflected, r_in.time),
                is_specular: true,
                attenuation: Vec3::new(1.0, 1.0, 1.0),
                pdf_ptr: pdf_ptr_,
            });
        }

        let reflect_prob = schlick(cos_theta, etai);
        if random_double_0_1() < reflect_prob {
            let reflected = Vec3::reflect(&(unit_v), &rec.normal);
            //serc.specular_ray = Ray::new2(&rec.p, &reflected, r_in.time);
            return Option::from(ScatterRecordstatic {
                specular_ray: Ray::new2(&rec.p, &reflected, r_in.time),
                is_specular: is_specular_,
                attenuation: attenuation_,
                pdf_ptr: pdf_ptr_,
            });
        }

        let refracted = Vec3::refract(&(unit_v), &rec.normal, etai);
        //let r_mid = Ray::new2(&rec.p, &refracted, r_in.time);
        //serc.specular_ray = Ray::new2(&rec.p, &refracted, r_in.time);
        Option::from(ScatterRecordstatic {
            specular_ray: Ray::new2(&rec.p, &refracted, r_in.time),
            is_specular: is_specular_,
            attenuation: attenuation_,
            pdf_ptr: pdf_ptr_,
        })
    }

    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecordstatic, scattered: &mut Ray) -> f64 {
        let cosine = rec.normal.dot(&Vec3::unit_vector(&scattered.dire));
        if cosine < 0.0 {
            return 0.0;
        }
        cosine / PI
    }

    fn emitted(&self, r_in: &Ray, rec: &HitRecordstatic, u: f64, v: f64, p: &Vec3) -> Vec3 {
        Vec3::zero()
    }
}

impl Clone for Dielectricstatic {
    fn clone(&self) -> Self {
        Self {
            ref_idx: self.ref_idx,
        }
    }
}

pub struct DiffuseLightstatic<T: Texturestatic> {
    pub emit: T,
}

impl<T: Texturestatic> DiffuseLightstatic<T> {
    pub fn new(a: T) -> DiffuseLightstatic<T> {
        DiffuseLightstatic { emit: a }
    }

    pub fn new2(c: Vec3) -> DiffuseLightstatic<SolidColorstatic> {
        DiffuseLightstatic {
            emit: (SolidColorstatic::new(c)),
        }
    }
}

impl<T: Texturestatic> Materialstatic for DiffuseLightstatic<T> {
    fn scatter(&self, r_in: &Ray, rec: &HitRecordstatic) -> Option<ScatterRecordstatic> {
        None
    }

    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecordstatic, scattered: &mut Ray) -> f64 {
        let cosine = rec.normal.dot(&Vec3::unit_vector(&scattered.dire));
        if cosine < 0.0 {
            return 0.0;
        }
        cosine / PI
    }

    //todo
    fn emitted(&self, r_in: &Ray, rec: &HitRecordstatic, u: f64, v: f64, p: &Vec3) -> Vec3 {
        self.emit.value(u, v, p)
    }
}

impl<T: Texturestatic> Clone for DiffuseLightstatic<T> {
    fn clone(&self) -> Self {
        Self {
            emit: self.emit.clone(),
        }
    }
}

pub struct Iostropicstatic<T: Texturestatic> {
    pub albedo: T,
}

impl<T: Texturestatic> Iostropicstatic<T> {
    pub fn new(c: Vec3) -> Iostropicstatic<SolidColorstatic> {
        Iostropicstatic {
            albedo: (SolidColorstatic::new(c)),
        }
    }

    pub fn new2(a: T) -> Iostropicstatic<T> {
        Iostropicstatic { albedo: a }
    }
}

impl<T: Texturestatic> Materialstatic for Iostropicstatic<T> {
    fn scatter(&self, r_in: &Ray, rec: &HitRecordstatic) -> Option<ScatterRecordstatic> {
        None
    }

    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecordstatic, scattered: &mut Ray) -> f64 {
        let cosine = rec.normal.dot(&Vec3::unit_vector(&scattered.dire));
        if cosine < 0.0 {
            return 0.0;
        }
        cosine / PI
    }

    fn emitted(&self, r_in: &Ray, rec: &HitRecordstatic, u: f64, v: f64, p: &Vec3) -> Vec3 {
        Vec3::zero()
    }
}

impl<T: Texturestatic> Clone for Iostropicstatic<T> {
    fn clone(&self) -> Self {
        Self {
            albedo: self.albedo.clone(),
        }
    }
}
