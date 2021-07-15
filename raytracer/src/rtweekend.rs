use crate::matirial::{Dielectric, HittableList, Lambertian, Material, Metal};
use crate::ray::Sphere;
use crate::Vec3;
use rand::Rng;
use std::f64::consts::PI;
use std::ops::{Mul, Sub};
use std::sync::Arc;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_double_0_1() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0..1.0)
}

pub fn random_double_a_b(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    };
    if x > max {
        return max;
    };
    x
}

pub fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * ((1.0 - cosine).powf(5.0))
}

pub fn random_secne() -> HittableList {
    let mut world = HittableList { objects: vec![] };
    let ground_material = Arc::new(Lambertian::new(&Vec3::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        mat_ptr: ground_material,
    }));
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double_0_1();
            let center_ = Vec3::new(
                (a as f64) + 0.9 * random_double_0_1(),
                0.2,
                (b as f64) + 0.9 * random_double_0_1(),
            );

            if center_.sub(Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let mut sphere_material: Arc<dyn Material>;
                if choose_mat < 0.8 {
                    //diffuse
                    let albedo = Vec3::random_v_0_1().mul(Vec3::random_v_0_1());
                    sphere_material = Arc::new(Lambertian::new(&albedo));
                    world.add(Arc::new(Sphere {
                        center: center_,
                        radius: 0.2,
                        mat_ptr: sphere_material,
                    }));
                } else {
                    if choose_mat < 0.95 {
                        //metal
                        let albedo = Vec3::random_v_a_b(0.5, 1.0);
                        let fuzz = random_double_a_b(0.0, 0.5);
                        sphere_material = Arc::new(Metal::new(&albedo, fuzz));
                        world.add(Arc::new(Sphere {
                            center: center_,
                            radius: 0.2,
                            mat_ptr: sphere_material,
                        }));
                    } else {
                        //glass
                        sphere_material = Arc::new(Dielectric::new(1.5));
                        world.add(Arc::new(Sphere {
                            center: center_,
                            radius: 0.2,
                            mat_ptr: sphere_material,
                        }));
                    }
                }
            }
        }
    }
    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere {
        center: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        mat_ptr: material1,
    }));

    let material2 = Arc::new(Lambertian::new(&Vec3::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere {
        center: Vec3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        mat_ptr: material2,
    }));

    let material3 = Arc::new(Metal::new(&Vec3::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere {
        center: Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        mat_ptr: material3,
    }));
    world
}
