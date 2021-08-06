use crate::matirial::clamp;

use crate::perlin::Perlin;

use crate::Vec3;
use image::DynamicImage;

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

pub trait Texturestatic: Texture + std::clone::Clone {}

pub struct SolidColorstatic {
    pub color_value: Vec3,
}

impl SolidColorstatic {
    pub fn new_zero() -> SolidColorstatic {
        SolidColorstatic {
            color_value: Vec3::zero(),
        }
    }

    pub fn new(c: Vec3) -> SolidColorstatic {
        SolidColorstatic { color_value: c }
    }

    pub fn new2(red: f64, green: f64, blue: f64) -> SolidColorstatic {
        SolidColorstatic {
            color_value: Vec3::new(red, green, blue),
        }
    }
}

impl Texture for SolidColorstatic {
    fn value(&self, u: f64, n: f64, p: &Vec3) -> Vec3 {
        self.color_value.clone()
    }
}

impl Clone for SolidColorstatic {
    fn clone(&self) -> Self {
        SolidColorstatic {
            color_value: self.color_value.clone(),
        }
    }
}

impl Texturestatic for SolidColorstatic {}

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

pub struct CheckerTexturestatic<T0: Texturestatic, T1: Texturestatic> {
    pub even: T0,
    pub odd: T1,
}

impl<T0: Texturestatic, T1: Texturestatic> CheckerTexturestatic<T0, T1> {
    pub fn new(even_: T0, odd_: T1) -> CheckerTexturestatic<T0, T1> {
        CheckerTexturestatic {
            odd: odd_,
            even: even_,
        }
    }

    pub fn new2(c1: Vec3, c2: Vec3) -> CheckerTexturestatic<SolidColorstatic, SolidColorstatic> {
        CheckerTexturestatic {
            even: (SolidColorstatic::new(c1)),
            odd: (SolidColorstatic::new(c2)),
        }
    }
}

impl<T0: Texturestatic, T1: Texturestatic> Texture for CheckerTexturestatic<T0, T1> {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        let sines = (10.0 * p.x).sin() * ((10.0 * p.y).sin()) * ((10.0 * p.z).sin());
        if sines < 0.0 {
            return self.odd.value(u, v, &p);
        };
        self.even.value(u, v, &p) //这里可不可以引用？我忘了诶
    }
}

impl<T0: Texturestatic, T1: Texturestatic> Clone for CheckerTexturestatic<T0, T1> {
    fn clone(&self) -> Self {
        Self {
            odd: self.odd.clone(),
            even: self.even.clone(),
        }
    }
}

impl<T0: Texturestatic, T1: Texturestatic> Texturestatic for CheckerTexturestatic<T0, T1> {}

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
        //"input/me.png"
        //../../in
        Vec3::new(1.0, 1.0, 1.0)
            .mul(0.5)
            .mul(((self.noise.turb(&p, 7)).mul(10.0).add(p.z * self.scale)).sin() + 1.0)
    }
}

pub struct NoiseTexturestatic {
    pub noise: Perlin,
    pub scale: f64,
}

impl NoiseTexturestatic {
    pub fn new(sc: f64) -> NoiseTexturestatic {
        NoiseTexturestatic {
            noise: Perlin::new(),
            scale: sc,
        }
    }
}

impl Texture for NoiseTexturestatic {
    fn value(&self, u: f64, n: f64, p: &Vec3) -> Vec3 {
        //Vec3::new(1.0, 1.0, 1.0).mul(self.noise.turb(&p.mul(self.scale), 7))
        //"input/me.png"
        //../../in
        Vec3::new(1.0, 1.0, 1.0)
            .mul(0.5)
            .mul(((self.noise.turb(&p, 7)).mul(10.0).add(p.z * self.scale)).sin() + 1.0)
    }
}

impl Clone for NoiseTexturestatic {
    fn clone(&self) -> Self {
        Self {
            noise: self.noise.clone(),
            scale: self.scale,
        }
    }
}

impl Texturestatic for NoiseTexturestatic {}

pub struct ImageTexture {
    pub data: DynamicImage,
    pub width: u32,
    pub height: u32,
}

impl ImageTexture {
    //这里以后可以改进//==现在不想改进
    pub fn new(filename: &str) -> ImageTexture {
        let pig = image::open(filename).unwrap();
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
        let mut i_kun = (u_mid * (self.width as f64)) as u32;
        if i_kun >= self.width {
            i_kun = (self.width) - 1;
        }
        let mut j_kun = (v_mid * (self.height as f64)) as u32;
        if j_kun >= self.height {
            j_kun = (self.height) - 1;
        }

        let pixel_ = image::GenericImageView::get_pixel(&(self.data), i_kun, j_kun);
        //println!("{}", pixel_[0]);
        Vec3::new(
            pixel_[0] as f64 / 255.0,
            pixel_[1] as f64 / 255.0,
            pixel_[2] as f64 / 255.0,
        )
    }
}

pub struct ImageTexturestatic {
    pub data: DynamicImage,
    pub width: u32,
    pub height: u32,
}

impl ImageTexturestatic {
    //这里以后可以改进//==现在不想改进
    pub fn new(filename: &str) -> ImageTexturestatic {
        let pig = image::open(filename).unwrap();
        let w = image::GenericImageView::width(&pig);
        let h = image::GenericImageView::height(&pig);

        ImageTexturestatic {
            data: pig,
            width: w,
            height: h,
        }
    }
}

impl Texture for ImageTexturestatic {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        let u_mid = clamp(u, 0.0, 1.0);
        let v_mid = 1.0 - clamp(v, 0.0, 1.0);
        let mut i_mid = (u_mid * (self.width as f64)) as u32;
        if i_mid >= self.width {
            i_mid = (self.width) - 1;
        }
        let mut j_mid = (v_mid * (self.height as f64)) as u32;
        if j_mid >= self.height {
            j_mid = (self.height) - 1;
        }

        let pixel_ = image::GenericImageView::get_pixel(&(self.data), i_mid, j_mid);
        //println!("{}", pixel_[0]);
        Vec3::new(
            pixel_[0] as f64 / 255.0,
            pixel_[1] as f64 / 255.0,
            pixel_[2] as f64 / 255.0,
        )
    }
}

impl Clone for ImageTexturestatic {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            width: self.width,
            height: self.height,
        }
    }
}

impl Texturestatic for ImageTexturestatic {}
