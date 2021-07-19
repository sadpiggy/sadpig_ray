use crate::aabb::Aabb;
use crate::aarect_h::{XyRect, XzRect, YzRect};
use crate::rtweekend;
use crate::HittableList;
use crate::Vec3;
use crate::RAY::{HitRecord, Hittable, Material, Ray};
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
        false //todo maybe
    }
}
