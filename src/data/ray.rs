use data::old_vec3::Point;

#[derive(Debug)]
pub struct Ray {
    pub origin: Point,
    pub direction: Point,
}

impl Ray {
    pub fn point_at_parameter(&self, p: f32) -> Point {
        self.origin.clone() + (self.direction.clone() * p)
    }
}
