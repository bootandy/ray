use core::ops;

#[derive(Debug, Clone)]
pub struct Vec3{
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {

    pub fn length(&self) -> f32{
        self.squared_length().sqrt()
    }

    pub fn squared_length(&self) -> f32{
        self.x*self.x + self.y * self.y + self.z*self.z
    }

    pub fn unit_vector(self) -> Vec3 {
        let k = 1.0 / self.length();
        self * k
    }

    pub fn dot(&self, rhs: &Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
    pub fn cross(&self, rhs: &Self) -> Vec3 {
        Vec3 {
            x: self.y * rhs.z - self.z *rhs.y,
            y: - self.x * rhs.z - self.z *rhs.x,
            z: self.x * rhs.y - self.y *rhs.x,
        }
    }

    pub fn as_pixel(&self) -> String {
        format!("{:03} {:03} {:03}\n", (self.x*255.9) as u8 , (self.y*255.9) as u8 , (self.z*255.9) as u8)
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Vec3{
        Vec3{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl ops::Div<Vec3> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f32) -> Vec3 {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f32) -> Vec3 {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Add<f32> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: f32) -> Vec3 {
        Vec3 {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}



impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        ((self.x - other.x).abs() < 0.0001 &&
            (self.y - other.y).abs() < 0.0001 &&
            (self.z - other.z).abs() < 0.0001)
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_add() {
        let a = Vec3{x:2.0, y:3.0, z:4.0};
        let b = Vec3{x:0.0, y:4.0, z:14.0};
        assert_eq!(a + b, Vec3{x:2.0, y:7.0, z:18.0});
    }

    #[test]
    fn test_sub() {
        let a = Vec3{x:2.0, y:3.0, z:4.0};
        let b = Vec3{x:0.0, y:4.0, z:14.0};
        assert_eq!(a - b, Vec3{x:2.0, y:-1.0, z:-10.0});
    }
    #[test]
    fn test_mul() {
        let a = Vec3{x:2.0, y:3.0, z:4.0};
        let b = Vec3{x:0.0, y:4.0, z:14.0};
        assert_eq!(a * b, Vec3{x:0.0, y:12.0, z:56.0});
    }
    #[test]
    fn test_div() {
        let a = Vec3{x:2.0, y:3.0, z:4.0};
        let b = Vec3{x:1.0, y:0.5, z:2.0};
        assert_eq!(a / b, Vec3{x:2.0, y:6.0, z:2.0});
    }

    #[test]
    fn test_mul_f32() {
        let a = Vec3{x:2.0, y:3.0, z:4.0};
        let b = 2.0;
        assert_eq!(a * b, Vec3{x:4.0, y:6.0, z:8.0});
    }
    #[test]
    fn test_div_f32() {
        let a = Vec3 { x: 2.0, y: 3.0, z: 4.0 };
        let b = 2.0;
        assert_eq!(a / b, Vec3 { x: 1.0, y: 1.5, z: 2.0 });
    }

    #[test]
    fn test_unit_vec() {
        let a = Vec3 { x: 2.0, y: 3.0, z: 4.0 };
        assert_eq!(
            a.unit_vector(),
            //Vec3 { x: 10.770329614269007, y:16.155494421403514, z:21.540659228538015});
            Vec3 { x: 0.37139067, y: 0.557086, z: 0.74278134 }
        );
    }

}

