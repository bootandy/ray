use data::ray::Ray;
use data::old_vec3::Point;
use data::old_vec3::Color;
use rand::random;
use data::sphere::HitRecord;

pub trait Material {
    fn scatter(&self, ray: &Ray, hr: HitRecord) -> Option<Ray>;
    fn get_albedo(&self) -> &Color;
    fn box_clone(&self) -> Box<dyn Material>;
}

impl Clone for Box<dyn Material> {
    fn clone(&self) -> Box<dyn Material> {
        self.box_clone()
    }
}

#[derive(Debug, Clone)]
pub struct Lambertian {
    pub albedo : Color
}

#[derive(Debug, Clone)]
pub struct Metal {
    pub albedo : Color
}


fn random_in_sphere() -> Point {
    let mut p = Point{x:1.0, y:1.0, z:1.0};
    while p.squared_length() >= 1.0 {
        p = Point{x: random::<f32>()*2.0 - 1.0, y: random::<f32>()*2.0 -1.0, z: random::<f32>()*2.0 - 1.0};
    }
    p
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hr: HitRecord) -> Option<Ray> {
        let target = hr.normal + random_in_sphere();
        let scattered_ray = Ray { origin: hr.p, direction: target  };
        Some(scattered_ray)
    }

    fn get_albedo(&self) -> &Color{
        &self.albedo
    }
    fn box_clone(&self) -> Box<dyn Material> {
        Box::new((*self).clone())
    }
}

fn reflect(v: Point, n : &Point) -> Point {
    v - (*n * v.dot(n) * 2.0)
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray,  hr: HitRecord) -> Option<Ray> {
        let reflected = reflect( ray.direction.clone().unit_vector(), &hr.normal);

        match reflected.dot(&hr.normal) > 0.0 {
            true=> {
                Some(Ray {origin: hr.p, direction: reflected})
            },
            false => {
                None
            }
        }
    }

    fn get_albedo(&self) -> &Color{
        &self.albedo
    }
    fn box_clone(&self) -> Box<dyn Material> {
        Box::new((*self).clone())
    }
}

