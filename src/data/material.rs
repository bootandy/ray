use rand::random;

use Point;
use Color;
use Ray;



fn random_in_sphere() -> Point {
    loop {
        let c = Point{
            x:random::<f32>()*2.0 - 1.0,
            y:random::<f32>()*2.0 - 1.0,
            z:random::<f32>()*2.0 - 1.0,
        };
        if c.squared_length() <= 1.0 {
            return c
        }
    }
}

fn reflect(v: Point, n: &Point) -> Point{
    v.clone() - (*n * (2.0 * v.dot(&n)))
}

fn refract(v: &Point, n: Point, ni_over_nt: f32) -> Option<Point> {
    let uv = v.unit_vector();
    let dt = uv.dot(&n);
    let discrim = 1.0 - ni_over_nt*ni_over_nt*(1.0 - dt*dt);
    if discrim > 0.0 {
        let r = (uv - (n.clone() * dt)) - (n * discrim.sqrt());
        return Some(r * ni_over_nt)
    } else {
        return None
    }
}

fn schlick(cos: f32, ref_idx: f32) -> f32 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    return r0 + (1.0 - r0)* (1.0 - cos).powi(5)
}

pub struct Metal {
    pub albedo: Color,
}
pub struct Lambertian {
    pub albedo: Color,
}
pub struct Dielectric {
    pub reflective_index: f32,
}

pub enum Material {
    Metal(Metal),
    Lambertian(Lambertian),
    Dielectric(Dielectric),
}

impl Material {
    // pop this out without the match move to each subclass
    pub fn scatter(&self, r: &Ray, normal: Point, p: Point) -> Option<Ray> {
        match self {
            Material::Metal(_metal) => {
                let reflected = reflect(r.direction.unit_vector(), &normal);
                // hard code fuzz of 0.1
                let scattered = Ray{origin:p, direction:reflected + random_in_sphere()*0.1};
                if scattered.direction.dot(&normal) > 0.0 {
                    return Some(scattered);
                } 
                return None;
            },
            Material::Lambertian(_l) => {
                let target = normal + random_in_sphere();
                return Some(Ray{origin:p, direction:target});
            },
            Material::Dielectric(d) => {

                let (outward_normal, ni_over_nt, cos) = {
                    if r.direction.dot(&normal) > 0.0 {
                        let cos = d.reflective_index * r.direction.dot(&normal) / r.direction.len();
                        (normal.clone()*-1.0, d.reflective_index, cos)
                    } else {
                        let cos = -r.direction.dot(&normal) / r.direction.len();
                        (normal.clone(), 1.0 / d.reflective_index, cos)
                    }
                };
                match refract(&r.direction, outward_normal, ni_over_nt) {
                    Some(refracted) => {
                        let reflect_prob = schlick(cos, d.reflective_index);
                        if reflect_prob < random::<f32>() {
                            let reflected = reflect(r.direction.clone(), &normal);
                            return Some(Ray{origin:p, direction:reflected});
                        } else {
                            return Some(Ray{origin:p, direction:refracted});
                        }
                    },
                    None => {
                        let reflected = reflect(r.direction.clone(), &normal);
                        return Some(Ray{origin:p, direction:reflected});
                    },
                }
            }
        }
    }

    pub fn get_albedo(&self) -> &Color {
        match self {
            Material::Metal(metal) => {
                &metal.albedo
            },
            Material::Lambertian(l) => {
                &l.albedo
            },
            Material::Dielectric(_) => {
                &Color{r:1.0, g:1.0, b:1.0}
            },
        }
    }
}