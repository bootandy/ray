use data::vec3::Vec3;

#[derive(Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}


impl Ray {
    pub fn point_at_parameter(&self, p: f32) -> Vec3 {
        self.origin.clone() + (self.direction.clone() * p)
    }
}