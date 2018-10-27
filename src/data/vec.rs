use std::ops;

#[derive(Debug, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point {

    pub fn dot(&self, o: &Point) -> f32 {
        self.x * o.x + self.y * o.y + self.z * o.z
    }

    pub fn add(&self, o: &Point) -> Point {
        Point{x:self.x+o.x, y:self.y+o.y, z:self.z+o.z}
    }
    pub fn sub(&self, o: &Point) -> Point {
        Point{x:self.x-o.x, y:self.y-o.y, z:self.z-o.z}
    }
    pub fn mul(&self, o: &Point)-> Point  {
        Point{x:self.x*o.x, y:self.y*o.y, z:self.z*o.z}
    }

    pub fn flat_mul(&self, f: f32)-> Point  {
        Point{x:self.x*f, y:self.y*f, z:self.z*f}
    }
    pub fn flat_div(&self, f: f32)-> Point  {
        Point{x:self.x/f, y:self.y/f, z:self.z/f}
    }
    pub fn flat_add(&self, f: f32)-> Point  {
        Point{x:self.x+f, y:self.y+f, z:self.z+f}
    }
    pub fn flat_sub(&self, f: f32)-> Point  {
        Point{x:self.x-f, y:self.y-f, z:self.z-f}
    }

    pub fn squared_length(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn length(&self) -> f32 {
        self.squared_length().sqrt()
    }

    pub fn unit_vector(&self) -> Point {
        Point{x: self.x / 3.0, y: self.y / 3.0, z: self.z / 3.0}
    }
    pub fn cross(&self, o: &Point) -> Point {
        Point{
            x:self.y * o.z - self.z * o.y,
            y:-(self.x * o.z - self.z * o.x),
            z:self.x * o.y - self.y * o.x,
        }
    }
}

impl ops::Add<&Point> for Point {
    type Output = Point;
    fn add(self, o: &Point) -> Point {
        Point{x:self.x+o.x, y:self.y+o.y, z:self.z+o.z}
    }
}


mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_cross() {
        let a = Point{x:2.0, y:3.0, z:4.0};
        let b = Point{x:5.0, y:6.0, z:7.0};
        let c = a.cross(&b);
        assert_eq!(c.x, -3.0);
        assert_eq!(c.y, 6.0);
        assert_eq!(c.z, -3.0);
    }
}