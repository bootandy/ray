extern  crate rand;
extern crate core;

use rand::random;
use std::cell::RefCell;
use std::f32;
use std::f32::consts::PI;
use std::fs::File;
use std::io::prelude::*;

use self::data::vec3::Vec3;
use self::data::ray::Ray;
use self::data::sphere::HittableObjects;
use data::sphere::Sphere;

pub mod data;

const NX :i32 = 200;
const NY :i32 = 100;
const ORIGIN: Vec3 = Vec3{x:0.0,y:0.0,z:0.0};
const HORIZ: Vec3 = Vec3{x:4.0,y:0.0,z:0.0};
const VERT: Vec3 = Vec3{x:0.0,y:2.0,z:0.0};
const LOW_LEFT: Vec3 = Vec3{x:-2.0,y:-1.0,z:-1.0};

const SPHERE: Vec3 = Vec3{x:0.0,y:0.0,z:-1.0};
const RADIUS :f32 = 0.5;


fn color(ray: &Ray, world : &HittableObjects) -> Vec3 {
    let hit_rec = world.hit_all(ray);
    match hit_rec {
        Some(hit) => {
            (hit.normal + 1.0) * 0.5
        },
        None => {
            let v = ray.direction.clone().unit_vector();
            let t = 0.5 * (v.y + 1.0);
            Vec3 { x: 1.0, y: 1.0, z: 1.0 } * (1.0 - t) + Vec3 { x: 0.5, y: 0.7, z: 1.0 } * t
        }
    }
}

fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    let mut buffer = File::create("out.ppm")?;
    buffer.write_all(format!("P3\n{} {}\n255\n", NX, NY).as_bytes())?;

    let s = Sphere {center: Vec3{x:0.0, y:0.0, z:-1.0}, radius: 0.5};
    let s2 = Sphere {center: Vec3{x:0.0, y:-100.5, z:-1.0}, radius: 100.0};
    let world = HittableObjects{
        objects: vec!(Box::new(s), Box::new(s2))
    };

    for y in (0..NY).rev() {
        for x in 0..NX {
            let r = x as f32 / NX as f32;
            let g = y as f32 / NY as f32;
            let ray = Ray{origin: ORIGIN, direction: LOW_LEFT + HORIZ*r + VERT*g};
            ray.point_at_parameter(2.0);
            let c = color(&ray, &world);
            buffer.write( c.as_pixel().as_bytes() );
        }
    }

    Ok(())
}

