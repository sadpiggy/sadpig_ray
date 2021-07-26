use crate::camera::random_double_0_1;
use crate::onb::Onb;
use crate::vec3;
use crate::RAY;
use crate::RAY::{Hittable, Sphere};
use crate::{rtweekend, Vec3};
use std::f64::consts::PI;
use std::sync::Arc;

pub trait PDF {
    fn value(&self, dire: &Vec3) -> f64;

    fn generate(&self) -> Vec3;
}
//impl Copy for std::sync::Arc<dyn PDF>{}

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
