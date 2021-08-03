use crate::camera::{random_double_0_1, random_double_a_b, Camera};
use crate::hittable_list::{HittableList, HittableListstatic};
use crate::matirial::{Dielectric, Lambertian, Lambertianstatic, Material, Metal};
use crate::rtweekend::{
    clamp, cornell_box, cornell_box_static, cornell_smoke, cornell_table_static, dinosaur_static,
    earth, final_scene, final_scene_static, get_obj, get_obj_test, random_secne, simple_light,
    two_perlin_spheres, two_spheres, two_spheres_static,
};
use crate::RAY::{Hittable, Hittablestatic, Sphere, Spherestatic};
use core::fmt::Alignment::Center;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
//pub use ray::Ray;
use crate::aabb::Aabb;
use crate::aarect_h::{XzRect, XzRectstatic};
use crate::texture::SolidColorstatic;
use crate::Vec3;
use image::imageops::FilterType::Lanczos3;
use std::f64::consts::PI;
use std::ops::{Add, AddAssign, Div, Mul, Sub};
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::vec::Vec;
use threadpool::ThreadPool;

pub fn Run() {
    let mut aspect_ratio_ = 3.0 / 2.0;
    let mut image_width: u32 = 1200;
    let mut image_height: u32 = (((image_width) as f64) / aspect_ratio_) as u32;
    //渲染质量
    let mut samples_per_pixels: u32 = 20;
    let max_depth = 10;
    //world
    let R = (PI / 4.0).cos();

    let mut world: HittableList = HittableList::new_zero(); // HittableList { objects: vec![] };

    let mut vfov_ = 40.0;
    let mut aperture_ = 0.0;
    let mut look_from_: Vec3 = Vec3::zero(); // = (Vec3::new(12.0, 2.0, 3.0));
    let mut look_at_: Vec3 = Vec3::zero(); // = (Vec3::new(0.0, 0.0, 0.0));
    let mut background = Vec3::zero();

    let mut case = 7;
    if case == 0 {
        world = random_secne();
        background = Vec3::new(0.7, 0.8, 1.0);
        look_from_ = Vec3::new(13.0, 2.0, 3.0);
        look_at_ = Vec3::new(0.0, 0.0, 0.0);
        vfov_ = 20.0;
        aperture_ = 0.1;
    }
    if case == 1 {
        world = two_spheres();
        background = Vec3::new(0.7, 0.8, 1.0);
        look_from_ = Vec3::new(13.0, 2.0, 3.0);
        look_at_ = Vec3::new(0.0, 0.0, 0.0);
        vfov_ = 20.0;
        aperture_ = 0.1;
    }
    if case == 2 {
        world = two_perlin_spheres();
        background = Vec3::new(0.7, 0.8, 1.0);
        look_from_ = Vec3::new(13.0, 2.0, 3.0);
        look_at_ = Vec3::new(0.0, 0.0, 0.0);
        vfov_ = 20.0;
        aperture_ = 0.1;
    }
    if case == 3 {
        world = earth();
        background = Vec3::new(0.7, 0.8, 1.0);
        look_from_ = Vec3::new(13.0, 2.0, 3.0);
        look_at_ = Vec3::new(0.0, 0.0, 0.0);
        vfov_ = 20.0;
        aperture_ = 0.1;
    }
    if case == 4 {
        world = simple_light();
        background = Vec3::new(0.0, 0.0, 0.0);
        look_from_ = Vec3::new(26.0, 3.0, 6.0);
        look_at_ = Vec3::new(0.0, 2.0, 0.0);
        vfov_ = 20.0;
    }
    if case == 5 {
        world = cornell_box();
        background = Vec3::new(0.0, 0.0, 0.0);
        look_from_ = Vec3::new(278.0, 278.0, -800.0);
        look_at_ = Vec3::new(278.0, 278.0, 0.0);
        vfov_ = 40.0;
        aspect_ratio_ = 1.0;
        image_width = 600;
        image_height = image_width;
    }
    if case == 6 {
        world = cornell_smoke();
        background = Vec3::new(0.0, 0.0, 0.0);
        look_from_ = Vec3::new(278.0, 278.0, -800.0);
        look_at_ = Vec3::new(278.0, 278.0, 0.0);
        vfov_ = 40.0;
        aspect_ratio_ = 1.0;
        image_width = 600;
        image_height = image_width;
    }
    if case == 7 {
        world = final_scene();
        background = Vec3::new(0.0, 0.0, 0.0);
        look_from_ = Vec3::new(478.0, 278.0, -600.0);
        look_at_ = Vec3::new(278.0, 278.0, 0.0);
        vfov_ = 40.0;
        aspect_ratio_ = 1.0;
        image_width = 800;
        image_height = image_width;
    }
    // if case ==8 {
    //     world = two_spheres_static();
    //     background = Vec3::new(0.7, 0.8, 1.0);
    //     look_from_ = Vec3::new(13.0, 2.0, 3.0);
    //     look_at_ = Vec3::new(0.0, 0.0, 0.0);
    //     vfov_ = 20.0;
    //     aperture_ = 0.1;
    // }

    //camera
    let cam = Camera::new(
        &look_from_,
        &look_at_,
        &(Vec3::new(0.0, 1.0, 0.0)),
        vfov_,
        aspect_ratio_,
        aperture_,
        10.0,
        0.0,
        1.0,
    );

    let (tx, rx) = channel();
    let n_jobs = 32;
    let n_workers = 6;
    let pool = ThreadPool::new(n_workers);

    let mut results: RgbImage = ImageBuffer::new(image_width as u32, image_height as u32);
    let bar = ProgressBar::new(n_jobs as u64);

    //多线程渲染
    for i in 0..n_jobs {
        let tx = tx.clone();
        let world_ = world.clone();
        //let lights_ptr = lights.clone();
        pool.execute(move || {
            // let mut motherfuck = HittableList::new_zero();
            // motherfuck.add(Arc::new(XzRect::new(
            //     213.0,
            //     343.0,
            //     227.0,
            //     332.0,
            //     554.0,
            //     Arc::new(Lambertian::new_zero()),
            // )));
            // motherfuck.add(Arc::new(Sphere {
            //     center: Vec3::new(190.0, 90.0, 190.0),
            //     radius: 90.0,
            //     mat_ptr: Arc::new(Lambertian::new_zero()),
            // }));
            let lights: Arc<dyn Hittable> = Arc::new(Sphere {
                center: Vec3::new(190.0, 90.0, 190.0),
                radius: 90.0,
                mat_ptr: Arc::new(Lambertian::new_zero()),
            });

            let row_begin = image_height as usize * i as usize / n_jobs;
            let row_end = image_height as usize * (i as usize + 1) / n_jobs;
            let render_height = row_end - row_begin;
            let mut img: RgbImage = ImageBuffer::new(image_width as u32, render_height as u32);
            for x in 0..image_width {
                for (img_y, y) in (row_begin..row_end).enumerate() {
                    let y = y as u32;
                    let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
                    for _s in 0..samples_per_pixels {
                        let u: f64 =
                            ((x) as f64 + random_double_0_1()) / ((image_width - 1) as f64);
                        let v: f64 = ((image_height - y) as f64 + random_double_0_1())
                            / ((image_height - 1) as f64);
                        let r = cam.get_ray(u, v);
                        pixel_color.add_assign(r.ray_color(
                            &background,
                            &world_,
                            &lights,
                            max_depth,
                        ));
                    }
                    let pixel = img.get_pixel_mut(x as u32, img_y as u32);

                    *pixel = image::Rgb([
                        pixel_color.get_u8_x(samples_per_pixels),
                        pixel_color.get_u8_y(samples_per_pixels),
                        pixel_color.get_u8_z(samples_per_pixels),
                    ]);
                }
            }
            tx.send((row_begin..row_end, img))
                .expect("failed to send result");
        });
    }

    //将图片写入
    for (rows, data) in rx.iter().take(n_jobs) {
        for (idx, row) in rows.enumerate() {
            for col in 0..image_width {
                let row = row as u32;
                let idx = idx as u32;
                *results.get_pixel_mut(col as u32, row) = *data.get_pixel(col as u32, idx);
            }
        }
        bar.inc(1);
    }

    results.save("output/test.png").unwrap();
    bar.finish();
}

pub fn Runstatic() {
    let mut aspect_ratio_ = 3.0 / 2.0;
    let mut image_width: u32 = 1200;
    let mut image_height: u32 = (((image_width) as f64) / aspect_ratio_) as u32;
    //渲染质量
    let mut samples_per_pixels: u32 = 5;
    let max_depth = 10;
    //world
    let R = (PI / 4.0).cos();

    let mut world: HittableListstatic = HittableListstatic::new_zero(); // HittableList { objects: vec![] };

    let mut vfov_ = 40.0;
    let mut aperture_ = 0.0;
    let mut look_from_: Vec3 = Vec3::zero(); // = (Vec3::new(12.0, 2.0, 3.0));
    let mut look_at_: Vec3 = Vec3::zero(); // = (Vec3::new(0.0, 0.0, 0.0));
    let mut background = Vec3::new(0.93, 0.93, 0.93);

    let mut case = 4;

    if case == 0 {
        world = two_spheres_static();
        background = Vec3::new(0.7, 0.8, 1.0);
        look_from_ = Vec3::new(13.0, 2.0, 3.0);
        look_at_ = Vec3::new(0.0, 0.0, 0.0);
        vfov_ = 20.0;
        aperture_ = 0.1;
    }
    if case == 1 {
        world = cornell_box_static();
        background = Vec3::new(0.0, 0.0, 0.0);
        look_from_ = Vec3::new(278.0, 278.0, -800.0);
        look_at_ = Vec3::new(278.0, 278.0, 0.0);
        vfov_ = 40.0;
        aspect_ratio_ = 1.0;
        image_width = 600;
        image_height = image_width;
    }
    if case == 2 {
        world = final_scene_static();
        background = Vec3::new(0.0, 0.0, 0.0);
        look_from_ = Vec3::new(478.0, 278.0, -600.0);
        look_at_ = Vec3::new(278.0, 278.0, 0.0);
        vfov_ = 40.0;
        aspect_ratio_ = 1.0;
        image_width = 800;
        image_height = image_width;
    }

    if case == 3 {
        world = get_obj_test();
        background = Vec3::new(0.7, 0.8, 1.0);
        look_from_ = Vec3::new(13.0, 2.0, 3.0);
        look_at_ = Vec3::new(0.0, 0.0, 0.0);
        vfov_ = 20.0;
        aperture_ = 0.1;
    }

    if case == 4 {
        world = cornell_table_static();
        // world = cornell_box_static();
        background = Vec3::new(0.0, 0.0, 0.0);
        look_from_ = Vec3::new(278.0, 278.0, -800.0);
        look_at_ = Vec3::new(278.0, 278.0, 0.0);
        vfov_ = 40.0;
        aspect_ratio_ = 1.0;
        image_width = 600;
        image_height = image_width;
    }
    //-5.28056 19.8212 -36.1185

    if case == 5 {
        world = dinosaur_static();
        background = Vec3::new(0.7, 0.8, 1.0);
        look_from_ = Vec3::new(278.0, 278.0, -800.0);
        look_at_ = Vec3::new(-5.28056, 19.8212, -36.1185);
        vfov_ = 40.0;
        aspect_ratio_ = 1.0;
        image_width = 600;
        image_height = image_width;
    }

    //camera
    let cam = Camera::new(
        &look_from_,
        &look_at_,
        &(Vec3::new(0.0, 1.0, 0.0)),
        vfov_,
        aspect_ratio_,
        aperture_,
        10.0,
        0.0,
        1.0,
    );

    let (tx, rx) = channel();
    let n_jobs = 32;
    let n_workers = 6;
    let pool = ThreadPool::new(n_workers);

    let mut results: RgbImage = ImageBuffer::new(image_width as u32, image_height as u32);
    let bar = ProgressBar::new(n_jobs as u64);

    //多线程渲染
    for i in 0..n_jobs {
        let tx = tx.clone();
        let world_ = world.clone();
        //let lights_ptr = lights.clone();
        pool.execute(move || {
            let mut motherfuck = HittableListstatic::new_zero();
            motherfuck.add(Arc::new(
                XzRectstatic::<Lambertianstatic<SolidColorstatic>>::new(
                    213.0,
                    343.0,
                    227.0,
                    332.0,
                    554.0,
                    (Lambertianstatic::<SolidColorstatic>::new_zero()),
                ),
            ));
            // motherfuck.add(Arc::new(Spherestatic {
            //     center: Vec3::new(0.0, 0.0, -10.1185),
            //     radius: 90.0,
            //     mat_ptr: (Lambertianstatic::<SolidColorstatic>::new_zero()),
            // }));
            motherfuck.add(Arc::new(Spherestatic {
                center: Vec3::new(190.0, 90.0, 190.0),
                radius: 90.0,
                mat_ptr: (Lambertianstatic::<SolidColorstatic>::new_zero()),
            }));
            let lights = (motherfuck);
            // let lights = (Spherestatic {
            //     center: Vec3::new(190.0, 90.0, 190.0),
            //     radius: 90.0,
            //     mat_ptr: (Lambertianstatic::<SolidColorstatic>::new_zero()),
            // });

            let row_begin = image_height as usize * i as usize / n_jobs;
            let row_end = image_height as usize * (i as usize + 1) / n_jobs;
            let render_height = row_end - row_begin;
            let mut img: RgbImage = ImageBuffer::new(image_width as u32, render_height as u32);
            for x in 0..image_width {
                for (img_y, y) in (row_begin..row_end).enumerate() {
                    let y = y as u32;
                    let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
                    for _s in 0..samples_per_pixels {
                        let u: f64 =
                            ((x) as f64 + random_double_0_1()) / ((image_width - 1) as f64);
                        let v: f64 = ((image_height - y) as f64 + random_double_0_1())
                            / ((image_height - 1) as f64);
                        let r = cam.get_ray(u, v);
                        pixel_color.add_assign(r.ray_color_static::<HittableListstatic>(
                            &background,
                            &world_,
                            &lights,
                            max_depth,
                        ));
                    }
                    let pixel = img.get_pixel_mut(x as u32, img_y as u32);

                    *pixel = image::Rgb([
                        pixel_color.get_u8_x(samples_per_pixels),
                        pixel_color.get_u8_y(samples_per_pixels),
                        pixel_color.get_u8_z(samples_per_pixels),
                    ]);
                }
            }
            tx.send((row_begin..row_end, img))
                .expect("failed to send result");
        });
    }

    //将图片写入
    for (rows, data) in rx.iter().take(n_jobs) {
        for (idx, row) in rows.enumerate() {
            for col in 0..image_width {
                let row = row as u32;
                let idx = idx as u32;
                *results.get_pixel_mut(col as u32, row) = *data.get_pixel(col as u32, idx);
            }
        }
        bar.inc(1);
    }

    results.save("output/test.png").unwrap();
    bar.finish();
}
