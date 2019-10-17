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
use data::world::build_world;
use std::f64::consts::PI;

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
const MAX_BOUNCE: i32 = 5;

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

struct Camera {
    low_left: Point,
    horiz: Point,
    vert: Point,
    origin: Point,
}

impl Camera {
    fn new(look_from: Point, look_at: Point, vfov: f32, aspect: f32) -> Camera {
        let theta = vfov * PI as f32 / 180.0;
        let half_height = (theta/2.0).tan();
        let half_width = aspect * half_height;
        let origin = look_from;
        let w = (look_from - look_at).unit_vector();
        let u = UP.cross(&w).unit_vector();
        let v = w.cross(&u);
        let low_left = ORIGIN - u*half_width - v*half_height - w;
        let horiz = u * 2.0 * half_width;
        let vert = v * 2.0 * half_height;
        Camera{low_left, horiz, vert, origin}
    }

    fn get_ray(&self, x: f32, y: f32) -> Ray {
        Ray {
            origin: self.origin.clone(),
            direction: self.low_left + self.horiz * x + self.vert * y,
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
    let world = build_world();
    let cam = Camera::new(Point{x:-2.0, y:2.0, z:1.0}, Point{x:0.0, y:0.0, z:-1.0}, 30.0, NX as f32/NY as f32);

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
