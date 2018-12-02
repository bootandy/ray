use Color;
use Point;
use rnd;


#[derive(Clone, Copy)]
pub struct ConstantTexture {
    pub color: Color,
}

impl ConstantTexture {
    pub fn value<'a>(&'a self) -> &'a Color {
        &self.color
    }
}

#[derive(Clone, Copy)]
pub struct CheckeredTexture {
    pub color1: Color,
    pub color2: Color,
}

impl CheckeredTexture {
    pub fn value<'a>(&'a self, p: &Point) -> &'a Color {
        let sins = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sins < 0.0 {
            &self.color1
        } else {
            &self.color2
        }
    }
}

lazy_static! {
    static ref permx : [u8; 256] = build_static_perm();
    static ref permy : [u8; 256] = build_static_perm();
    static ref permz : [u8; 256] = build_static_perm();
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
    let mut ran_float = [Color{r:1.0, g:1.0, b:1.0}; 256];
    for i in 0..256 {
        let a = rnd();
        ran_float[i] = ran_float[i] * a;
    }
    NoiseTexture {
        ran_float
    }
}

#[derive(Clone, Copy)]
pub struct NoiseTexture {
    pub ran_float : [Color; 256],
}

impl NoiseTexture {
    pub fn value<'a>(&'a self, u: f32, v: f32, p: &Point) -> &'a Color {
        self.noise(p)
    }
    fn noise<'a>(&'a self, p: &Point) -> &'a Color {
        /*let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();
        */
        let i = ((4.0 * p.x) as u8) as usize;
        let j = ((4.0 * p.y) as u8) as usize;
        let k = ((4.0 * p.z) as u8) as usize;
        let xord = (permx[i] ^ permy[j] ^ permz[k]) as usize;
        &self.ran_float[xord]
    }
}

#[derive(Clone, Copy)]
pub enum Texture {
    T(ConstantTexture),
    CT(CheckeredTexture),
    NT(NoiseTexture),
}

impl Texture {
    pub fn value<'a>(&'a self, u: f32, v: f32, p: &Point) -> &'a Color {
        match self {
            Texture::T(t) => t.value(),
            Texture::CT(ct) => ct.value(p),
            Texture::NT(nt) => nt.value(u, v, p),
        }
    }
}
