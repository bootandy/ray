
use rand::random;

use super::vec::Point;
use super::ray::Ray;
use super::shapes::HitableRecord;
use super::super::ORIGIN;

const GLASS_ALBEDO: Point = Point{x:1.0, y:1.0, z:1.0};

fn get_rnd() -> f32 {
    random::<f32>() * 2.0 - 1.0
}

fn random_in_sphere() -> Point {
    let mut p = Point{x:1.0, y:1.0, z:1.0};
    while p.squared_length() >= 1.0 {
        p = Point{x:get_rnd(), y:get_rnd(), z:get_rnd()}
    }
    p
}


pub trait Material {
    fn scatter(&self, r: &Ray, rec: &HitableRecord) -> Option<Ray>;
    fn get_albedo(&self) -> &Point;
}

pub struct Lambertian {
    pub albedo: Point,
}

impl Material for Lambertian {
    fn scatter(&self, _r: &Ray, rec: &HitableRecord) -> Option<Ray> {
        let target = rec.p.add(&rec.normal).add(&random_in_sphere());
        let scattered = Ray{origin:rec.p.clone(), direction:target.sub(&rec.p)};
        return Some(scattered);
    }
    fn get_albedo(&self) -> &Point {&self.albedo}
}

fn reflect(v: &Point, n: &Point) -> Point {
    v.sub(&n.flat_mul(2.0 * v.dot(&n)))
}

pub struct Metal {
    pub albedo: Point,
    pub fuzz: f32, // suggest less than .5 and ideally 0.
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, rec: &HitableRecord) -> Option<Ray> {
        assert!(self.fuzz <= 1.0);
        let reflected = reflect(&r.direction.unit_vector(), &rec.normal);
        let scattered = Ray{origin:rec.p.clone(), direction:reflected.add(&random_in_sphere().flat_mul(self.fuzz))};
        if scattered.direction.dot(&rec.normal) > 0.0 {
            return Some(scattered)
        } else {
            return None
        }
    }
    fn get_albedo(&self) -> &Point {&self.albedo}
}

// Glass / Water etc:
pub struct Dielectric {
    pub ref_idx: f32,
}

impl Dielectric {
    fn schlick(&self, cosine: f32) -> f32 {
        let r0 = (1.0-self.ref_idx) / (1.0+self.ref_idx);
        let r1 = r0 * r0;
        return r1 + (1.0 - r1) * ((1.0 - cosine).powf(5.0))
    }
    // doesn't seem to work correctly
    fn refract(&self, v: &Point, n: &Point, ni_over_nt: f32) -> Option<Point> {
        let uv = v.unit_vector();
        let dt = uv.dot(n);
        let discim = 1.0 - (ni_over_nt*ni_over_nt * (1.0 - (dt*dt)));
        if discim > 0.0 {
            let to_sub = n.flat_mul(discim.sqrt());
            return Some(uv.sub(&n.flat_mul(dt)).flat_mul(ni_over_nt).sub(&to_sub));
        }
        return None
    }
}

impl Material for Dielectric {
    fn scatter(&self, r: &Ray, rec: &HitableRecord) -> Option<Ray> {
        let reflected = reflect(&r.direction, &rec.normal);

        let (outward_normal, ni_over_nt, cosine) = {
            if r.direction.dot(&rec.normal) > 0.0 {
                let cos = r.direction.dot(&rec.normal) * (self.ref_idx / 3.0);
                (ORIGIN.sub(&rec.normal), self.ref_idx, cos)
            }
            else {
                let cos = -r.direction.dot(&rec.normal) / 3.0;
                (rec.normal.clone(), 1.0 / self.ref_idx, cos)
            }
        };

        match self.refract(&r.direction, &outward_normal, ni_over_nt){
            Some(refracted) => {
                let reflect_prob = self.schlick(cosine);
                if random::<f32>() < reflect_prob {
                    return Some(Ray{origin:rec.p.clone(), direction:reflected});
                } else {
                    return Some(Ray{origin:rec.p.clone(), direction:refracted});
                }
            },
            None => {
                return Some(Ray{origin:rec.p.clone(), direction:reflected});
            }
        }
    }
    fn get_albedo(&self) -> &Point {&GLASS_ALBEDO}
}
