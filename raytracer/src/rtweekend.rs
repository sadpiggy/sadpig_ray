use crate::aarect_h::{XyRect, XzRect, YzRect};
use crate::bvh::BvhNode;
use crate::constant_medium::ConstantMedium;
use crate::matirial::{Dielectric, DiffuseLight, HittableList, Lambertian, Material, Metal};
use crate::moving_sphere::MovingSphere;
use crate::texture::{CheckerTexture, ImageTexture, NoiseTexture};
use crate::Vec3;
use crate::BOX_H::Hezi;
use crate::RAY::{Hittable, RotateY, Sphere, Translate};
use rand::Rng;
use std::collections::hash_map::Entry::Vacant;
use std::f64::consts::PI;
use std::ops::{Add, Mul, Sub};
use std::sync::atomic::Ordering::AcqRel;
use std::sync::Arc;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn f_min(v1: f64, v2: f64) -> f64 {
    if v1 < v2 {
        return v1;
    }
    v2
}

pub fn f_max(v1: f64, v2: f64) -> f64 {
    if v1 > v2 {
        return v1;
    }
    v2
}

pub fn random_double_0_1() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0..1.0)
}

pub fn random_double_a_b(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

pub fn random_int_a_b(min: i32, max: i32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..(max))
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

    let checker = Arc::new(CheckerTexture::new2(
        Vec3::new(0.2, 0.3, 0.1),
        Vec3::new(0.9, 0.9, 0.9),
    ));
    world.add(Arc::new(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        mat_ptr: Arc::new(Lambertian::new1(checker)),
    }));

    // let ground_material = Arc::new(Lambertian::new(&Vec3::new(0.5, 0.5, 0.5)));
    // world.add(Arc::new(Sphere {
    //     center: Vec3::new(0.0, -1000.0, 0.0),
    //     radius: 1000.0,
    //     mat_ptr: ground_material,
    // }));
    //
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
                    // auto center2 = center + vec3(0, random_double(0,.5), 0);
                    // world.add(make_shared<moving_sphere>(
                    //     center, center2, 0.0, 1.0, 0.2, sphere_material));
                    let center2 = center_.add(Vec3::new(0.0, random_double_a_b(0.0, 0.5), 0.0));
                    world.add(Arc::new(MovingSphere {
                        center0: center_,
                        center1: center2,
                        time0: 0.0,
                        time1: 1.0,
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

pub fn two_spheres() -> HittableList {
    let mut objects: HittableList = HittableList::new_zero();
    let checker = Arc::new(CheckerTexture::new2(
        Vec3::new(0.2, 0.3, 0.1),
        Vec3::new(0.9, 0.9, 0.9),
    ));

    //let pertext = Arc::new(NoiseTexture::new());

    objects.add(Arc::new(Sphere {
        center: Vec3::new(0.0, -10.0, 0.0),
        radius: 10.0,
        mat_ptr: Arc::new(Lambertian::new1(checker.clone())),
    }));

    objects.add(Arc::new(Sphere {
        center: Vec3::new(0.0, 10.0, 0.0),
        radius: 10.0,
        mat_ptr: Arc::new(Lambertian::new1(checker.clone())),
    }));

    objects
}

pub fn two_perlin_spheres() -> HittableList {
    let mut objects: HittableList = HittableList::new_zero();

    let pertext = Arc::new(NoiseTexture::new(4.0));

    objects.add(Arc::new(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        mat_ptr: Arc::new(Lambertian::new1(pertext.clone())),
    }));

    objects.add(Arc::new(Sphere {
        center: Vec3::new(0.0, 2.0, 0.0),
        radius: 2.0,
        mat_ptr: Arc::new(Lambertian::new1(pertext.clone())),
    }));

    objects
}

pub fn earth() -> HittableList {
    let mut objects: HittableList = HittableList::new_zero();
    let earth_textrue = Arc::new(ImageTexture::new("input/sanhuo.png"));
    objects.add(Arc::new(Sphere {
        center: Vec3::new(0.0, 0.0, 0.0),
        radius: 2.0,
        mat_ptr: Arc::new(Lambertian::new1(earth_textrue)),
    }));
    objects
}

pub fn simple_light() -> HittableList {
    let mut objects: HittableList = HittableList::new_zero();
    let pertext = Arc::new(NoiseTexture::new(4.0));
    objects.add(Arc::new(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        mat_ptr: Arc::new(Lambertian::new1(pertext.clone())),
    }));
    objects.add(Arc::new(Sphere {
        center: Vec3::new(0.0, 2.0, 0.0),
        radius: 2.0,
        mat_ptr: Arc::new(Lambertian::new1(pertext)),
    }));
    let difflight = Arc::new(DiffuseLight::new2(Vec3::new(4.0, 4.0, 4.0)));
    objects.add(Arc::new(XyRect::new(3.0, 5.0, 1.0, 3.0, -2.0, difflight)));
    objects
}

pub fn cornell_box() -> HittableList {
    let mut objects: HittableList = HittableList::new_zero();
    let red = Arc::new(Lambertian::new(&(Vec3::new(0.65, 0.05, 0.05))));
    let white = Arc::new(Lambertian::new(&(Vec3::new(0.73, 0.73, 0.73))));
    let green = Arc::new(Lambertian::new(&(Vec3::new(0.12, 0.45, 0.15))));
    let light = Arc::new(DiffuseLight::new2(Vec3::new(15.0, 15.0, 15.0)));
    objects.add(Arc::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    objects.add(Arc::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    objects.add(Arc::new(XzRect::new(
        213.0, 343.0, 227.0, 332.0, 554.0, light,
    )));
    objects.add(Arc::new(XzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    objects.add(Arc::new(XzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    objects.add(Arc::new(XyRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));

    let mut box1: Arc<dyn Hittable> = Arc::new(Hezi::new(
        Vec3::zero(),
        Vec3::new(165.0, 330.0, 165.0),
        white.clone(),
    ));
    box1 = Arc::new(RotateY::new(box1, 15.0));
    box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    objects.add(box1);

    let mut box2: Arc<dyn Hittable> = Arc::new(Hezi::new(
        Vec3::zero(),
        Vec3::new(165.0, 165.0, 165.0),
        white.clone(),
    ));
    box2 = Arc::new(RotateY::new(box2, -18.0));
    box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
    objects.add(box2);

    objects
}

pub fn cornell_smoke() -> HittableList {
    let mut objects: HittableList = HittableList::new_zero();
    let red = Arc::new(Lambertian::new(&(Vec3::new(0.65, 0.05, 0.05))));
    let white = Arc::new(Lambertian::new(&(Vec3::new(0.73, 0.73, 0.73))));
    let green = Arc::new(Lambertian::new(&(Vec3::new(0.12, 0.45, 0.15))));
    let light = Arc::new(DiffuseLight::new2(Vec3::new(15.0, 15.0, 15.0)));
    objects.add(Arc::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    objects.add(Arc::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    objects.add(Arc::new(XzRect::new(
        213.0, 343.0, 227.0, 332.0, 554.0, light,
    )));
    objects.add(Arc::new(XzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    objects.add(Arc::new(XzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    objects.add(Arc::new(XyRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));

    let mut box1: Arc<dyn Hittable> = Arc::new(Hezi::new(
        Vec3::zero(),
        Vec3::new(165.0, 330.0, 165.0),
        white.clone(),
    ));
    box1 = Arc::new(RotateY::new(box1, 15.0));
    box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));

    let mut box2: Arc<dyn Hittable> = Arc::new(Hezi::new(
        Vec3::zero(),
        Vec3::new(165.0, 165.0, 165.0),
        white.clone(),
    ));
    box2 = Arc::new(RotateY::new(box2, -18.0));
    box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));

    objects.add(Arc::new(ConstantMedium::new2(
        box1,
        0.01,
        Vec3::new(0.0, 0.0, 0.0),
    )));
    objects.add(Arc::new(ConstantMedium::new2(
        box2,
        0.01,
        Vec3::new(1.0, 1.0, 1.0),
    )));

    objects
}

pub fn final_scene() -> HittableList {
    let white = Arc::new(Lambertian::new(&Vec3::new(0.73, 0.73, 0.73)));
    let mut boxes1 = HittableList::new_zero();
    let mut objects = HittableList::new_zero();
    let ground = Arc::new(Lambertian::new(&Vec3::new(0.48, 0.83, 0.53)));
    let green = Arc::new(Lambertian::new(&(Vec3::new(0.12, 0.45, 0.15))));
    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + (i as f64) * w;
            let z0 = -1000.0 + (j as f64) * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_double_a_b(1.0, 101.0);
            let z1 = z0 + w;

            boxes1.add(Arc::new(Hezi::new(
                Vec3::new(x0, y0, z0),
                Vec3::new(x1, y1, z1),
                ground.clone(),
            )));
        }
    }

    objects.add(Arc::new(BvhNode::new_dog(&boxes1, 0.0, 1.0)));

    let light = Arc::new(DiffuseLight::new2(Vec3::new(7.0, 7.0, 7.0)));
    objects.add(Arc::new(XzRect::new(
        123.0, 423.0, 147.0, 412.0, 554.0, light,
    )));
    //return objects;

    let center1 = Vec3::new(400.0, 400.0, 200.0);
    let center2 = center1.add(Vec3::new(30.0, 0.0, 0.0));
    let moving_sphere_material = Arc::new(Lambertian::new(&Vec3::new(0.7, 0.3, 0.1)));
    objects.add(Arc::new(MovingSphere::new(
        center1,
        center2,
        0.0,
        1.0,
        50.0,
        moving_sphere_material,
    )));

    objects.add(Arc::new(Sphere {
        center: Vec3::new(260.0, 150.0, 45.0),
        radius: 50.0,
        mat_ptr: Arc::new(Dielectric::new(1.5)),
    }));
    objects.add(Arc::new(Sphere {
        center: Vec3::new(0.0, 150.0, 145.0),
        radius: 50.0,
        mat_ptr: Arc::new(Metal::new(&Vec3::new(0.8, 0.8, 0.9), 1.0)),
    }));

    let mut boundary = Arc::new(Sphere {
        center: Vec3::new(360.0, 150.0, 145.0),
        radius: 70.0,
        mat_ptr: Arc::new(Dielectric::new(1.5)),
    });
    objects.add(boundary.clone());
    objects.add(Arc::new(ConstantMedium::new2(
        boundary.clone(),
        0.2,
        Vec3::new(0.2, 0.4, 0.9),
    )));
    boundary = Arc::new(Sphere {
        center: Vec3::new(0.0, 0.0, 0.0),
        radius: 5000.0,
        mat_ptr: Arc::new(Dielectric::new(1.5)),
    });
    objects.add(Arc::new(ConstantMedium::new2(
        boundary.clone(),
        0.0001,
        Vec3::new(1.0, 1.0, 1.0),
    )));

    let emat = Arc::new(Lambertian::new1(Arc::new(ImageTexture::new(
        "input/me.png",
    ))));
    let pertext = Arc::new(NoiseTexture::new(0.1));
    // objects.add(Arc::new(Sphere {
    //     center: Vec3::new(400.0, 200.0, 400.0),
    //     radius: 100.0,
    //     mat_ptr: emat,
    // }));
    objects.add(Arc::new(Sphere {
        center: Vec3::new(400.0, 200.0, 400.0),
        radius: 100.0,
        mat_ptr: Arc::new(Lambertian::new1(pertext.clone())),
    }));

    objects.add(Arc::new(Sphere {
        center: Vec3::new(220.0, 280.0, 300.0),
        radius: 80.0,
        mat_ptr: Arc::new(Lambertian::new1(pertext)),
    }));

    let mut boxes2 = HittableList::new_zero();
    let white = Arc::new(Lambertian::new(&Vec3::new(0.73, 0.73, 0.73)));
    let ns = 1000;
    for j in 0..ns {
        boxes2.add(Arc::new(Sphere {
            center: Vec3::random_v_a_b(0.0, 165.0),
            radius: 10.0,
            mat_ptr: white.clone(),
        }))
    }

    let rinima = Arc::new(BvhNode::new_dog(&boxes2, 0.0, 1.0));

    objects.add(Arc::new(Translate::new(
        Arc::new(RotateY::new(rinima, 15.0)),
        Vec3::new(-100.0, 270.0, 395.0),
    )));

    objects
}
