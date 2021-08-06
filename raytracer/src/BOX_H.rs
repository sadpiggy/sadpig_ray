use crate::aabb::Aabb;
use crate::aarect_h::{XyRect, XyRectstatic, XzRect, XzRectstatic, YzRect, YzRectstatic};
use crate::matirial::Materialstatic;
use crate::HittableList;
use crate::Vec3;
use crate::RAY::{HitRecord, HitRecordstatic, Hittable, Hittablestatic, Material, Ray};
use std::sync::Arc;

pub struct Hezi {
    pub hezi_min: Vec3,
    pub hezi_max: Vec3,
    pub sides: HittableList,
}

impl Hezi {
    pub fn new(p0: Vec3, p1: Vec3, ptr: Arc<dyn Material>) -> Hezi {
        let mut sides_ = HittableList::new_zero();
        sides_.add(Arc::new(XyRect::new(
            p0.x,
            p1.x,
            p0.y,
            p1.y,
            p1.z,
            ptr.clone(),
        )));
        sides_.add(Arc::new(XyRect::new(
            p0.x,
            p1.x,
            p0.y,
            p1.y,
            p0.z,
            ptr.clone(),
        )));

        sides_.add(Arc::new(XzRect::new(
            p0.x,
            p1.x,
            p0.z,
            p1.z,
            p1.y,
            ptr.clone(),
        )));
        sides_.add(Arc::new(XzRect::new(
            p0.x,
            p1.x,
            p0.z,
            p1.z,
            p0.y,
            ptr.clone(),
        )));

        sides_.add(Arc::new(YzRect::new(
            p0.y,
            p1.y,
            p0.z,
            p1.z,
            p1.x,
            ptr.clone(),
        )));
        sides_.add(Arc::new(YzRect::new(
            p0.y,
            p1.y,
            p0.z,
            p1.z,
            p0.x,
            ptr.clone(),
        )));

        Hezi {
            hezi_min: p0,
            hezi_max: p1,
            sides: sides_,
        }
    }
}

impl Hittable for Hezi {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        self.sides.hit(r, t_min, t_max, rec)
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        //true //todo maybe
        output_box.minimum = self.hezi_min.clone();
        output_box.maximum = self.hezi_max.clone();
        true
    }

    fn pdf_value(&self, o: &Vec3, v: &Vec3) -> f64 {
        0.0
    }

    fn random(&self, o: &Vec3) -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
    }
}

pub struct Hezistatic<T: Materialstatic + Clone> {
    pub hezi_min: Vec3,
    pub hezi_max: Vec3,
    pub sides: (
        XyRectstatic<T>,
        XyRectstatic<T>,
        XzRectstatic<T>,
        XzRectstatic<T>,
        YzRectstatic<T>,
        YzRectstatic<T>,
    ),
}

impl<T: Materialstatic + Clone> Hezistatic<T> {
    pub fn new(p0: Vec3, p1: Vec3, ptr: T) -> Self {
        Self {
            sides: (
                XyRectstatic::new(p0.x, p1.x, p0.y, p1.y, p1.z, ptr.clone()),
                XyRectstatic::new(p0.x, p1.x, p0.y, p1.y, p0.z, ptr.clone()),
                XzRectstatic::new(p0.x, p1.x, p0.z, p1.z, p1.y, ptr.clone()),
                XzRectstatic::new(p0.x, p1.x, p0.z, p1.z, p0.y, ptr.clone()),
                YzRectstatic::new(p0.y, p1.y, p0.z, p1.z, p1.x, ptr.clone()),
                YzRectstatic::new(p0.y, p1.y, p0.z, p1.z, p0.x, ptr.clone()),
            ),
            hezi_min: p0,
            hezi_max: p1,
        }
    }
}

impl<T: Materialstatic + Clone> Hittablestatic for Hezistatic<T> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecordstatic> {
        let mut ans: Option<HitRecordstatic> = None;
        let mut closest = t_max;
        if let Some(rec) = self.sides.0.hit(r, t_min, closest) {
            ans = Some(rec.clone());
            closest = rec.t;
        }
        if let Some(rec) = self.sides.1.hit(r, t_min, closest) {
            ans = Some(rec.clone());
            closest = rec.t;
        }
        if let Some(rec) = self.sides.2.hit(r, t_min, closest) {
            ans = Some(rec.clone());
            closest = rec.t;
        }
        if let Some(rec) = self.sides.3.hit(r, t_min, closest) {
            ans = Some(rec.clone());
            closest = rec.t;
        }
        if let Some(rec) = self.sides.4.hit(r, t_min, closest) {
            ans = Some(rec.clone());
            closest = rec.t;
        }
        if let Some(rec) = self.sides.5.hit(r, t_min, closest) {
            ans = Some(rec.clone());
        }
        ans
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        Some(Aabb {
            minimum: self.hezi_min.clone(),
            maximum: self.hezi_max.clone(),
        })
    }

    fn pdf_value(&self, o: &Vec3, v: &Vec3) -> f64 {
        0.0
    }

    fn random(&self, o: &Vec3) -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
    }
}
