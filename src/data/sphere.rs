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
    orientation: u8,
    pub material: Material,
    is_flipped: bool,
}

impl Rectangle {
    pub fn new_xy(x0: f32, x1: f32, y0: f32, y1: f32, k: f32, material: Material, is_flipped: bool) -> Rectangle {
        Rectangle {
            x0: x0.min(x1),
            x1: x0.max(x1),
            y0: y0.min(y1),
            y1: y0.max(y1),
            k,
            orientation: 0,
            material,
            is_flipped,
        }
    }
    pub fn new_xz(x0: f32, x1: f32, z0: f32, z1: f32, k: f32, material: Material, is_flipped: bool) -> Rectangle {
        Rectangle {
            x0: x0.min(x1),
            x1: x0.max(x1),
            y0: z0.min(z1),
            y1: z0.max(z1),
            k,
            orientation: 1,
            material,
            is_flipped,
        }
    }
    pub fn new_yz(y0: f32, y1: f32, z0: f32, z1: f32, k: f32, material: Material, is_flipped: bool) -> Rectangle {
        Rectangle {
            x0: z0.min(z1),
            x1: z0.max(z1),
            y0: y0.min(y1),
            y1: y0.max(y1),
            k,
            orientation: 2,
            material,
            is_flipped,
        }
    }
    // I think being clever was a very bad idea.
    fn get_xs(&self) -> (f32, f32) {
        match self.orientation {
            0 => (self.x0, self.x1),
            1 => (self.x0, self.x1),
            2 => (self.k - 0.0001, self.k + 0.0001),
            _ => panic!("bad orientation!"),
        }
    }
    fn get_ys(&self) -> (f32, f32) {
        match self.orientation {
            0 => (self.y0, self.y1),
            1 => (self.k - 0.0001, self.k + 0.0001),
            2 => (self.y0, self.y1),
            _ => panic!("bad orientation!"),
        }
    }
    fn get_zs(&self) -> (f32, f32) {
        match self.orientation {
            0 => (self.k - 0.0001, self.k + 0.0001),
            1 => (self.y0, self.y1),
            2 => (self.x0, self.x1),
            _ => panic!("bad orientation!"),
        }
    }
    fn get_x_for(&self, p: &Point) -> f32 {
        match self.orientation {
            0 => p.x,
            1 => p.x,
            2 => p.y,
            _ => panic!("bad orientation!"),
        }
    }
    fn get_y_for(&self, p: &Point) -> f32 {
        match self.orientation {
            0 => p.y,
            1 => p.z,
            2 => p.z,
            _ => panic!("bad orientation!"),
        }
    }
    fn get_z_for(&self, p: &Point) -> f32 {
        match self.orientation {
            0 => p.z,
            1 => p.y,
            2 => p.x,
            _ => panic!("bad orientation!"),
        }
    }
    fn get_normal_for(&self) -> Point {
        let result = match self.orientation {
            0 => Point {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            1 => Point {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            2 => Point {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            _ => panic!("bad orientation!"),
        };
        if self.is_flipped {
            result * -1.0
        } else {
            result
        }
    }
}

impl Hittable for Rectangle {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let t = (self.k - self.get_z_for(&r.origin)) / self.get_z_for(&r.direction);
        if t < t_min || t > t_max {
            return None;
        }
        let x = self.get_x_for(&r.origin) + (t * self.get_x_for(&r.direction));
        let y = self.get_y_for(&r.origin) + (t * self.get_y_for(&r.direction));
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }
        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (y - self.y0) / (self.y1 - self.y0);

        let point = r.point_at_parameter(t);
        Some(Hit {
            t,
            p: point,
            u,
            v,
            normal: self.get_normal_for(),
            material: &self.material,
        })
    }

    fn bounding_box(&self) -> BoundingBox {
        let xs = self.get_xs();
        let ys = self.get_ys();
        let zs = self.get_zs();

        let p1 = Point {
            x: xs.0,
            y: ys.0,
            z: zs.0,
        };
        let p2 = Point {
            x: xs.1,
            y: ys.1,
            z: zs.1,
        };
        BoundingBox {
            point1: p1,
            point2: p2,
        }
    }
}
