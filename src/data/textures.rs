use image::DynamicImage;
use image::GenericImageView;
use rnd;
use Color;
use Point;
use PURE_COLOR;
use std::num::Wrapping;

#[derive(Clone, Copy)]
pub struct ConstantTexture {
    pub color: Color,
}

impl ConstantTexture {
    pub fn value(&self) -> Color {
        self.color
    }
}

#[derive(Clone, Copy)]
pub struct CheckeredTexture {
    pub color1: Color,
    pub color2: Color,
}

impl CheckeredTexture {
    pub fn value(&self, p: &Point) -> Color {
        let sins = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sins < 0.0 {
            self.color1
        } else {
            self.color2
        }
    }
}

lazy_static! {
    static ref permx: [u8; 256] = build_static_perm();
    static ref permy: [u8; 256] = build_static_perm();
    static ref permz: [u8; 256] = build_static_perm();
}
fn build_static_perm() -> [u8; 256] {
    let mut result: [u8; 256] = [0; 256];
    for i in 0..256 {
        result[i] = i as u8;
    }
    for i in (0..256).rev() {
        let target = (rnd() * i as f32) as usize;
        result.swap(i, target);
    }
    result
}

pub fn build_noise() -> NoiseTexture {
    let mut ran_float = [Point {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    }; 256];
    for i in 0..256 {
        let p = Point {
            x: 1.0 - 2.0 * rnd(),
            y: 1.0 - 2.0 * rnd(),
            z: 1.0 - 2.0 * rnd(),
        };
        ran_float[i] = p.unit_vector();
    }
    let scale = rnd();
    NoiseTexture { ran_float, scale }
}

#[derive(Clone, Copy)]
pub struct NoiseTexture {
    ran_float: [Point; 256],
    scale: f32,
}

impl NoiseTexture {
    pub fn value(&self, p: &Point) -> Color {
        let c = ((self.turb(p) * 10.0) + p.y * self.scale).sin() * 0.5;
        Color { r: c, g: c, b: c }
    }

    pub fn turb(&self, p: &Point) -> f32 {
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;
        for _i in 0..1 {
            accum += self.noise(&temp_p) * weight;
            weight *= 0.5;
            temp_p = temp_p * 2.0;
        }
        accum.abs()
    }

    fn noise(&self, p: &Point) -> f32 {
        let i = Wrapping(p.x.floor() as u8);
        let j = Wrapping(p.y.floor() as u8);
        let k = Wrapping(p.z.floor() as u8);

        let mut c = [[[Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }; 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ran_float[(permx[(i + Wrapping(di as u8)).0 as usize]
                                                       ^ permy[(j + Wrapping(dj as u8)).0 as usize]
                                                       ^ permz[(k + Wrapping(dk as u8)).0 as usize])
                                                       as usize];
                }
            }
        }
        self.trilinear_int(c, p)
    }

    fn trilinear_int(&self, c: [[[Point; 2]; 2]; 2], p: &Point) -> f32 {
        let u2 = p.x - p.x.floor();
        let v2 = p.y - p.y.floor();
        let w2 = p.z - p.z.floor();
        let u = u2 * u2 * (3.0 - 2.0 * u2);
        let v = v2 * v2 * (3.0 - 2.0 * v2);
        let w = w2 * w2 * (3.0 - 2.0 * w2);

        let mut accom: f32 = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight = Point {
                        x: u - i as f32,
                        y: v - j as f32,
                        z: w - k as f32,
                    };
                    let f_bit = (i as f32 * u + (1 - i) as f32 * (1.0 - u))
                        * (j as f32 * v + (1 - j) as f32 * (1.0 - v))
                        * (k as f32 * w + (1 - k) as f32 * (1.0 - w));

                    accom += (c[i][j][k] * f_bit).dot(&weight);
                }
            }
        }
        accom
    }
}

pub fn build_image_texture() -> ImageTexture {
    let img = image::open("./earth.jpeg").unwrap();
    println!("{:?}", img.dimensions());
    ImageTexture { img: Box::new(img) }
}

#[derive(Clone)]
pub struct ImageTexture {
    pub img: Box<DynamicImage>,
}

impl ImageTexture {
    pub fn value(&self, u: f32, v: f32) -> Color {
        let (width, height) = self.img.dimensions();
        let x_pixel = (u) * width as f32;
        let y_pixel = (1.0 - v) * height as f32;
        let pixel = self.img.get_pixel(x_pixel as u32, y_pixel as u32);
        let colors = pixel.data;
        Color {
            r: colors[0] as f32 / 255.0,
            b: colors[2] as f32 / 255.0,
            g: colors[1] as f32 / 255.0,
        }
    }
}

#[derive(Clone)]
pub enum Texture {
    T(ConstantTexture),
    CT(CheckeredTexture),
    NT(NoiseTexture),
    IT(ImageTexture),
}

impl Texture {
    pub fn value(&self, p: &Point, u: f32, v: f32) -> Color {
        match self {
            Texture::T(t) => t.value(),
            Texture::CT(ct) => ct.value(p),
            Texture::NT(nt) => nt.value(p),
            Texture::IT(it) => it.value(u, v),
        }
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_noise() {
        // Tests nothing - just me playing
        let mut ran_float = [255u8; 256];
        for i in 0..256 {
            if i % 2 == 0 {
                ran_float[i] = 0;
            } else {
                ran_float[i] = 255;
            }
            //ran_float[i] = i as u8;
        }
        let n = NoiseTexture { ran_float };
        for i in 0..100 {
            let a = n.noise(&Point {
                x: 0.5,
                y: 10.5 - i as f32 / 300.0,
                z: 0.5,
            });
        }
    }
}
