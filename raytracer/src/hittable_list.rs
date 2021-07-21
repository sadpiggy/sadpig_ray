use crate::aabb::Aabb;
pub use crate::matirial::Material;
use crate::moving_sphere::MovingSphere;
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
    pub fn new_zero() -> HittableList {
        HittableList { objects: vec![] }
    }

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

    // if (objects.empty()) return false;
    //
    // aabb temp_box;
    // bool first_box = true;
    //
    // for (const auto& object : objects) {
    // if (!object->bounding_box(time0, time1, temp_box)) return false;
    // output_box = first_box ? temp_box : surrounding_box(output_box, temp_box);
    // first_box = false;
    // }
    //
    // return true;

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        if self.objects.is_empty() {
            return false;
        }
        let mut temp_box = Aabb::new_zero();
        let mut first_box = true;
        for object in &self.objects {
            if !object.bounding_box(time0, time1, &mut temp_box) {
                return false;
            }
            if first_box {
                output_box.minimum = temp_box.minimum.clone();
                output_box.maximum = temp_box.maximum.clone();
            } else {
                let pig = MovingSphere::surrounding_box(&output_box, &temp_box);
                output_box.minimum = pig.minimum.clone();
                output_box.maximum = pig.maximum.clone();
                first_box = false;
            }
        }
        true
    }
}

unsafe impl Sync for HittableList {}
unsafe impl Send for HittableList {}
