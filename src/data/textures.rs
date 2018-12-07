use rnd;
use Color;
use Point;
use PURE_COLOR;

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
        let tmp = result[i];
        result[i] = result[target];
        result[target] = tmp;
    }
    result
}

pub fn build_noise() -> NoiseTexture {
    let mut ran_float = [255u8; 256];
    for i in 0..256 {
        let a = rnd();
        ran_float[i] = (ran_float[i] as f32 * a) as u8;
    }
    NoiseTexture { ran_float }
}

#[derive(Clone, Copy)]
pub struct NoiseTexture {
    pub ran_float: [u8; 256],
}

impl NoiseTexture {
    pub fn value(&self, p: &Point) -> Color {
        self.noise(p)
    }
    fn noise(&self, p: &Point) -> Color {
        let i = p.x.floor() as usize;
        let j = p.y.floor() as usize;
        let k = p.z.floor() as usize;

        let mut c = [[[0; 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ran_float[(permx[(i + di) & 255] ^ permy[(j+dj) & 255] ^ permz[(k+dk)&255]) as usize];
                }
            }
        }
        let c = self.trilinear_int(c, p);
        Color {
            r:c,
            g:c,
            b:c,
        }
    }

    fn trilinear_int(&self, c: [[[u8; 2]; 2]; 2], p: &Point) -> f32 {
        let u2 = p.x - p.x.floor();
        let v2 = p.y - p.y.floor();
        let w2 = p.z - p.z.floor();
        let u = u2 * u2 * (3.0 - 2.0 * u2);
        let v = v2 * v2 * (3.0 - 2.0 * v2);
        let w = w2 * w2 * (3.0 - 2.0 * w2);

        let mut accom : u8 = 0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    accom += ((i as f32 * u + (1 - i) as f32 * (1.0 - u))
                         * (j as f32 * v + (1 - j) as f32 * (1.0 - v))
                         * (k as f32 * w + (1 - k) as f32 * (1.0 - w))
                         * c[i][j][k] as f32) as u8;
                }
            }
        }
        accom as f32 / 256.0
    }
}

#[derive(Clone, Copy)]
pub enum Texture {
    T(ConstantTexture),
    CT(CheckeredTexture),
    NT(NoiseTexture),
}

impl Texture {
    pub fn value(&self, p: &Point) -> Color {
        match self {
            Texture::T(t) => t.value(),
            Texture::CT(ct) => ct.value(p),
            Texture::NT(nt) => nt.value(p),
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
            let a = n.noise(&Point{x:0.5, y:10.5 - i as f32 / 300.0, z:0.5});
            println!("{:?}", a);
        }
        
    }
}
