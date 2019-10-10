use rand::random;
use rayon::prelude::*;
use std::cell::RefCell;
use std::f32;
use std::f32::consts::PI;
use std::fs::File;
use std::io::prelude::*;


fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    let mut buffer = File::create("out.ppm")?;
    buffer.write_all(format!("P3\n{} {}\n255\n", NX, NY).as_bytes())?;

    let cam = get_camera(
        Point {
            x: 278.0,
            y: 278.0,
            z: -800.0,
        },
        Point {
            x: 278.0,
            y: 278.0,
            z: 0.0,
        },
        Point {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        40.0,
        NX as f32 / NY as f32,
        0.0,
        0.0,
        1.0,
    );
}

