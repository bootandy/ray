extern crate core;
extern crate itertools;
extern crate rand;

use rand::random;
use std::cell::RefCell;
use std::f32;
use std::fs::File;
use std::io::prelude::*;

use self::data::ray::Ray;
use self::data::sphere::HittableObjects;
use self::data::vec3::Vec3;
use data::world::build_world;

pub mod data;

// speed notes:
// 10 & 10 take ~ 8 seconds
// 100 & 10 ~ 90 seconds
// Swaping Box for & in the hit record made no difference
// Swaping Box for & in the sphere made no difference
// Switching to use Enums in the sphere made no difference

// before we did 100 & 5 in ~ 7.5 seconds


const NX: i32 = 200;
const NY: i32 = 100;
const NS: i32 = 100;
const MAX_BOUNCE: i32 = 5;

const ORIGIN: Vec3 = Vec3 {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};
const HORIZ: Vec3 = Vec3 {
    x: 4.0,
    y: 0.0,
    z: 0.0,
};
const VERT: Vec3 = Vec3 {
    x: 0.0,
    y: 2.0,
    z: 0.0,
};
const LOW_LEFT: Vec3 = Vec3 {
    x: -2.0,
    y: -1.0,
    z: -1.0,
};

const ZERO_VEC :Vec3 =  Vec3 {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};


fn get_ray(x: f32, y: f32) -> Ray {
    Ray {
        origin: ORIGIN,
        direction: LOW_LEFT + HORIZ * x + VERT * y,
    }
}

fn color(ray: &Ray, world: &HittableObjects, max_bounce: i32) -> Vec3 {
    if max_bounce == 0 {
        ZERO_VEC .clone()
    } else {
        let hit_rec = world.hit_all(ray);
        match hit_rec {
            Some(hit) => {
                let alb = hit.material_hit.get_albedo();
                let bounce = hit.material_hit.scatter(ray, hit);
                match bounce {
                    Some(scattered) => {
                        color(&scattered, world, max_bounce - 1) * alb
                    },
                    None => {
                        ZERO_VEC.clone()
                    },
                }
            }
            None => {
                let v = ray.direction.clone().unit_vector();
                let t = 0.5 * (v.y + 1.0);
                Vec3 {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                } * (1.0 - t)
                    + Vec3 {
                        x: 0.5,
                        y: 0.7,
                        z: 1.0,
                    } * t
            }
        }
    }
}

fn get_color_at(x: i32, y: i32, world: &HittableObjects) -> Vec3 {
    let r = (x as f32 + random::<f32>()) / NX as f32;
    let g = (y as f32 + random::<f32>()) / NY as f32;
    let ray = get_ray(r, g);
    color(&ray, world, MAX_BOUNCE)
}

fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    let mut buffer = File::create("out.ppm")?;
    buffer.write_all(format!("P3\n{} {}\n255\n", NX, NY).as_bytes())?;
    let world = build_world();

    for y in (0..NY).rev() {
        for x in 0..NX {

            let mut color_sum = Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
            for _ in 0..NS {
                color_sum = color_sum + get_color_at(x, y, &world)
            }
            color_sum = color_sum / NS as f32;
            buffer.write(color_sum.as_pixel().as_bytes());
        }
    }

    Ok(())
}
