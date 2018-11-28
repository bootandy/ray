use std::f32;

use std::cmp::Ordering::Equal;

use rand::random;
use Material;
use Point;
use Ray;

//#[deny(clippy::many_single_char_names)]
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

fn surrounding_box(a: &BoundingBox, b: &BoundingBox) -> BoundingBox {
    let p1 = Point {
        x: a.point1.x.min(b.point1.x),
        y: a.point1.y.min(b.point1.y),
        z: a.point1.z.min(b.point1.z),
    };
    let p2 = Point {
        x: a.point2.x.max(b.point2.x),
        y: a.point2.y.max(b.point2.y),
        z: a.point2.z.max(b.point2.z),
    };
    BoundingBox {
        point1: p1,
        point2: p2,
    }
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
        let point1 = Point {
            x: (self.center0.x + self.radius.abs()).max(self.center1.x + self.radius.abs()),
            y: (self.center0.y + self.radius.abs()).max(self.center1.y + self.radius.abs()),
            z: (self.center0.z + self.radius.abs()).max(self.center1.z + self.radius.abs()),
        };
        let point2 = Point {
            x: (self.center0.x - self.radius.abs()).min(self.center1.x - self.radius.abs()),
            y: (self.center0.y - self.radius.abs()).min(self.center1.y - self.radius.abs()),
            z: (self.center0.z - self.radius.abs()).min(self.center1.z - self.radius.abs()),
        };
        BoundingBox { point1, point2 }
    }
}

impl SphereMoving {
    fn get_center(&self, time: f32) -> Point {
        if (self.time0 - self.time1).abs() < 1000.0 {
            self.center0
        } else {
            let t_diff = (time - self.time0) / (self.time1 - self.time0);
            self.center0 + ((self.center1 - self.center0) * t_diff)
        }
    }
}
#[derive(Clone)]
pub struct SphereList {
    pub spheres: Vec<SphereThing>,
}

#[derive(Clone)]
pub enum BvhBox {
    Leaf(BvhLeaf),
    Node(BvhNode),
}

#[derive(Clone)]
pub struct BvhNode {
    pub left: Box<BvhBox>,
    pub right: Box<BvhBox>,
    pub boxx: BoundingBox,
}

#[derive(Clone)]
pub struct BvhLeaf {
    pub boxx: BoundingBox,
    pub has_a: SphereThing,
}

// pub enum HitResult {
//     Possible(PossibleHit),
//     Hit(Hit),
// }
pub struct PossibleHit<'a> {
    pub boxx: &'a BvhBox,
    pub t: f32,
}

impl BvhNode {
    pub fn hit<'a>(&self, the_enum: &'a BvhBox, r: &Ray) -> Option<PossibleHit<'a>> {
        match self.boxx.hit(r) {
            Some(rr) => Some(PossibleHit {
                boxx: the_enum,
                t: rr,
            }),
            None => None,
        }
    }
}
impl BvhLeaf {
    pub fn hit<'a>(&self, the_enum: &'a BvhBox, r: &Ray) -> Option<PossibleHit<'a>> {
        match self.boxx.hit(r) {
            Some(rr) => Some(PossibleHit {
                boxx: the_enum,
                t: rr,
            }),
            None => None,
        }
    }
}

fn unpack_dig<'a>(a: Option<Hit<'a>>, b: Option<Hit<'a>>) -> Option<Hit<'a>> {
    match (a, b) {
        (Some(l), Some(r)) => {
            if l.t < r.t {
                Some(l)
            } else {
                Some(r)
            }
        }
        (Some(l), None) => Some(l),
        (None, Some(r)) => Some(r),
        (None, None) => None,
    }
}

impl BvhBox {
    fn hit(&self, r: &Ray) -> Option<PossibleHit> {
        match self {
            BvhBox::Leaf(l) => l.hit(&self, r),
            BvhBox::Node(n) => n.hit(&self, r),
        }
    }

    pub fn dig(&self, r: &Ray) -> Option<Hit> {
        match self {
            BvhBox::Leaf(leaf) => {
                let the_hit = leaf.has_a.hit(r, 0.0001, f32::MAX);
                match the_hit {
                    Some(h) => Some(h),
                    None => None,
                }
            }
            BvhBox::Node(node) => {
                let left_hit = node.left.hit(r);
                let right_hit = node.right.hit(r);

                if left_hit.is_some() && right_hit.is_some() {
                    let left_hit_t = left_hit.as_ref().unwrap().t;
                    let right_hit_t = right_hit.as_ref().unwrap().t;
                    if left_hit_t < right_hit_t {
                        let left_dig = left_hit.as_ref().unwrap().boxx.dig(r);
                        if left_dig.is_some() && left_dig.as_ref().unwrap().t < right_hit_t {
                            left_dig
                        } else {
                            let right_dig = right_hit.as_ref().unwrap().boxx.dig(r);
                            unpack_dig(left_dig, right_dig)
                        }
                    } else {
                        let right_dig = right_hit.as_ref().unwrap().boxx.dig(r);
                        if right_dig.is_some() && right_dig.as_ref().unwrap().t < left_hit_t {
                            right_dig
                        } else {
                            let left_dig = left_hit.as_ref().unwrap().boxx.dig(r);
                            unpack_dig(left_dig, right_dig)
                        }
                    }
                } else if left_hit.is_some() {
                    left_hit.as_ref().unwrap().boxx.dig(r)
                } else if right_hit.is_some() {
                    right_hit.as_ref().unwrap().boxx.dig(r)
                } else {
                    None
                }
            }
        }
    }
    pub fn get_box(&self) -> &BoundingBox {
        match self {
            BvhBox::Leaf(leaf) => leaf.get_box(),
            BvhBox::Node(node) => &node.boxx,
        }
    }
}
//# nasty duplication:
impl BvhLeaf {
    pub fn get_box(&self) -> &BoundingBox {
        &self.boxx
    }
}

pub fn get_bvh_box2(spheres: Vec<SphereThing>) -> BvhBox {
    let mut bounds = vec![];
    for a in spheres {
        let mut b = a.bounding_box();
        bounds.push(BvhLeaf { boxx: b, has_a: a });
    }
    get_bvh_box(&mut bounds)
}

pub fn get_bvh_box(spheres: &mut [BvhLeaf]) -> BvhBox {
    let axis: i32 = (random::<f32>() * 3.0) as i32;

    spheres.sort_by(|a, b| {
        a.get_box()
            .point1
            .nth(axis)
            .partial_cmp(&b.get_box().point1.nth(axis))
            .unwrap_or(Equal)
    });
    if spheres.len() == 1 {
        BvhBox::Leaf(spheres[0].clone())
    } else if spheres.len() == 2 {
        let boxx = surrounding_box(spheres[0].get_box(), spheres[1].get_box());
        BvhBox::Node(BvhNode {
            left: Box::new(BvhBox::Leaf(spheres[0].clone())),
            right: Box::new(BvhBox::Leaf(spheres[1].clone())),
            boxx,
        })
    } else {
        let n = spheres.len();
        let left = get_bvh_box(&mut spheres[0..n / 2]);
        let right = get_bvh_box(&mut spheres[n / 2..]);
        let boxx = surrounding_box(left.get_box(), right.get_box());
        BvhBox::Node(BvhNode {
            left: Box::new(left),
            right: Box::new(right),
            boxx,
        })
    }
}

#[derive(Clone)]
pub struct BoundingBox {
    pub point1: Point,
    pub point2: Point,
}

impl BoundingBox {
    pub fn hit(&self, r: &Ray) -> Option<f32> {
        let mut tmin = f32::MIN;
        let mut tmax = f32::MAX;
        for a in 0..3 {
            let p1 = (self.point1.nth(a) - r.origin.nth(a)) / r.direction.nth(a);
            let p2 = (self.point2.nth(a) - r.origin.nth(a)) / r.direction.nth(a);
            let t0 = p1.min(p2);
            let t1 = p1.max(p2);
            tmin = tmin.max(t0);
            tmax = tmax.min(t1);
            if tmax <= tmin {
                return None;
            }
        }
        Some(tmin)
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_bound_box() {
        let a = Point {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        let b = Point {
            x: 1.1,
            y: -1.0,
            z: -1.0,
        };
        let bb = BoundingBox {
            point1: a,
            point2: b,
        };
        // fire ray right (hit)
        let r_hit = Ray {
            origin: Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            direction: Point {
                x: 1.0,
                y: 0.000001,
                z: 0.000001,
            },
            time: 0.0,
        };
        // Fire ray up
        let r_miss_y = Ray {
            origin: Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            direction: Point {
                x: 0.0,
                y: 1.1,
                z: 0.000001,
            },
            time: 0.0,
        };
        // Fire ray forwards
        let r_miss_z = Ray {
            origin: Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            direction: Point {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            time: 0.0,
        };
        // Fire ray right but over box
        let r_miss_x = Ray {
            origin: Point {
                x: 0.0,
                y: 2.0,
                z: 0.0,
            },
            direction: Point {
                x: 1.0,
                y: 0.0,
                z: 1.0,
            },
            time: 0.0,
        };
        // Fire angled ray
        let r_hit_funny = Ray {
            origin: Point {
                x: 0.0,
                y: 2.0,
                z: 2.0,
            },
            direction: Point {
                x: 1.0,
                y: -2.0,
                z: -1.5,
            },
            time: 0.0,
        };
        assert!(bb.hit(&r_hit));
        assert!(!bb.hit(&r_miss_y));
        assert!(!bb.hit(&r_miss_z));
        assert!(!bb.hit(&r_miss_x));
        assert!(bb.hit(&r_hit_funny));
    }
}
