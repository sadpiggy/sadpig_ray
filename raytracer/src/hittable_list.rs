use crate::aabb::Aabb;
pub use crate::matirial::Material;
use crate::moving_sphere::MovingSphere;
use crate::rtweekend::random_int_a_b;
pub use crate::vec3::Vec3;
use crate::RAY::{HitRecord, HitRecordstatic, Hittable, Hittablestatic, Ray};
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
            }
        }
        return hit_anything;
    }

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

    fn pdf_value(&self, o: &Vec3, v: &Vec3) -> f64 {
        let weight = 1.0 / (self.objects.len() as f64);
        let mut sum = 0.0;
        for object in &self.objects {
            sum += weight * object.pdf_value(o, v);
        }
        sum
    }

    fn random(&self, o: &Vec3) -> Vec3 {
        let int_size = self.objects.len() as i32;
        self.objects
            .get(random_int_a_b(0, int_size) as usize)
            .unwrap()
            .random(o)
    }
}
unsafe impl Sync for HittableList {}
unsafe impl Send for HittableList {}

#[derive(Clone)]
pub struct HittableListstatic {
    pub objects: Vec<Arc<dyn Hittablestatic>>,
}
impl HittableListstatic {
    pub fn new_zero() -> HittableListstatic {
        HittableListstatic { objects: vec![] }
    }

    pub fn add(&mut self, object: Arc<dyn Hittablestatic>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}
impl Hittablestatic for HittableListstatic {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecordstatic> {
        let mut rec_mid: Option<HitRecordstatic> = None;
        let mut closest_so_far = t_max;
        for object in self.objects.iter() {
            if let Some(tmp) = object.hit(r, t_min, closest_so_far) {
                rec_mid = Some(tmp.clone());
                closest_so_far = tmp.t;
            }
        }
        rec_mid
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        if self.objects.is_empty() {
            return None;
        }
        let mut bbox;
        if let Some(mid_box) = self.objects[0].bounding_box(time0, time1) {
            bbox = mid_box;
        } else {
            return None;
        }
        for i in 1..self.objects.len() {
            if let Some(mid_box) = self.objects[i].bounding_box(time0, time1) {
                bbox = MovingSphere::surrounding_box(&bbox, &mid_box);
            } else {
                return None;
            }
        }
        Some(bbox)
    }

    fn pdf_value(&self, o: &Vec3, v: &Vec3) -> f64 {
        let weight = 1.0 / (self.objects.len() as f64);
        let mut sum = 0.0;
        for object in &self.objects {
            sum += weight * object.pdf_value(o, v);
        }
        sum
    }

    fn random(&self, o: &Vec3) -> Vec3 {
        let int_size = self.objects.len() as i32;
        self.objects
            .get(random_int_a_b(0, int_size) as usize)
            .unwrap()
            .random(o)
    }
}
unsafe impl Sync for HittableListstatic {}
unsafe impl Send for HittableListstatic {}
