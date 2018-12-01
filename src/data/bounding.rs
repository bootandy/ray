use std::cmp::Ordering::Equal;
use std::f32;

use rnd;
use Hit;
use Hittable;
use Point;
use Ray;
use SphereThing;

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

#[derive(Clone, Copy)]
pub struct PossibleHit<'a> {
    pub boxx: &'a BvhBox,
    pub t: f32,
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

fn hit_bvh<'a>(the_enum: &'a BvhBox, r: &Ray) -> Option<PossibleHit<'a>> {
    match the_enum.get_box().hit(r) {
        Some(rr) => Some(PossibleHit {
            boxx: the_enum,
            t: rr,
        }),
        None => None,
    }
}

fn get_t_or_max(hit: &Option<Hit>) -> f32 {
    match hit {
        Some(h) => h.t,
        None => f32::MAX,
    }
}

fn process_hit<'a>(hit: PossibleHit<'a>, deepest_hit: f32, r: &Ray) -> Option<Hit<'a>> {
    if hit.t > deepest_hit {
        None
    } else {
        hit.boxx.dig(r, f32::MAX)
    }
}

fn process_double_hit<'a>(
    hit: PossibleHit<'a>,
    hit2: PossibleHit<'a>,
    deepest_hit: f32,
    r: &Ray,
) -> Option<Hit<'a>> {
    if hit.t > deepest_hit {
        None
    } else {
        let hit_dug = hit.boxx.dig(r, f32::MAX);
        let hit_dug_t = get_t_or_max(&hit_dug);
        if hit_dug_t < hit2.t {
            hit_dug
        } else {
            match (hit_dug, hit2.boxx.dig(r, hit_dug_t)) {
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
    }
}

impl BvhNode {
    pub fn hit<'a>(&'a self, the_enum: &'a BvhBox, r: &Ray) -> Option<PossibleHit<'a>> {
        hit_bvh(the_enum, r)
    }
    pub fn dig<'a>(&'a self, r: &Ray, deepest_hit: f32) -> Option<Hit<'a>> {
        let left_hit: Option<PossibleHit<'a>> = self.left.hit(r);
        let right_hit: Option<PossibleHit<'a>> = self.right.hit(r);
        match (left_hit, right_hit) {
            (Some(left), Some(right)) => {
                if left.t < right.t {
                    process_double_hit(left, right, deepest_hit, r)
                } else {
                    process_double_hit(right, left, deepest_hit, r)
                }
            }
            (Some(left), None) => process_hit(left, deepest_hit, r),
            (None, Some(right)) => process_hit(right, deepest_hit, r),
            (None, None) => None,
        }
    }
}

impl BvhLeaf {
    pub fn hit<'a>(&'a self, the_enum: &'a BvhBox, r: &Ray) -> Option<PossibleHit<'a>> {
        hit_bvh(the_enum, r)
    }
    pub fn dig(&self, r: &Ray) -> Option<Hit> {
        match self.has_a.hit(r, 0.0001, f32::MAX) {
            Some(h) => Some(h),
            None => None,
        }
    }
    pub fn get_box(&self) -> &BoundingBox {
        &self.boxx
    }
}

impl BvhBox {
    fn hit(&self, r: &Ray) -> Option<PossibleHit> {
        match self {
            BvhBox::Leaf(l) => l.hit(&self, r),
            BvhBox::Node(n) => n.hit(&self, r),
        }
    }

    pub fn dig(&self, r: &Ray, deepest_hit: f32) -> Option<Hit> {
        match self {
            BvhBox::Leaf(leaf) => leaf.dig(r),
            BvhBox::Node(node) => node.dig(r, deepest_hit),
        }
    }
    pub fn get_box(&self) -> &BoundingBox {
        match self {
            BvhBox::Leaf(leaf) => leaf.get_box(),
            BvhBox::Node(node) => &node.boxx,
        }
    }
}

pub fn get_bvh_box(spheres: &mut [BvhLeaf]) -> BvhBox {
    let axis: i32 = (rnd() * 3.0) as i32;

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
