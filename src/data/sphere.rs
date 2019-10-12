use data::vec3::Vec3;
use data::ray::Ray;
use std::f32::MAX;

pub struct HitRecord {
    t: f32,
    p: Vec3,
    pub normal: Vec3,
}

pub trait Hittable {
    fn hit(&self, ray : &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Hittable for Sphere {
    fn hit(&self, ray : &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {

        let oc = ray.origin.clone() - self.center.clone();
        let a :f32 = ray.direction.dot(&ray.direction);
        let b :f32 = 2.0 * oc.dot(&ray.direction);
        let c :f32 = - (self.radius * self.radius) + oc.dot(&oc);
        let disciminant = b*b - (a*c*4.0);

        if disciminant > 0.0 {
            let temp = (-b - ((b * b - (4.0 * a * c)).sqrt())) / (2.0 * a);

            let the_t = {
                if temp < t_max && temp > t_min {
                    Some(temp)
                } else {
                    let temp = (-b + ((b * b - (4.0 * a * c)).sqrt())) / (2.0 * a);
                    if temp < t_max && temp > t_min {
                        Some(temp)
                    } else {
                        None
                    }
                }
            };
            match the_t {
                Some(t) => {
                    Some(HitRecord {
                        t,
                        p: ray.point_at_parameter(t),
                        normal: (ray.point_at_parameter(t) - self.center.clone()) / self.radius,
                    })
                },
                None => None,
            }
        } else {
            None
        }
    }

}

pub struct HittableObjects {
    pub objects: Vec<Box<dyn Hittable >>,
}

impl HittableObjects {
    pub fn hit_all(&self, ray: &Ray) -> Option<HitRecord> {
        let mut t_hit = MAX;
        let mut best = None;

        for o in self.objects.iter() {
            //let tmp = *o + 1;

            match o.hit(ray, 0.0, t_hit) { // change to pass ref of previously hit thing
                Some(hr) => {
                    t_hit = hr.t;
                    best = Some(hr);
                },
                None => {},
            }
        }
        best
    }
}
