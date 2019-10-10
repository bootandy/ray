extern  crate rand;
extern crate core;

use rand::random;
use std::cell::RefCell;
use std::f32;
use std::f32::consts::PI;
use std::fs::File;
use std::io::prelude::*;

use self::data::vec3::Vec3;
pub mod data;

const NX :i32 = 200;
const NY :i32 = 100;

fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    let mut buffer = File::create("out.ppm")?;
    buffer.write_all(format!("P3\n{} {}\n255\n", NX, NY).as_bytes())?;

    for y in 0..NY {
        for x in 0..NX {
            let r = (x as f32 / NX as f32);
            let g = (y as f32 / NY as f32);
            let b = 0.2;
            let s = format!("{:03} {:03} {:03}\n", (r*255.0) as u8 , (g*255.0) as u8 , (b*255.0) as u8);
            //println!("{:?}", s);
            buffer.write( s.as_bytes() );
        }
    }

    Ok(())
}

