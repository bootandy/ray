use data::ray::Ray;
use data::vec3::Vec3;
use rand::random;

pub trait Material {
    fn scatter(&self, ray: &Ray, p: &Vec3, normal: &Vec3) -> Option<Ray>;
    fn get_albedo(&self) -> Vec3;
    fn box_clone(&self) -> Box<dyn Material>;
}

impl Clone for Box<dyn Material>
{
    fn clone(&self) -> Box<dyn Material> {
        self.box_clone()
    }
}

#[derive(Debug, Clone)]
pub struct Lambertian {
    pub albedo : Vec3
}

#[derive(Debug, Clone)]
pub struct Metal {
    pub albedo : Vec3
}


fn random_in_sphere() -> Vec3 {
    let mut p = Vec3{x:1.0, y:1.0, z:1.0};
    while p.squared_length() >= 1.0 {
        p = Vec3{x: random::<f32>() - 1.0, y: random::<f32>() -1.0, z: random::<f32>() - 1.0} * 2.0;
    }
    p
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, p: &Vec3, normal :&Vec3) -> Option<Ray> {
        let target = p.clone() + normal.clone() + random_in_sphere();
        let scattered_ray = Ray { origin: p.clone(), direction: target - p.clone() };
        Some(scattered_ray)
    }

    fn get_albedo(&self) -> Vec3{
        self.albedo.clone()
    }
    fn box_clone(&self) -> Box<dyn Material> {
        Box::new((*self).clone())
    }
}

fn reflect(v: &Vec3, n : &Vec3) -> Vec3 {
    v.clone() - (n.clone() * v.clone().dot(n) * 2.0)
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, p: &Vec3, normal :&Vec3) -> Option<Ray> {
        let reflected = reflect( &ray.direction.clone().unit_vector(), normal);

        match reflected.dot(normal) > 0.0 {
            true=> {
                Some(Ray {origin: p.clone(), direction: reflected})
            },
            false => {
                None
            }
        }
    }

    fn get_albedo(&self) -> Vec3{
        self.albedo.clone()
    }
    fn box_clone(&self) -> Box<dyn Material> {
        Box::new((*self).clone())
    }
}

