use std::ops;

trait Ops {
}

impl ops::Add<f32> for Color {
    type Output = Color ;
    fn add(self, rhs: f32) -> Color {
        Color{r:self.r+rhs, g:self.g+rhs, b:self.b+rhs}
    }
}

impl ops::Add<f32> for Point {
    type Output = Point;
    fn add(self, rhs: f32) -> Point {
        Point{x:self.x+rhs, y:self.y+rhs, z:self.z+rhs}
    }
}

#[derive(Debug, Clone, Copy, From, Add, AddAssign, Sub, Mul, Div)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, Copy, From, Add, AddAssign, Sub, Mul, Div)]
pub struct Color {
    pub r:f32,
    pub g:f32,
    pub b:f32,
}

fn len(a: f32, b: f32, c: f32) -> f32 {
    (a * a + b * b + c * c).sqrt()
}

fn to_bytes(a : f32) -> u8 {
    (255.99 * a.sqrt()) as u8
}
impl Color {
    pub fn as_color_str(&self) -> String {
        format!("{} {} {}\n", to_bytes(self.r), to_bytes(self.g), to_bytes(self.b))
    }
    pub fn mul(&self, rhs: &Color) -> Color {
        Color{r:self.r*rhs.r, g:self.g*rhs.g, b:self.b*rhs.b}
    }
    pub fn len(&self) -> f32 {
        len(self.r, self.g, self.b)
    }
}

impl Point  {
    pub fn dot(&self, r: &Point) -> f32 {
        self.x * r.x + self.y * r.y + self.z * r.z 
    }
    pub fn squared_length(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: -(self.x * other.z - self.z * other.x),
            z: self.x * other.y - self.y * other.x,
        }
    }
    pub fn len(&self) -> f32 {
        len(self.x, self.y, self.z)
    }

    pub fn unit_vector(&self)-> Point {
        let pp = self.len();
        Point {
            x: self.x / pp,
            y: self.y / pp,
            z: self.z / pp,
        }
    }
}
