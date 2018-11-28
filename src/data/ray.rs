use Point;

pub struct Ray {
    pub origin: Point,
    pub direction: Point,
    pub time: f32,
}

impl Ray {
    pub fn point_at_parameter(&self, t: f32) -> Point {
        self.origin.clone() + self.direction.clone() * t
    }
}
