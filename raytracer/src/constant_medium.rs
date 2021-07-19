use crate::aabb::Aabb;
use crate::camera::random_double_0_1;
use crate::matirial::Iostropic;
use crate::texture;
use crate::texture::Texture;
use crate::Material;
use crate::RAY;
use crate::RAY::{HitRecord, Hittable, Ray};
use crate::{rtweekend, Vec3};
use std::f64::consts::E;
use std::f64::INFINITY;
use std::sync::Arc;

pub struct ConstantMedium {
    pub boundary: Arc<dyn Hittable>,
    pub phase_function: Arc<dyn Material>,
    pub neg_inv_density: f64,
}

impl ConstantMedium {
    pub fn new(b: Arc<dyn Hittable>, d: f64, a: Arc<dyn Texture>) -> ConstantMedium {
        ConstantMedium {
            boundary: b,
            phase_function: Arc::new(Iostropic::new2(a)),
            neg_inv_density: (-1.0 / d),
        }
    }
    pub fn new2(b: Arc<dyn Hittable>, d: f64, c: Vec3) -> ConstantMedium {
        ConstantMedium {
            boundary: b,
            phase_function: Arc::new(Iostropic::new(c)),
            neg_inv_density: (-1.0 / d),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let enableDebug = false;
        let debugging = enableDebug && random_double_0_1() < 0.00001;
        let mut rec1: HitRecord = HitRecord::new_blank();
        let mut rec2 = HitRecord::new_blank();
        if !self.boundary.hit(r, -INFINITY, INFINITY, &mut rec1) {
            return false;
        }
        if !self.boundary.hit(r, rec1.t + 0.0001, INFINITY, &mut rec2) {
            return false;
        }
        if debugging {
            println!("laozi is in 45line in constant_.lalala.rs");
        }
        if rec1.t < t_min {
            rec1.t = t_min;
        }
        if rec2.t > t_max {
            rec2.t = t_max;
        }
        if rec1.t >= rec2.t {
            return false;
        }
        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }
        let ray_length = r.dire.length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * (random_double_0_1().log(E));

        if hit_distance > distance_inside_boundary {
            return false;
        }
        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = r.at(rec.t);
        if debugging {
            println!("laozi is in 57line in constant_.lalala.rs");
        }
        rec.normal = Vec3::new(1.0, 0.0, 0.0);
        rec.front_face = true;
        rec.mat_ptr = self.phase_function.clone();
        true
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        self.boundary.bounding_box(time0, time1, output_box)
    }
}
