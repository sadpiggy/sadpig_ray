use crate::aabb::Aabb;
use crate::aarect_h::YzRect;
use crate::camera::random_double_a_b;
pub use crate::matirial::Material;
use crate::matirial::{
    Lambertian, Lambertianstatic, Materialstatic, ScatterRecord, ScatterRecordstatic,
};
use crate::onb::Onb;
use crate::pdf::{CosinePdf, HittablePdf, HittablePdfstatic, MixturePdf, MixturePdfstatic, PDF};
use crate::rtweekend::{degrees_to_radians, f_max, f_min};
use crate::texture::SolidColorstatic;
pub use crate::vec3::Vec3;
use rand::distributions::Open01;
use std::f32::consts::PI;
use std::f64::INFINITY;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use std::sync::Arc;

//pub use crate::vec3::Ray;
#[derive(Clone)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat_ptr: Arc<dyn Material>,
    pub u: f64,
    pub v: f64,
}
//设置为mut
impl HitRecord {
    pub fn new(
        p_: Vec3,
        normal_: Vec3,
        t_: f64,
        front_face_: bool,
        mat_ptr_: Arc<dyn Material>,
        u_: f64,
        v_: f64,
    ) -> HitRecord {
        HitRecord {
            p: p_,
            normal: normal_,
            t: t_,
            front_face: front_face_,
            mat_ptr: mat_ptr_,
            u: u_,
            v: v_,
        }
    }

    pub fn new_blank() -> HitRecord {
        let pig = Lambertian::new(&Vec3::zero());
        HitRecord {
            p: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            normal: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            t: 0.0,
            front_face: false,
            mat_ptr: Arc::new(pig),
            u: 0.0,
            v: 0.0,
        }
    }

    //这里·可能·有bug
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = (r.dire.dot(&outward_normal.clone()) < 0.0);
        if self.front_face {
            //println!("niamisisi");
            // self.clone();
            self.normal = (outward_normal).clone();
        } else {
            // println!("llllll");
            self.normal = (outward_normal).clone().mul(-1.0);
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool;

    fn pdf_value(&self, o: &Vec3, v: &Vec3) -> f64;

    fn random(&self, o: &Vec3) -> Vec3;
}

pub trait Hittablestatic {
    fn hit(&self, r: &Raystatic, t_min: f64, t_max: f64) -> Option<HitRecordstatic>;

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool;

    fn pdf_value(&self, o: &Vec3, v: &Vec3) -> f64;

    fn random(&self, o: &Vec3) -> Vec3;
}

#[derive(Clone)]
pub struct HitRecordstatic<'a> {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat_ptr: &'a dyn Materialstatic,
    pub u: f64,
    pub v: f64,
}
//设置为mut
impl<'a> HitRecordstatic<'a> {
    pub fn new(
        p_: Vec3,
        normal_: Vec3,
        t_: f64,
        front_face_: bool,
        mat_ptr_: &'a dyn Materialstatic,
        u_: f64,
        v_: f64,
    ) -> HitRecordstatic<'a> {
        HitRecordstatic {
            p: p_,
            normal: normal_,
            t: t_,
            front_face: front_face_,
            mat_ptr: mat_ptr_,
            u: u_,
            v: v_,
        }
    }

    //这里·可能·有bug
    // pub fn set_face_normal(&mut self, r: &Raystatic, outward_normal: &Vec3) {
    //     self.front_face = (r.dire.dot(&outward_normal.clone()) < 0.0);
    //     if self.front_face {
    //         self.normal = (outward_normal).clone();
    //     } else {
    //         self.normal = (outward_normal).clone().mul(-1.0);
    //     }
    // }
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub mat_ptr: Arc<dyn Material>,
}

pub struct Ray {
    pub orig: Vec3,
    pub dire: Vec3,
    pub time: f64,
}

impl Ray {
    pub fn new(x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64, t: f64) -> Self {
        Self {
            orig: Vec3 {
                x: x1,
                y: y1,
                z: z1,
            },
            dire: Vec3 {
                x: x2,
                y: y2,
                z: z2,
            },
            time: t,
        }
    }

    pub fn new2(orig_: &Vec3, dire_: &Vec3, t: f64) -> Self {
        Self {
            orig: Vec3 {
                x: orig_.x,
                y: orig_.y,
                z: orig_.z,
            },
            dire: Vec3 {
                x: dire_.x,
                y: dire_.y,
                z: dire_.z,
            },
            time: t,
        }
    }

    pub fn get_orig(&self) -> &Vec3 {
        &self.orig
    }

    pub fn get_dire(&self) -> &Vec3 {
        &self.dire
    }

    pub fn at(&self, t: f64) -> Vec3 {
        Vec3 {
            x: self.orig.x + self.dire.x * t,
            y: self.orig.y + self.dire.y * t,
            z: self.orig.z + self.dire.z * t,
        }
    }
}

impl Sphere {
    pub fn new_zero() -> Sphere {
        Sphere {
            center: Vec3::zero(),
            radius: 0.0,
            mat_ptr: Arc::new(Lambertian::new(&Vec3::zero())),
        }
    }

    pub fn get_sphere_uv(p: &Vec3, u: &mut f64, v: &mut f64) {
        //什么玩意儿？
        let theta = (-(p.y)).acos();
        let phi = ((-p.z) / p.x).atan() + (PI as f64);
        *u = (phi) / (2.0 * (PI as f64));
        *v = theta / (PI as f64);
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = r.orig.sub(self.center);
        let a: f64 = r.dire.squared_length();
        let half_b: f64 = r.dire.dot(&oc);
        let c: f64 = oc.squared_length() - self.radius * self.radius;
        let pan: f64 = half_b * half_b - a * c;
        if pan <= 0.0 {
            return false;
        };
        let root: f64 = pan.sqrt();
        let t: f64 = (-half_b - root) / a;
        if t > t_min && t < t_max {
            rec.t = t;
            rec.p = r.at(t);
            let outward_normal_ = &((rec.p.sub(self.center.clone())).div(self.radius));
            rec.set_face_normal(&r, &outward_normal_);
            Sphere::get_sphere_uv(&outward_normal_, &mut rec.u, &mut rec.v);
            rec.mat_ptr = self.mat_ptr.clone();
            return true;
        }
        let t: f64 = (-half_b + root) / a;
        if t > t_min && t < t_max {
            rec.t = t;
            rec.p = r.at(t);
            let outward_normal_ = &((rec.p.sub(self.center.clone())).div(self.radius));
            rec.set_face_normal(&r, &outward_normal_);
            Sphere::get_sphere_uv(&outward_normal_, &mut rec.u, &mut rec.v);
            // rec.set_face_normal(&r, &((rec.p.sub(self.center.clone())).div(self.radius)));
            rec.mat_ptr = self.mat_ptr.clone();
            return true;
        }
        return false;
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        output_box.minimum = self
            .center
            .sub(Vec3::new(self.radius, self.radius, self.radius));
        output_box.maximum = self
            .center
            .add(Vec3::new(self.radius, self.radius, self.radius));
        true
    }

    fn pdf_value(&self, o: &Vec3, v: &Vec3) -> f64 {
        let mut rec = HitRecord::new_blank();
        if !self.hit(&Ray::new2(o, v, 0.0), 0.001, INFINITY, &mut rec) {
            return 0.0;
        }
        let cos_max = (1.0
            - self.radius * self.radius / (self.center.sub(o.clone()).squared_length()))
        .sqrt();
        let solid_angle = (PI as f64) * 2.0 * (1.0 - cos_max);
        1.0 / solid_angle
    }

    fn random(&self, o: &Vec3) -> Vec3 {
        let dire = self.center.sub(o.clone());
        let dist_square = dire.squared_length();
        let mut uvw = Onb::new_zero();
        uvw.build_from_w(&dire);
        uvw.local_2(&Vec3::random_to_sphere(self.radius, dist_square))
    }
}

pub struct Spherestatic<T: Materialstatic> {
    pub center: Vec3,
    pub radius: f64,
    pub mat_ptr: T,
}

impl<T: Materialstatic> Spherestatic<T> {
    pub fn new_zero() -> Spherestatic<Lambertianstatic<SolidColorstatic>> {
        Spherestatic {
            center: Vec3::zero(),
            radius: 0.0,
            mat_ptr: Lambertianstatic::<SolidColorstatic>::new_zero(),
        }
    }

    pub fn get_sphere_uv(p: &Vec3, u: &mut f64, v: &mut f64) {
        //什么玩意儿？
        let theta = (-(p.y)).acos();
        let phi = ((-p.z) / p.x).atan() + (PI as f64);
        *u = (phi) / (2.0 * (PI as f64));
        *v = theta / (PI as f64);
    }
}

impl<T: Materialstatic> Hittablestatic for Spherestatic<T> {
    fn hit(&self, r: &Raystatic, t_min: f64, t_max: f64) -> Option<HitRecordstatic> {
        let oc: Vec3 = r.orig.sub(self.center);
        let a: f64 = r.dire.squared_length();
        let half_b: f64 = r.dire.dot(&oc);
        let c: f64 = oc.squared_length() - self.radius * self.radius;
        let pan: f64 = half_b * half_b - a * c;
        if pan <= 0.0 {
            return None;
        };
        let root: f64 = pan.sqrt();
        let t: f64 = (-half_b - root) / a;
        if t > t_min && t < t_max {
            let mut u: f64 = 0.0;
            let mut v: f64 = 0.0;
            let mut normal = (r.at(t) - self.center) / self.radius;
            Sphere::get_sphere_uv(&normal, &mut u, &mut v);
            let front_face = (r.dire.dot(&normal.clone()) < 0.0); //todo set front_face 在这里
            if !front_face {
                normal = normal * -1.0;
            }
            return Some(HitRecordstatic {
                t,
                p: r.at(t),
                normal,
                mat_ptr: &self.mat_ptr,
                u,
                v,
                front_face,
            });
        }
        let t: f64 = (-half_b + root) / a;
        if t > t_min && t < t_max {
            let mut u: f64 = 0.0;
            let mut v: f64 = 0.0;
            let mut normal = (r.at(t) - self.center) / self.radius;
            Sphere::get_sphere_uv(&normal, &mut u, &mut v);
            let front_face = (r.dire.dot(&normal.clone()) < 0.0);
            if !front_face {
                normal = normal * -1.0;
            }
            return Some(HitRecordstatic {
                t,
                p: r.at(t),
                normal,
                mat_ptr: &self.mat_ptr,
                u,
                v,
                front_face,
            });
        }
        return None;
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        output_box.minimum = self
            .center
            .sub(Vec3::new(self.radius, self.radius, self.radius));
        output_box.maximum = self
            .center
            .add(Vec3::new(self.radius, self.radius, self.radius));
        true
    }

    fn pdf_value(&self, o: &Vec3, v: &Vec3) -> f64 {
        let pig = Lambertianstatic::<SolidColorstatic>::new(&Vec3::zero());
        let rec = self.hit(&Raystatic::new2(o, v, 0.0), 0.001, INFINITY);
        if rec.is_none() {
            return 0.0;
        }
        let cos_max = (1.0
            - self.radius * self.radius / (self.center.sub(o.clone()).squared_length()))
        .sqrt();
        let solid_angle = (PI as f64) * 2.0 * (1.0 - cos_max);
        1.0 / solid_angle
    }

    fn random(&self, o: &Vec3) -> Vec3 {
        let dire = self.center.sub(o.clone());
        let dist_square = dire.squared_length();
        let mut uvw = Onb::new_zero();
        uvw.build_from_w(&dire);
        uvw.local_2(&Vec3::random_to_sphere(self.radius, dist_square))
    }
}

impl Ray {
    pub fn ray_color(
        &self,
        background: &Vec3,
        world: &dyn Hittable,
        lights: &Arc<dyn Hittable>,
        depth: i32,
    ) -> Vec3 {
        let mut rec: HitRecord = HitRecord::new_blank();
        if depth <= 0 {
            return Vec3::zero();
        }
        let inf = INFINITY;
        if world.hit(&self, 0.001, inf, &mut rec) {
            let mut scattered = Ray::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
            let mut albedo = Vec3::zero();
            // let emitted = rec.mat_ptr.emitted(rec.u, rec.v, &rec.p);
            let emitted = rec.mat_ptr.emitted(&self, &rec, rec.u, rec.v, &rec.p);
            let mut pdf = 0.0;
            let mut srec = ScatterRecord::new_zero();
            //let albedo = Vec3::zero();

            if rec.mat_ptr.scatter(&self, &rec, &mut srec) {
                if srec.is_specular {
                    return srec.attenuation
                        * srec
                            .specular_ray
                            .ray_color(background, world, lights, depth - 1);
                }
                let light_ptr = Arc::new(HittablePdf::new(lights.clone(), &rec.p));
                let mixed_pdf = MixturePdf::new(light_ptr.clone(), srec.pdf_ptr.clone());

                let pig = Ray::new2(&rec.p, &mixed_pdf.generate(), self.time);
                scattered.time = pig.time;
                scattered.dire = pig.dire.clone();
                scattered.orig = pig.orig.clone();
                //pdf_val = light_pdf.value(scattered.direction());
                let pdf_val = mixed_pdf.value(&scattered.dire);

                return emitted.add(
                    srec.attenuation
                        .mul((rec.mat_ptr.scattering_pdf(&self, &rec, &mut scattered)))
                        .mul(scattered.ray_color(&background, world, lights, depth - 1))
                        / (pdf_val),
                );
            }
            return emitted;
        }

        background.clone()
    }
}

pub struct Raystatic {
    pub orig: Vec3,
    pub dire: Vec3,
    pub time: f64,
}

impl Raystatic {
    pub fn new(x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64, t: f64) -> Self {
        Self {
            orig: Vec3 {
                x: x1,
                y: y1,
                z: z1,
            },
            dire: Vec3 {
                x: x2,
                y: y2,
                z: z2,
            },
            time: t,
        }
    }

    pub fn new2(orig_: &Vec3, dire_: &Vec3, t: f64) -> Self {
        Self {
            orig: Vec3 {
                x: orig_.x,
                y: orig_.y,
                z: orig_.z,
            },
            dire: Vec3 {
                x: dire_.x,
                y: dire_.y,
                z: dire_.z,
            },
            time: t,
        }
    }

    pub fn get_orig(&self) -> &Vec3 {
        &self.orig
    }

    pub fn get_dire(&self) -> &Vec3 {
        &self.dire
    }

    pub fn at(&self, t: f64) -> Vec3 {
        Vec3 {
            x: self.orig.x + self.dire.x * t,
            y: self.orig.y + self.dire.y * t,
            z: self.orig.z + self.dire.z * t,
        }
    }
}

pub fn ray_color<T: Hittablestatic>(
    r: &Raystatic,
    background: &Vec3,
    world: &dyn Hittablestatic,
    lights: &dyn Hittablestatic,
    depth: i32,
) -> Vec3 {
    //let mut rec: HitRecord = HitRecord::new_blank();
    if depth <= 0 {
        return Vec3::zero();
    }
    let inf = INFINITY;
    let mut rec = world.hit(r, 0.001, inf);
    if rec.is_some() {
        let mut rec = rec.unwrap();
        let mut scattered = Raystatic::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let mut albedo = Vec3::zero();
        let emitted = rec.mat_ptr.emitted(r, &rec, rec.u, rec.v, &rec.p);
        let mut pdf = 0.0;
        //let mut srec = ScatterRecordstatic::new_zero();
        let pdf_ptr_ = &CosinePdf::new_zero();
        let mut srec = ScatterRecordstatic {
            specular_ray: Raystatic::new2(&Vec3::zero(), &Vec3::zero(), 0.0),
            is_specular: false,
            attenuation: Vec3::zero(),
            pdf_ptr: &pdf_ptr_,
        };
        //let albedo = Vec3::zero();

        if rec.mat_ptr.scatter(r, &rec, &mut srec) {
            if srec.is_specular {
                return srec.attenuation
                    * ray_color(&srec.specular_ray, background, world, lights, depth - 1);
            }
            let light_ptr = (HittablePdfstatic::new(lights, &rec.p));
            // let light_
            let mixed_pdf = MixturePdfstatic::new(&light_ptr, srec.pdf_ptr);

            let pig = Ray::new2(&rec.p, &mixed_pdf.generate(), r.time);
            scattered.time = pig.time;
            scattered.dire = pig.dire.clone();
            scattered.orig = pig.orig.clone();
            //pdf_val = light_pdf.value(scattered.direction());
            let pdf_val = mixed_pdf.value(&scattered.dire);
            return emitted.add(
                srec.attenuation
                    .mul((rec.mat_ptr.scattering_pdf(r, &rec, &mut scattered)))
                    .mul(ray_color(&scattered, &background, world, lights, depth - 1))
                    / (pdf_val),
            );
        }
        return emitted;
    }

    background.clone()
}

pub struct Translate {
    pub ptr: Arc<dyn Hittable>,
    pub offset: Vec3,
}

impl Translate {
    pub fn new(p: Arc<dyn Hittable>, off: Vec3) -> Translate {
        Translate {
            ptr: p,
            offset: off,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let moved_r = Ray::new2(
            &(r.orig.sub(self.offset.clone())),
            &(r.dire.clone()),
            r.time,
        );
        if !self.ptr.hit(&moved_r, t_min, t_max, rec) {
            return false;
        }
        rec.p = rec.p.add(self.offset.clone());
        rec.set_face_normal(&moved_r, &rec.normal.clone());
        true
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        if !self.ptr.bounding_box(time0, time1, output_box) {
            return false;
        }
        output_box.minimum = output_box.minimum.add(self.offset.clone());
        output_box.maximum = output_box.maximum.add(self.offset.clone());
        true
    }

    fn pdf_value(&self, o: &Vec3, v: &Vec3) -> f64 {
        0.0
    }

    fn random(&self, o: &Vec3) -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
    }
}

pub struct RotateY {
    pub ptr: Arc<dyn Hittable>,
    pub sin_theta: f64,
    pub cos_theta: f64,
    pub has_hezi: bool,
    pub bbox: Aabb,
}

impl RotateY {
    pub fn new(p: Arc<dyn Hittable>, angle: f64) -> RotateY {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let ptr: Arc<dyn Hittable> = p;
        let mut bbox: Aabb = Aabb::new_zero();
        let has_hezi = ptr.bounding_box(0.0, 1.0, &mut bbox);
        let mut min_ = Vec3::new(INFINITY, INFINITY, INFINITY);
        let mut max_ = Vec3::new(-INFINITY, -INFINITY, -INFINITY);
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = (i as f64) * bbox.maximum.x + (1.0 - i as f64) * bbox.minimum.x;
                    let y = (j as f64) * bbox.maximum.y + (1.0 - j as f64) * bbox.minimum.y;
                    let z = (k as f64) * bbox.maximum.z + (1.0 - k as f64) * bbox.minimum.z;
                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;
                    let tester = Vec3::new(newx, y, newz);
                    for c in 0..3 as usize {
                        if c == 0 {
                            min_.x = f_min(min_.x, tester.x);
                            max_.x = f_max(max_.x, tester.x);
                        }
                        if c == 1 {
                            min_.y = f_min(min_.y, tester.y);
                            max_.y = f_max(max_.y, tester.y);
                        }
                        if c == 2 {
                            min_.z = f_min(min_.z, tester.z);
                            max_.z = f_max(max_.z, tester.z);
                        }
                    }
                }
            }
        }
        bbox.minimum = min_;
        bbox.maximum = max_;
        RotateY {
            ptr,
            sin_theta,
            cos_theta,
            has_hezi,
            bbox,
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut orig = r.orig.clone();
        let mut dire = r.dire.clone();
        orig.x = self.cos_theta * r.orig.x - self.sin_theta * r.orig.z;
        orig.z = self.sin_theta * r.orig.x + self.cos_theta * r.orig.z;

        dire.x = self.cos_theta * r.dire.x - self.sin_theta * r.dire.z;
        dire.z = self.sin_theta * r.dire.x + self.cos_theta * r.dire.z;

        let rotated_r = Ray::new2(&orig, &dire, r.time);
        if !self.ptr.hit(&rotated_r, t_min, t_max, rec) {
            return false;
        }

        let mut p = rec.p.clone();
        let mut normal = rec.normal.clone();

        p.x = self.cos_theta * rec.p.x + self.sin_theta * rec.p.z;
        p.z = -self.sin_theta * rec.p.x + self.cos_theta * rec.p.z;

        normal.x = self.cos_theta * rec.normal.x + self.sin_theta * rec.normal.z;
        normal.z = -self.sin_theta * rec.normal.x + self.cos_theta * rec.normal.z;

        rec.p = p;
        rec.set_face_normal(&rotated_r, &normal);
        true
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        output_box.minimum = self.bbox.minimum.clone();
        output_box.maximum = self.bbox.maximum.clone();
        self.has_hezi
    }

    fn pdf_value(&self, o: &Vec3, v: &Vec3) -> f64 {
        0.0
    }

    fn random(&self, o: &Vec3) -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
    }
}

pub struct FlipFace {
    pub ptr: Arc<dyn Hittable>,
}

impl FlipFace {
    pub fn new_zero() -> FlipFace {
        FlipFace {
            ptr: Arc::new(Sphere::new_zero()),
        }
    }

    pub fn new(p: Arc<dyn Hittable>) -> FlipFace {
        FlipFace { ptr: p.clone() }
    }
}

impl Hittable for FlipFace {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        if !self.ptr.hit(r, t_min, t_max, rec) {
            return false;
        }
        rec.front_face = !rec.front_face;
        true
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        self.ptr.bounding_box(time0, time1, output_box)
    }

    fn pdf_value(&self, o: &Vec3, v: &Vec3) -> f64 {
        0.0
    }

    fn random(&self, o: &Vec3) -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
    }
}
