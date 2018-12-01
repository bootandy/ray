use std::f32;

use BoundingBox;
use Material;
use Point;
use Ray;

fn hit<'a>(
    r: &Ray,
    t_min: f32,
    t_max: f32,
    radius: f32,
    material: &'a Material,
    center: &Point,
) -> Option<Hit<'a>> {
    let origin_less_center = r.origin - *center;
    let a = r.direction.dot(&r.direction);
    let b = origin_less_center.dot(&r.direction);
    let c = origin_less_center.dot(&origin_less_center) - radius * radius;
    let quadratic_calc = b * b - (a * c);
    if quadratic_calc > 0.0 {
        let temp = (-b - (b * b - a * c).sqrt()) / a;
        if temp < t_max && temp > t_min {
            let point = r.point_at_parameter(temp);
            let normal = (point - *center) / radius;
            return Some(Hit {
                t: temp,
                p: point,
                normal,
                material,
            });
        }
        let temp = (-b + (b * b - a * c).sqrt()) / a;
        if temp < t_max && temp > t_min {
            let point = r.point_at_parameter(temp);
            let normal = (point - *center) / radius;
            return Some(Hit {
                t: temp,
                p: point,
                normal,
                material,
            });
        }
    }
    None
}

pub struct Hit<'a> {
    pub t: f32,
    pub p: Point,
    pub normal: Point,
    pub material: &'a Material,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit>;
    fn bounding_box(&self) -> BoundingBox;
}

#[derive(Clone)]
pub enum SphereThing {
    S(Sphere),
    SM(SphereMoving),
}

impl Hittable for SphereThing {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        match self {
            SphereThing::S(s) => s.hit(r, t_min, t_max),
            SphereThing::SM(s) => s.hit(r, t_min, t_max),
        }
    }
    fn bounding_box(&self) -> BoundingBox {
        match self {
            SphereThing::S(s) => s.bounding_box(),
            SphereThing::SM(s) => s.bounding_box(),
        }
    }
}

#[derive(Clone)]
pub struct Sphere {
    pub center: Point,
    pub radius: f32,
    pub material: Material,
}
impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        hit(r, t_min, t_max, self.radius, &self.material, &self.center)
    }
    fn bounding_box(&self) -> BoundingBox {
        let radius = Point {
            x: self.radius,
            y: self.radius,
            z: self.radius,
        };
        BoundingBox {
            point1: self.center - radius,
            point2: self.center + radius,
        }
    }
}

#[derive(Clone)]
pub struct SphereMoving {
    pub center0: Point,
    pub center1: Point,
    pub radius: f32,
    pub material: Material,
    pub time0: f32,
    pub time1: f32,
}

impl Hittable for SphereMoving {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        hit(
            r,
            t_min,
            t_max,
            self.radius,
            &self.material,
            &self.get_center(r.time),
        )
    }
    fn bounding_box(&self) -> BoundingBox {
        let point2 = Point {
            x: (self.center0.x + self.radius.abs()).max(self.center1.x + self.radius.abs()),
            y: (self.center0.y + self.radius.abs()).max(self.center1.y + self.radius.abs()),
            z: (self.center0.z + self.radius.abs()).max(self.center1.z + self.radius.abs()),
        };
        let point1 = Point {
            x: (self.center0.x - self.radius.abs()).min(self.center1.x - self.radius.abs()),
            y: (self.center0.y - self.radius.abs()).min(self.center1.y - self.radius.abs()),
            z: (self.center0.z - self.radius.abs()).min(self.center1.z - self.radius.abs()),
        };
        BoundingBox { point1, point2 }
    }
}

impl SphereMoving {
    fn get_center(&self, time: f32) -> Point {
        let t_diff = (time - self.time0) / (self.time1 - self.time0);
        self.center0 + ((self.center1 - self.center0) * t_diff)
    }
}
#[derive(Clone)]
pub struct SphereList {
    pub spheres: Vec<SphereThing>,
}
