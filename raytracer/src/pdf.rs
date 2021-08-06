use crate::camera::random_double_0_1;
use crate::onb::Onb;

use crate::Vec3;
use crate::RAY::{Hittable, Hittablestatic, Sphere};
use std::f64::consts::PI;
use std::sync::Arc;

pub trait PDF {
    fn value(&self, dire: &Vec3) -> f64;

    fn generate(&self) -> Vec3;
}
//impl Copy for std::sync::Arc<dyn PDF>{}

pub trait PDFstatic: PDF + Clone {}

pub struct CosinePdf {
    pub uvw: Onb,
}

impl CosinePdf {
    pub fn new_zero() -> CosinePdf {
        CosinePdf {
            uvw: Onb::new_zero(),
        }
    }

    pub fn new(w: &Vec3) -> CosinePdf {
        let mut pig = Onb::new_zero();
        pig.build_from_w(w);
        CosinePdf { uvw: pig }
    }
}

impl PDF for CosinePdf {
    fn value(&self, dire: &Vec3) -> f64 {
        let cosine = Vec3::unit_vector(dire).dot(&self.uvw.w());
        return if cosine <= 0.0 { 0.0 } else { cosine / PI };
    }

    fn generate(&self) -> Vec3 {
        self.uvw.local_2(&Vec3::random_cosine_dire())
    }
}

pub struct CosinePdfstatic {
    pub uvw: Onb,
}

impl CosinePdfstatic {
    pub fn new_zero() -> CosinePdfstatic {
        CosinePdfstatic {
            uvw: Onb::new_zero(),
        }
    }

    pub fn new(w: &Vec3) -> CosinePdfstatic {
        let mut pig = Onb::new_zero();
        pig.build_from_w(w);
        CosinePdfstatic { uvw: pig }
    }
}

impl PDF for CosinePdfstatic {
    fn value(&self, dire: &Vec3) -> f64 {
        let cosine = Vec3::unit_vector(dire).dot(&self.uvw.w());
        return if cosine <= 0.0 { 0.0 } else { cosine / PI };
    }

    fn generate(&self) -> Vec3 {
        self.uvw.local_2(&Vec3::random_cosine_dire())
    }
}

impl Clone for CosinePdfstatic {
    fn clone(&self) -> Self {
        Self {
            uvw: self.uvw.clone(),
        }
    }
}

impl PDFstatic for CosinePdfstatic {}

pub struct HittablePdf {
    pub o: Vec3,
    pub ptr: Arc<dyn Hittable>,
}

impl HittablePdf {
    pub fn new_zero() -> HittablePdf {
        HittablePdf {
            o: Vec3::zero(),
            ptr: Arc::new(Sphere::new_zero()),
        }
    }

    pub fn new(p: Arc<dyn Hittable>, origin: &Vec3) -> HittablePdf {
        HittablePdf {
            o: origin.clone(),
            ptr: p.clone(),
        }
    }
}

impl PDF for HittablePdf {
    fn value(&self, dire: &Vec3) -> f64 {
        self.ptr.pdf_value(&self.o, dire)
    }

    fn generate(&self) -> Vec3 {
        self.ptr.random(&self.o)
    }
}

pub struct HittablePdfstatic<'a, T: Hittablestatic> {
    pub o: Vec3,
    pub ptr: &'a T,
}

impl<'a, T: Hittablestatic> HittablePdfstatic<'a, T> {
    pub fn new(p: &'a T, origin: &Vec3) -> Self {
        Self {
            o: origin.clone(),
            ptr: p,
        }
    }
}

impl<'a, T: Hittablestatic> PDF for HittablePdfstatic<'a, T> {
    fn value(&self, dire: &Vec3) -> f64 {
        self.ptr.pdf_value(&self.o, dire)
    }

    fn generate(&self) -> Vec3 {
        self.ptr.random(&self.o)
    }
}

impl<'a, T: Hittablestatic> Clone for HittablePdfstatic<'a, T> {
    fn clone(&self) -> Self {
        Self {
            o: self.o.clone(),
            ptr: self.ptr,
        }
    }
}

impl<'a, T: Hittablestatic> PDFstatic for HittablePdfstatic<'a, T> {}

pub struct MixturePdf {
    p0: Arc<dyn PDF>,
    p1: Arc<dyn PDF>,
}

impl MixturePdf {
    pub fn new_zero() -> MixturePdf {
        MixturePdf {
            p0: Arc::new(HittablePdf::new_zero()),
            p1: Arc::new(HittablePdf::new_zero()),
        }
    }
    pub fn new(p0: Arc<dyn PDF>, p1: Arc<dyn PDF>) -> MixturePdf {
        MixturePdf { p0, p1 }
    }
}

impl PDF for MixturePdf {
    fn value(&self, dire: &Vec3) -> f64 {
        return 0.5 * self.p0.value(dire) + 0.5 * self.p1.value(dire);
    }

    fn generate(&self) -> Vec3 {
        if random_double_0_1() < 0.5 {
            return self.p0.generate();
        }
        self.p1.generate()
    }
}

pub struct MixturePdfstatic<'a, T0: PDFstatic, T1: PDFstatic> {
    p0: &'a T0,
    p1: &'a T1,
}

impl<'a, T0: PDFstatic, T1: PDFstatic> MixturePdfstatic<'a, T0, T1> {
    pub fn new(p0: &'a T0, p1: &'a T1) -> Self {
        Self { p0, p1 }
    }
}

impl<'a, T0: PDFstatic, T1: PDFstatic> PDF for MixturePdfstatic<'a, T0, T1> {
    fn value(&self, dire: &Vec3) -> f64 {
        return 0.5 * self.p0.value(dire) + 0.5 * self.p1.value(dire);
    }

    fn generate(&self) -> Vec3 {
        if random_double_0_1() < 0.5 {
            return self.p0.generate();
        }
        self.p1.generate()
    }
}

impl<'a, T0: PDFstatic, T1: PDFstatic> Clone for MixturePdfstatic<'a, T0, T1> {
    fn clone(&self) -> Self {
        Self {
            p0: self.p0,
            p1: self.p1,
        }
    }
}

impl<'a, T0: PDFstatic, T1: PDFstatic> PDFstatic for MixturePdfstatic<'a, T0, T1> {}

pub struct NonePdf {
    pub val: f64,
}

impl NonePdf {
    pub fn new() -> NonePdf {
        Self { val: 0.0 }
    }
}

impl PDF for NonePdf {
    fn value(&self, dire: &Vec3) -> f64 {
        return 0.0;
    }
    fn generate(&self) -> Vec3 {
        return Vec3::zero();
    }
}
