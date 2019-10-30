use data::material::Material;
use data::old_vec3::Point;
use data::ray::Ray;
use std::f32::MAX;

pub struct HitRecord<'a> {
    pub t: f32,
    pub p: Point,
    pub normal: Point,
    pub material_hit: &'a Box<dyn Material>,
}
fn is_closer_than(hr: &Option<HitRecord>, target: f32) -> bool {
    let biggest_seen = match hr {
        None => MAX,
        Some(thing) => thing.t
    };
    target < biggest_seen && target > 0.001
}


pub enum Hittable {
    Sphere(Sphere),
    MovingSphere(MovingSphere),
}

impl Hittable {
    fn hit(&self, ray: &Ray, closest_found: &Option<HitRecord>) -> Option<HitRecord> {
        match self {
            Hittable::Sphere(sphere) => sphere.hit(ray, closest_found),
            Hittable::MovingSphere(sphere) => sphere.hit(ray, closest_found),
        }
    }
}

pub struct Sphere {
    pub center: Point,
    pub radius: f32,
    pub material: Box<dyn Material>,
}

impl Sphere {
    fn hit(&self, ray: &Ray, closest_found: &Option<HitRecord>) -> Option<HitRecord> {
        let oc = ray.origin.clone() - self.center.clone();
        let a: f32 = ray.direction.dot(&ray.direction);
        let b: f32 = 2.0 * oc.dot(&ray.direction);
        let c: f32 = -(self.radius * self.radius) + oc.dot(&oc);
        let disciminant = b * b - (a * c * 4.0);

        if disciminant > 0.0 {
            let temp = (-b - disciminant.sqrt()) / (2.0 * a);

            let the_t = {
                if is_closer_than(closest_found, temp) {
                    Some(temp)
                } else {
                    let temp = (-b + disciminant.sqrt()) / (2.0 * a);
                    if is_closer_than(closest_found, temp) {
                        Some(temp)
                    } else {
                        None
                    }
                }
            };
            match the_t {
                Some(t) => Some(HitRecord {
                    t,
                    p: ray.point_at_parameter(t),
                    normal: (ray.point_at_parameter(t) - self.center.clone()) / self.radius,
                    material_hit: &self.material,
                }),
                None => None,
            }
        } else {
            None
        }
    }
}

pub struct MovingSphere {
    pub center0: Point,
    pub center1: Point,
    pub radius: f32,
    pub material: Box<dyn Material>,
    pub time0: f32,
    pub time1: f32,
}

impl MovingSphere {
    fn center(&self, time: f32) -> Point {
         self.center0 +  (self.center1 - self.center0)* ((time - self.time0)/(self.time1 - self.time0))
    }
    fn hit(&self, ray: &Ray, closest_found: &Option<HitRecord>) -> Option<HitRecord> {
        let oc = ray.origin.clone() - self.center(ray.time);
        let a: f32 = ray.direction.dot(&ray.direction);
        let b: f32 = 2.0 * oc.dot(&ray.direction);
        let c: f32 = -(self.radius * self.radius) + oc.dot(&oc);
        let disciminant = b * b - (a * c * 4.0);

        if disciminant > 0.0 {
            let temp = (-b - disciminant.sqrt()) / (2.0 * a);

            let the_t = {
                if is_closer_than(closest_found, temp) {
                    Some(temp)
                } else {
                    let temp = (-b + disciminant.sqrt()) / (2.0 * a);
                    if is_closer_than(closest_found, temp) {
                        Some(temp)
                    } else {
                        None
                    }
                }
            };
            match the_t {
                Some(t) => Some(HitRecord {
                    t,
                    p: ray.point_at_parameter(t),
                    normal: (ray.point_at_parameter(t) - self.center(ray.time)) / self.radius,
                    material_hit: &self.material,
                }),
                None => None,
            }
        } else {
            None
        }
    }
}

pub struct HittableObjects {
    pub objects: Vec<Hittable>,
}

impl HittableObjects {
    pub fn hit_all(&self, ray: &Ray) -> Option<HitRecord> {
        let mut best = None;

        for o in self.objects.iter() {
            match o.hit(ray, &best) {
                // change to pass ref of previously hit thing
                Some(hr) => {
                    best = Some(hr);
                }
                None => {}
            }
        }
        best
    }
}
