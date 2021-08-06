use crate::rtweekend::random_int_a_b;
use crate::Vec3;

use std::ops::Mul;

//pub use ray::Ray;

pub struct Perlin {
    pub point_count: u32,
    //pub ranfloat: [f64; 256],
    //todo 搞不懂
    pub ranvec: [Vec3; 256],
    pub perm_x: [i32; 256],
    pub perm_y: [i32; 256],
    pub perm_z: [i32; 256],
}

impl Perlin {
    pub fn new() -> Perlin {
        let mut ranvec_: [Vec3; 256] = [Vec3::zero(); 256];
        for i in (0 as usize)..(256 as usize) {
            ranvec_[i] = Vec3::unit_vector(&(Vec3::random_v_a_b(-1.0, 1.0)));
        }

        // let mut ranfloat_: [f64; 256] = [0.0; 256];
        // for i in (0 as usize)..(256 as usize) {
        //     ranfloat_[i] = random_double_0_1();
        // }
        let mut perm_x_: [i32; 256] = [0; 256];
        let mut perm_y_: [i32; 256] = [0; 256];
        let mut perm_z_: [i32; 256] = [0; 256];
        for i in (0 as usize)..(256 as usize) {
            perm_x_[i] = i as i32;
            perm_y_[i] = i as i32;
            perm_z_[i] = i as i32;
        }
        let mut pig = Perlin {
            point_count: 256,
            // ranfloat: ranfloat_,
            ranvec: ranvec_,
            perm_x: perm_x_,
            perm_y: perm_y_,
            perm_z: perm_z_,
        };
        pig.permute();
        pig
    }

    pub fn permute(&mut self) {
        let mut i: usize = 255;
        while i > 0 {
            let targetx = (random_int_a_b(0, i as i32)) as usize;
            let targety = random_int_a_b(0, i as i32) as usize;
            let targetz = random_int_a_b(0, i as i32) as usize;
            let temx = self.perm_x[i];
            let temy = self.perm_y[i];
            let temz = self.perm_z[i];
            self.perm_x[i] = self.perm_x[targetx];
            self.perm_x[targetx] = temx;
            self.perm_y[i] = self.perm_y[targety];
            self.perm_y[targety] = temy;
            self.perm_z[i] = self.perm_z[targetz];
            self.perm_z[targetz] = temz;
            i -= 1;
        }
    }

    // let i = ((4.0 * p.x) as i32) & 255;
    // let j = ((4.0 * p.y) as i32) & 255;
    // let k = ((4.0 * p.z) as i32) & 255;
    // self.ranfloat
    // [(self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize]) as usize]
    pub fn noise(&self, p: &Vec3) -> f64 {
        let mut u = p.x - p.x.floor();
        let mut v = p.y - p.y.floor();
        let mut w = p.z - p.z.floor();

        let i = (p.x.floor()) as i32 as usize;
        let j = (p.y).floor() as i32 as usize;
        let k = (p.z).floor() as i32 as usize;
        let mut c: [[[Vec3; 2]; 2]; 2] = [[[Vec3::zero(); 2]; 2]; 2];

        for di in 0 as usize..2 as usize {
            for dj in 0 as usize..2 as usize {
                for dk in 0 as usize..2 as usize {
                    c[di][dj][dk] = self.ranvec[(self.perm_x[(i + di) & 255 as usize]
                        ^ self.perm_y[(j + dj) & 255 as usize]
                        ^ self.perm_z[(k + dk) & 255 as usize])
                        as usize];
                }
            }
        }
        Perlin::trilinear_interp(c, u, v, w)
    }

    pub fn trilinear_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);

        let mut accum = 0.0;
        for i in 0 as usize..2 as usize {
            for j in 0 as usize..2 as usize {
                for k in 0 as usize..2 as usize {
                    let weight_v = Vec3::new(u - (i as f64), v - (j as f64), w - (k as f64));
                    accum += ((i as f64) * uu + ((1 - i) as f64) * (1.0 - uu))
                        * ((j as f64) * vv + ((1 - j) as f64) * (1.0 - vv))
                        * ((k as f64) * ww + ((1 - k) as f64) * (1.0 - ww))
                        * weight_v.dot(&c[i][j][k]);
                }
            }
        }
        accum
    }

    pub fn turb(&self, p: &Vec3, depth: i32) -> f64 {
        //默认depth=7
        let mut accum = 0.0;
        let mut temp_p = p.clone();
        let mut weight = 1.0;
        for i in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p = temp_p.mul(2.0);
        }
        accum.abs()
    }
}

impl Clone for Perlin {
    fn clone(&self) -> Self {
        Self {
            point_count: self.point_count,
            ranvec: self.ranvec.clone(),
            perm_x: self.perm_x.clone(),
            perm_y: self.perm_y.clone(),
            perm_z: self.perm_z.clone(),
        }
    }
}
