use data::old_vec3::Color;
use data::old_vec3::Point;
use data::ray::Ray;
use data::sphere::HitRecord;
use rand::random;

//pub trait Material {
//    fn scatter(&self, ray: &Ray, hr: HitRecord) -> Option<Ray>;
//    fn get_albedo(&self) -> &Color;
//    fn box_clone(&self) -> Box<dyn Material>;
//}

//impl Clone for Box<dyn Material> {
//    fn clone(&self) -> Box<dyn Material> {
//        self.box_clone()
//    }
//}


// #[derive(Send , Sync)]
#[derive(Debug, Clone)]
pub struct Lambertian {
    pub albedo: Color,
}

#[derive(Debug, Clone)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32,
}

#[derive(Debug, Clone)]
pub struct Dielectric{
    pub reflective_index: f32,
}

pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl Material {
    pub fn scatter(&self, r: &Ray, hr: HitRecord) -> Option<Ray> {
        match self {
            Material::Metal(metal) => {
                metal.scatter(r, hr)
            },
            Material::Dielectric(e) => {
                e.scatter(r, hr)
            },
            Material::Lambertian(l) => {
                l.scatter(r, hr)
            },
        }
    }
    pub fn get_albedo(&self) -> &Color {
        match self {
            Material::Metal(metal) => {
                metal.get_albedo()
            },
            Material::Dielectric(e) => {
                e.get_albedo()
            },
            Material::Lambertian(l) => {
                l.get_albedo()
            },
        }
    }
}

fn random_in_sphere() -> Point {
    let mut p = Point {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };
    while p.squared_length() >= 1.0 {
        p = Point {
            x: random::<f32>() * 2.0 - 1.0,
            y: random::<f32>() * 2.0 - 1.0,
            z: random::<f32>() * 2.0 - 1.0,
        };
    }
    p
}

fn reflect(v: Point, n: &Point) -> Point {
    v - (*n * v.dot(n) * 2.0)
}

impl Lambertian {
    fn scatter(&self, _ray: &Ray, hr: HitRecord) -> Option<Ray> {
        let target = hr.normal + random_in_sphere();
        let scattered_ray = Ray {
            origin: hr.p,
            direction: target,
        };
        Some(scattered_ray)
    }

    fn get_albedo(&self) -> &Color {
        &self.albedo
    }
}


impl Metal {
    fn scatter(&self, ray: &Ray, hr: HitRecord) -> Option<Ray> {
        let reflected = reflect(ray.direction.clone().unit_vector(), &hr.normal) + (random_in_sphere() * self.fuzz);

        match reflected.dot(&hr.normal) > 0.0 {
            true => Some(Ray {
                origin: hr.p,
                direction: reflected,
            }),
            false => None,
        }
    }

    fn get_albedo(&self) -> &Color {
        &self.albedo
    }
}

impl Dielectric {
    fn schlick(&self, cos: f32) -> f32 {
        let r0 = (1.0 - self.reflective_index) / (1.0 + self.reflective_index);
        let r0s = r0.powi(2);
        return r0s + (1.0 - r0s) * (1.0 - cos).powi(5)
    }

    fn refract(&self, uv: &Point, out_normal: Point, ni_over_nt: f32, cos: f32) -> Option<Point> {
        let dt = uv.dot(&out_normal);
        let discrim = 1.0 - (ni_over_nt.powi(2) * (1.0 - dt.powi(2)));

        let reflect_prob = self.schlick(cos);
        if reflect_prob > random::<f32>() {
            return None
        }
        else{
            if discrim > 0.0 {
                return Some(((uv.clone() - out_normal * dt) * ni_over_nt) - out_normal * discrim.sqrt());
            } else {
                return Some(((uv.clone() - out_normal * dt) * ni_over_nt) - out_normal * (-discrim).sqrt());
            }
        }
    }

}

impl Dielectric {
    fn scatter(&self, ray: &Ray, hr: HitRecord) -> Option<Ray> {

        let part_cos = ray.direction.dot(&hr.normal) / ray.direction.len();
        let (out_normal, ni_over_nt, cos) = {
            match ray.direction.dot(&hr.normal) > 0.0 {
                true => {
                    (hr.normal*-1.0, self.reflective_index, part_cos * self.reflective_index)
                },
                false => {
                    (hr.normal, 1.0 / self.reflective_index, -1.0 * part_cos)
                }
            }
        };

        let scattered_direction = {
            match self.refract(&ray.direction.unit_vector(), out_normal, ni_over_nt, cos) {
                Some(point) => point,
                None => reflect(ray.direction.clone(), &hr.normal)
            }
        };
        Some(Ray{origin: hr.p, direction: scattered_direction})
    }

    fn get_albedo(&self) -> &Color {
        &Color{r:1.0, g:1.0, b:1.0}
    }
}


