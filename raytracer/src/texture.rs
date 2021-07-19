use crate::hittable_list;
use crate::matirial;
use crate::matirial::clamp;
use crate::perlin;
use crate::perlin::Perlin;
use crate::RAY::{HitRecord, Material, Ray};
use crate::{rtweekend, Vec3};
use image::{DynamicImage, GenericImageView, ImageBuffer, Primitive, Rgb, RgbImage};
use imageproc::drawing::Canvas;
use std::ops::{Add, Mul};
use std::sync::Arc;

pub trait Texture {
    fn value(&self, u: f64, n: f64, p: &Vec3) -> Vec3;
}

pub struct SolidColor {
    pub color_value: Vec3,
}

impl SolidColor {
    pub fn new_zero() -> SolidColor {
        SolidColor {
            color_value: Vec3::zero(),
        }
    }

    pub fn new(c: Vec3) -> SolidColor {
        SolidColor { color_value: c }
    }

    pub fn new2(red: f64, green: f64, blue: f64) -> SolidColor {
        SolidColor {
            color_value: Vec3::new(red, green, blue),
        }
    }
}

impl Texture for SolidColor {
    //应该未完成
    fn value(&self, u: f64, n: f64, p: &Vec3) -> Vec3 {
        self.color_value.clone()
    }
}

pub struct CheckerTexture {
    pub odd: Arc<dyn Texture>,
    pub even: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(even_: Arc<dyn Texture>, odd_: Arc<dyn Texture>) -> CheckerTexture {
        CheckerTexture {
            odd: odd_,
            even: even_,
        }
    }

    pub fn new2(c1: Vec3, c2: Vec3) -> CheckerTexture {
        CheckerTexture {
            even: Arc::new(SolidColor::new(c1)),
            odd: Arc::new(SolidColor::new(c2)),
        }
    }

    pub fn new_zero() -> CheckerTexture {
        CheckerTexture {
            even: Arc::new(SolidColor::new(Vec3::zero())),
            odd: Arc::new(SolidColor::new(Vec3::zero())),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        let sines = (10.0 * p.x).sin() * ((10.0 * p.y).sin()) * ((10.0 * p.z).sin());
        if sines < 0.0 {
            return self.odd.value(u, v, &p);
        };
        self.even.value(u, v, &p) //这里可不可以引用？我忘了诶
    }
}

pub struct NoiseTexture {
    pub noise: Perlin,
    pub scale: f64,
}

impl NoiseTexture {
    pub fn new(sc: f64) -> NoiseTexture {
        NoiseTexture {
            noise: Perlin::new(),
            scale: sc,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, u: f64, n: f64, p: &Vec3) -> Vec3 {
        //Vec3::new(1.0, 1.0, 1.0).mul(self.noise.turb(&p.mul(self.scale), 7))
        Vec3::new(1.0, 1.0, 1.0)
            .mul(0.5)
            .mul(((self.noise.turb(&p, 7)).mul(10.0).add(p.z * self.scale)).sin() + 1.0)
    }
}

pub struct ImageTexture {
    pub data: DynamicImage,
    pub width: u32,
    pub height: u32,
}

impl ImageTexture {
    //这里以后可以改进//==现在不想改进
    pub fn new(filename: &str) -> ImageTexture {
        let mut pig = image::open(filename).unwrap();
        let w = image::GenericImageView::width(&pig);
        let h = image::GenericImageView::height(&pig);

        ImageTexture {
            data: pig,
            width: w,
            height: h,
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        let u_mid = clamp(u, 0.0, 1.0);
        let v_mid = 1.0 - clamp(v, 0.0, 1.0);
        let mut i = (u_mid * (self.width as f64)) as u32;
        if i >= self.width {
            i = (self.width) - 1;
        }
        let mut j = (v_mid * (self.height as f64)) as u32;
        if j >= self.height {
            j = (self.height) - 1;
        }

        let pixel_ = image::GenericImageView::get_pixel(&(self.data), i, j);
        //println!("{}", pixel_[0]);
        Vec3::new(
            pixel_[0] as f64 / 255.0,
            pixel_[1] as f64 / 255.0,
            pixel_[2] as f64 / 255.0,
        )
    }
}
