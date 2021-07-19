use crate::aabb::Aabb;
pub use crate::hittable_list::HittableList;
use crate::matirial::{Dielectric, Lambertian, Material, Metal};
use crate::rtweekend::{f_max, f_min};
use crate::Vec3;
pub use crate::RAY::Sphere;
pub use crate::RAY::{HitRecord, Hittable, Ray};
pub use std::alloc::handle_alloc_error;
use std::ops::{Add, Div, Mul, Sub};
use std::sync::Arc;

#[derive(Clone)]
pub struct MovingSphere {
    pub center0: Vec3,
    pub center1: Vec3,
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
    pub mat_ptr: Arc<dyn Material>,
}

impl MovingSphere {
    pub fn new(
        cen0: Vec3,
        cen1: Vec3,
        t0: f64,
        t1: f64,
        rad: f64,
        mat_ptr_: Arc<dyn Material>,
    ) -> MovingSphere {
        //mat_ptr参数需要引用吗？我不知道//应该不写引用吧
        MovingSphere {
            center0: cen0,
            center1: cen1,
            time0: t0,
            time1: t1,
            radius: rad,
            mat_ptr: mat_ptr_,
        }
    }

    pub fn center(&self, time: f64) -> Vec3 {
        let pig: f64 = (time - self.time0) / (self.time1 - self.time0);
        self.center0
            .add((self.center1.sub(self.center0.clone())).mul(pig))
    }

    pub fn surrounding_box(box0: &Aabb, box1: &Aabb) -> Aabb {
        let small: Vec3 = Vec3::new(
            f_min(box0.minimum.x, box1.minimum.x),
            f_min(box0.minimum.y, box1.minimum.y),
            f_min(box0.minimum.z, box1.minimum.z),
        );
        let big: Vec3 = Vec3::new(
            f_max(box0.maximum.x, box1.maximum.x),
            f_max(box0.maximum.y, box1.maximum.y),
            f_max(box0.maximum.z, box1.maximum.z),
        );
        Aabb {
            minimum: small,
            maximum: big,
        }
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = r.orig.sub(self.center(r.time));
        let a: f64 = r.dire.squared_length();
        let half_b: f64 = r.dire.dot(&oc);
        let c: f64 = oc.squared_length() - self.radius * self.radius;
        let pan: f64 = half_b * half_b - a * c;
        if pan < 0.0 {
            return false;
        };

        let root: f64 = pan.sqrt();
        let t: f64 = (-half_b - root) / a;
        if t > t_min && t < t_max {
            rec.t = t;
            rec.p = r.at(t);
            let outward_normal = (rec.p.sub(self.center(r.time))).div(self.radius);
            rec.set_face_normal(&r, &(outward_normal));
            rec.mat_ptr = self.mat_ptr.clone();
            return true;
        }
        let t: f64 = (-half_b + root) / a;
        if t > t_min && t < t_max {
            rec.t = t;
            rec.p = r.at(t);
            let outward_normal = (rec.p.sub(self.center(r.time))).div(self.radius);
            rec.set_face_normal(&r, &(outward_normal));
            //rec.set_face_normal(&r, &((rec.p.sub(self.center.clone())).div(self.radius)));
            rec.mat_ptr = self.mat_ptr.clone();
            return true;
        }
        return false;
    }

    //     bool moving_sphere::bounding_box(double _time0, double _time1, aabb& output_box) const {
    //     aabb box0(
    //     center(_time0) - vec3(radius, radius, radius),
    //     center(_time0) + vec3(radius, radius, radius));
    //     aabb box1(
    //     center(_time1) - vec3(radius, radius, radius),
    //     center(_time1) + vec3(radius, radius, radius));
    //     output_box = surrounding_box(box0, box1);
    //     return true;
    // }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        let box0 = Aabb {
            minimum: self
                .center(_time0)
                .sub(Vec3::new(self.radius, self.radius, self.radius)),
            maximum: self
                .center(_time0)
                .add(Vec3::new(self.radius, self.radius, self.radius)),
        };
        let box1 = Aabb {
            minimum: self
                .center(_time1)
                .sub(Vec3::new(self.radius, self.radius, self.radius)),
            maximum: self
                .center(_time1)
                .add(Vec3::new(self.radius, self.radius, self.radius)),
        };
        let box_mid = MovingSphere::surrounding_box(&box0, &box1);
        output_box.minimum = box_mid.minimum.clone();
        output_box.maximum = box_mid.maximum.clone();
        true
    }
}