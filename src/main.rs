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
pub mod data;

const NX :i32 = 200;
const NY :i32 = 100;
const ORIGIN: Vec3 = Vec3{x:0.0,y:0.0,z:0.0};
const HORIZ: Vec3 = Vec3{x:4.0,y:0.0,z:0.0};
const VERT: Vec3 = Vec3{x:0.0,y:2.0,z:0.0};
const LOW_LEFT: Vec3 = Vec3{x:-2.0,y:-1.0,z:-1.0};

const SPHERE: Vec3 = Vec3{x:0.0,y:0.0,z:-1.0};
const RADIUS :f32 = 0.5;


fn hit_sphere(ray: &Ray) -> bool {
    let oc = ray.origin.clone() - SPHERE;
    let a = ray.direction.dot(&ray.direction);
    let b = 2.0 * oc.dot(&ray.direction);
    let c = - (RADIUS * RADIUS) + oc.dot(&oc);
    return (b*b - (a*c*4.0)) > 0.0
}

fn color(ray: &Ray) -> Vec3 {
    if hit_sphere(ray){
        Vec3{x:1.0, y:0.0, z:0.0}
    } else {
        let v = ray.direction.clone().unit_vector();
        let t = 0.5 * (v.y + 1.0);
        Vec3 { x: 1.0, y: 1.0, z: 1.0 } * (1.0 - t) + Vec3 { x: 0.5, y: 0.7, z: 1.0 } * t
    }
}

fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    let mut buffer = File::create("out.ppm")?;
    buffer.write_all(format!("P3\n{} {}\n255\n", NX, NY).as_bytes())?;

    for y in 0..NY {
        for x in 0..NX {
            let r = x as f32 / NX as f32;
            let g = y as f32 / NY as f32;
            let ray = Ray{origin: ORIGIN, direction: LOW_LEFT + HORIZ*r + VERT*g};
            let c = color(&ray);
            buffer.write( c.as_pixel().as_bytes() );
        }
    }

    Ok(())
}

