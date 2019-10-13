use data::ray::Ray;
use data::old_vec3::Point;
use std::f32::MAX;
use data::material::Material;

pub struct HitRecord<'a> {
    pub t: f32,
    pub p: Point,
    pub normal: Point,
    pub material_hit: &'a dyn Material,
}

pub enum Hittable<'a> {
    Sphere(Sphere<'a>),
}

impl Hittable<'_> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        match self {
            Hittable::Sphere(sphere) => {
                sphere.hit(ray, t_min, t_max)
            }
        }

    }
}

pub struct Sphere<'a> {
    pub center: Point,
    pub radius: f32,
    pub material: &'a dyn Material,
}

impl Sphere<'_>  {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin.clone() - self.center.clone();
        let a: f32 = ray.direction.dot(&ray.direction);
        let b: f32 = 2.0 * oc.dot(&ray.direction);
        let c: f32 = -(self.radius * self.radius) + oc.dot(&oc);
        let disciminant = b * b - (a * c * 4.0);

        if disciminant > 0.0 {
            let temp = (-b - disciminant.sqrt()) / (2.0 * a);

            let the_t = {
                if temp < t_max && temp > t_min {
                    Some(temp)
                } else {
                    let temp = (-b + disciminant.sqrt()) / (2.0 * a);
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
                        material_hit: &*self.material,
                    })
                },
                None => None,
            }
        } else {
            None
        }
    }
}

pub struct HittableObjects<'a> {
    pub objects: Vec<Hittable<'a>>,
}

impl HittableObjects<'_> {
    pub fn hit_all(&self, ray: &Ray) -> Option<HitRecord> {
        let mut t_hit = MAX;
        let mut best = None;

        for o in self.objects.iter() {

            match o.hit(ray, 0.001, t_hit) {
                // change to pass ref of previously hit thing
                Some(hr) => {
                    t_hit = hr.t;
                    best = Some(hr);
                }
                None => {}
            }
        }
        best
    }
}
