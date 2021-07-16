pub use crate::matirial::Material;
pub use crate::vec3::Vec3;
use crate::RAY::{HitRecord, Hittable, Ray};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use std::sync::Arc;
use std::vec;

#[derive(Clone)]

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec: HitRecord = HitRecord::new_blank();
        let mut hit_anything: bool = false;
        let mut closet_so_far: f64 = t_max;
        for object in &self.objects {
            if object.hit(&r, t_min, closet_so_far, &mut temp_rec) {
                hit_anything = true;
                closet_so_far = temp_rec.t;
                *rec = (temp_rec).clone();
                // rec.t = temp_rec.t;
                // rec.p = temp_rec.p.clone();
                // rec.normal = temp_rec.normal.clone();
                // rec.front_face = temp_rec.front_face;
            }
        }
        return hit_anything;
    }
}
