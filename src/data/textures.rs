use Color;
use Point;

#[derive(Clone, Copy)]
pub struct ConstantTexture {
    pub color: Color,
}

impl ConstantTexture {
    pub fn value<'a>(&'a self, u: f32, v: f32, p: &Point) -> &'a Color {
        &self.color
    }
}

#[derive(Clone, Copy)]
pub struct CheckeredTexture {
    pub color1: Color,
    pub color2: Color,
}

impl CheckeredTexture {
    pub fn value<'a>(&'a self, u: f32, v: f32, p: &Point) -> &'a Color {
        let sins = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sins < 0.0 {
            &self.color1
        } else {
            &self.color2
        }
    }
}

#[derive(Clone, Copy)]
pub enum Texture {
    T(ConstantTexture),
    CT(CheckeredTexture),
}

impl Texture {
    pub fn value<'a>(&'a self, u: f32, v: f32, p: &Point) -> &'a Color {
        match self {
            Texture::T(t) => t.value(u, v, p),
            Texture::CT(ct) => ct.value(u, v, p),
        }
    }
}
