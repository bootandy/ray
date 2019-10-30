use data::old_vec3::Color;
use data::old_vec3::Point;
use data::ray::Ray;
use data::sphere::HitRecord;
use rand::{Rng, XorShiftRng};
use std::boxed::Box;

pub trait Material: Send + Sync {
    fn scatter(&self, rng: &mut XorShiftRng, ray: &Ray, hr: HitRecord) -> Option<Ray>;
    fn get_albedo(&self) -> &Color;
}

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
pub struct Dielectric {
    pub reflective_index: f32,
}

fn random_in_sphere(rng: &mut impl Rng) -> Point {
    loop {
        let p = Point {
            x: rng.gen::<f32>() * 2.0 - 1.0,
            y: rng.gen::<f32>() * 2.0 - 1.0,
            z: rng.gen::<f32>() * 2.0 - 1.0,
        };
        if p.squared_length() < 1.0 {
            return p;
        }
    }
}

fn reflect(v: Point, n: &Point) -> Point {
    v - (*n * v.dot(n) * 2.0)
}

impl Material for Lambertian {
    fn scatter(&self, rng: &mut XorShiftRng, ray: &Ray, hr: HitRecord) -> Option<Ray> {
        let target = hr.normal + random_in_sphere(rng);
        let scattered_ray = Ray {
            origin: hr.p,
            direction: target,
            time: ray.time,
        };
        Some(scattered_ray)
    }

    fn get_albedo(&self) -> &Color {
        &self.albedo
    }
}

impl Material for Metal {
    fn scatter(&self, rng: &mut XorShiftRng, ray: &Ray, hr: HitRecord) -> Option<Ray> {
        let reflected = reflect(ray.direction.clone().unit_vector(), &hr.normal)
            + (random_in_sphere(rng) * self.fuzz);

        match reflected.dot(&hr.normal) > 0.0 {
            true => Some(Ray {
                origin: hr.p,
                direction: reflected,
                time: ray.time,
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
        return r0s + (1.0 - r0s) * (1.0 - cos).powi(5);
    }

    fn refract(
        &self,
        rng: &mut XorShiftRng,
        uv: &Point,
        out_normal: Point,
        ni_over_nt: f32,
        cos: f32,
    ) -> Option<Point> {
        let dt = uv.dot(&out_normal);
        let discrim = 1.0 - (ni_over_nt.powi(2) * (1.0 - dt.powi(2)));

        let reflect_prob = self.schlick(cos);
        if reflect_prob > rng.gen::<f32>() {
            return None;
        } else if discrim > 0.0 {
            return Some(
                ((uv.clone() - out_normal * dt) * ni_over_nt) - out_normal * discrim.sqrt(),
            );
        } else {
            return Some(
                ((uv.clone() - out_normal * dt) * ni_over_nt) - out_normal * (-discrim).sqrt(),
            );
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, rng: &mut XorShiftRng, ray: &Ray, hr: HitRecord) -> Option<Ray> {
        let part_cos = ray.direction.dot(&hr.normal) / ray.direction.len();

        let (out_normal, ni_over_nt, cos) = {
            if ray.direction.dot(&hr.normal) > 0.0 {
                (
                    hr.normal * -1.0,
                    self.reflective_index,
                    part_cos * self.reflective_index,
                )
            } else {
                (hr.normal, 1.0 / self.reflective_index, -1.0 * part_cos)
            }
        };

        let scattered_direction = {
            match self.refract(
                rng,
                &ray.direction.unit_vector(),
                out_normal,
                ni_over_nt,
                cos,
            ) {
                Some(point) => point,
                None => reflect(ray.direction.clone(), &hr.normal),
            }
        };
        Some(Ray {
            origin: hr.p,
            direction: scattered_direction,
            time: ray.time,
        })
    }

    fn get_albedo(&self) -> &Color {
        &Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        }
    }
}
