extern crate core;
extern crate itertools;

#[macro_use]
extern crate derive_more;
extern crate rand;

use rand::random;
use std::cell::RefCell;
use std::f32;
use std::fs::File;
use std::io::prelude::*;

use self::data::old_vec3::Color;
use self::data::old_vec3::Point;
use self::data::ray::Ray;
use self::data::sphere::HittableObjects;
use data::world::*;
use std::f32::consts::PI;

pub mod data;

// speed notes:
// 10 & 10 take ~ 8 seconds
// 100 & 10 ~ 90 seconds
// Swaping Box for & in the hit record made no difference
// Swaping Box for & in the sphere made no difference
// Switching to use Enums in the sphere made no difference
// Switching to use old vec class. 10 10  no difference

// before we did 100 & 5 in ~ 7.5 seconds

const NX: i32 = 200;
const NY: i32 = 100;
const NS: i32 = 100;
const MAX_BOUNCE: i32 = 50;

const ORIGIN: Point = Point {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};
const UP: Point = Point {
    x: 0.0,
    y: 1.0,
    z: 0.0,
};
const BLACK: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
};
const APERTURE: f32 = 0.001;

fn random_in_unit_disk() -> Point {
    loop {
        let p = Point {
            x: random::<f32>() * 2.0 - 1.0,
            y: random::<f32>() * 2.0 - 1.0,
            z: 0.0,
        };
        if p.dot(&p) < 1.0 {
            return p;
        }
    }
}

struct Camera {
    origin: Point,
    lower_left: Point,
    horizontal: Point,
    vertical: Point,
    u: Point,
    v: Point,
    lens_radius: f32,
}

impl Camera {
    fn get_ray(&self, x: f32, y: f32) -> Ray {
        let rd = random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        Ray {
            origin: offset + self.origin,
            direction: self.lower_left + self.horizontal * x + self.vertical * y - self.origin - offset,
        }
    }

    fn new(look_from: Point, look_at: Point, vfov: f32, aspect: f32, aperture: f32) -> Camera {
        let focus_dist = (look_from - look_at).len();
        let lens_radius = aperture / 2.0;
        let theta = vfov * PI / 180.0;
        let half_height = f32::tan(theta / 2.0);
        let half_width = aspect * half_height;
        let w = (look_from - look_at).unit_vector();
        let u = (UP.cross(&w)).unit_vector();
        let v = w.cross(&u);

        let lower_left = look_from - (u * half_width * focus_dist) - (v * half_height * focus_dist) - w * focus_dist;
        let horizontal = u * (2.0 * focus_dist * half_width);
        let vertical = v * (2.0 * focus_dist * half_height);
        Camera {
            lower_left,
            horizontal,
            vertical,
            origin: look_from,
            lens_radius,
            u,
            v,
        }
    }
}


fn color(ray: &Ray, world: &HittableObjects, max_bounce: i32) -> Color {
    if max_bounce == 0 {
        BLACK
    } else {
        let hit_rec = world.hit_all(ray);
        match hit_rec {
            Some(hit) => {
                let alb = hit.material_hit.get_albedo();
                let bounce = hit.material_hit.scatter(ray, hit);
                match bounce {
                    Some(scattered) => color(&scattered, world, max_bounce - 1).mul(alb),
                    None => BLACK
                }
            }
            None => {
                let v = ray.direction.unit_vector();
                let t = 0.5 * (v.y + 1.0);
                Color {
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                } * (1.0 - t)
                    + Color {
                        r: 0.5,
                        g: 0.7,
                        b: 1.0,
                    } * t
            }
        }
    }
}

fn get_color_at(x: i32, y: i32, camera: &Camera, world: &HittableObjects) -> Color {
    let r = (x as f32 + random::<f32>()) / NX as f32;
    let g = (y as f32 + random::<f32>()) / NY as f32;
    let ray = camera.get_ray(r, g);
    color(&ray, world, MAX_BOUNCE)
}

fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    let mut buffer = File::create("out.ppm")?;
    buffer.write_all(format!("P3\n{} {}\n255\n", NX, NY).as_bytes())?;

    //let world = build_world();
    let world = build_many();

    let cam = Camera::new(
        Point{x:0.0, y: 3.0, z: 10.0},
        Point{x:0.0, y: 0.0, z: -1.0},
        90.0,
        NX as f32 / NY as f32,
        APERTURE,
    );

    for y in (0..NY).rev() {
        for x in 0..NX {
            let mut color_sum = BLACK.clone();
            for _ in 0..NS {
                color_sum = color_sum + get_color_at(x, y, &cam, &world)
            }
            color_sum = color_sum / NS as f32;
            buffer.write(color_sum.as_color_str().as_bytes()).unwrap();
        }
    }

    Ok(())
}
