use std::f32;
use std::f32::consts::PI;

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
            let (u, v) = get_sphere_uv(normal);
            return Some(Hit {
                t: temp,
                p: point,
                u,
                v,
                normal,
                material,
            });
        }
        let temp = (-b + (b * b - a * c).sqrt()) / a;
        if temp < t_max && temp > t_min {
            let point = r.point_at_parameter(temp);
            let normal = (point - *center) / radius;
            let (u, v) = get_sphere_uv(normal);
            return Some(Hit {
                t: temp,
                p: point,
                u,
                v,
                normal,
                material,
            });
        }
    }
    None
}

fn get_sphere_uv(p: Point) -> (f32, f32) {
    let phi = limit_value_for_trig(p.z).atan2(limit_value_for_trig(p.x));
    let theta = limit_value_for_trig(p.y).asin();
    assert!(!phi.is_nan());
    assert!(!theta.is_nan());
    let u = 1.0 - (phi + PI) / (2.0 * PI);
    let v = (theta + PI / 2.0) / PI;
    (u, v)
}

fn limit_value_for_trig(a: f32) -> f32 {
    a.max(-0.999).min(0.9999)
}

pub struct Hit<'a> {
    pub t: f32,
    pub p: Point,
    pub u: f32,
    pub v: f32,
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
    R(Rectangle),
}

impl Hittable for SphereThing {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        match self {
            SphereThing::S(s) => s.hit(r, t_min, t_max),
            SphereThing::SM(s) => s.hit(r, t_min, t_max),
            SphereThing::R(s) => s.hit(r, t_min, t_max),
        }
    }
    fn bounding_box(&self) -> BoundingBox {
        match self {
            SphereThing::S(s) => s.bounding_box(),
            SphereThing::SM(s) => s.bounding_box(),
            SphereThing::R(s) => s.bounding_box(),
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

#[derive(Clone)]
pub struct Rectangle {
    pub x0: f32,
    pub x1: f32,
    pub y0: f32,
    pub y1: f32,
    pub k: f32,
    pub material: Material,
}

impl Hittable for Rectangle{
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let t = (self.k - r.origin.z) / r.direction.z;
        if t < t_min || t > t_max {
            return None;
        }
        let x = r.origin.x + (t * r.direction.x);
        let y = r.origin.y + (t * r.direction.y);
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }
        let u = (x - self.x0) / (self.x1-self.x0);
        let v = (y - self.y0) / (self.y1-self.y0);

        let point = r.point_at_parameter(t);
        let normal = Point{x:0.0, y:0.0, z:1.0};
        Some(Hit {
                t, 
                p: point,
                u,
                v,
                normal,
                material: &self.material,
        })
    }

    fn bounding_box(&self) -> BoundingBox {
        let p1 = Point {
            x: self.x0,
            y: self.y0,
            z: self.k - 0.0000001,
        };
        let p2 = Point {
            x: self.x1,
            y: self.y1,
            z: self.k + 0.0000001,
        };
        BoundingBox {
            point1: p1,
            point2: p2,
        }
    }
}
