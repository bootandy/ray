use rand::random;
use rayon::prelude::*;
use std::cell::RefCell;
use std::f32;
use std::fs::File;
use std::io::prelude::*;

use data::bounding::*;
use data::material::*;
use data::ray::Ray;
use data::sphere::*;
use data::textures::*;
use data::vec3::*;
use data::camera::*;
use data::layouts::sphere_layout::*;

pub mod data;

#[macro_use]
extern crate derive_more;
extern crate image;
extern crate rand;
extern crate rayon;

#[macro_use]
extern crate lazy_static;

/// Remove randomness for reproducable builds when timing speed
pub fn rnd() -> f32 {
    random::<f32>()
    //0.4
}

fn color(r: &Ray, bound_box: &BvhBox, depth: u8) -> Color {
    if depth >= 50 {
        return NO_COLOR;
    }

    match bound_box.dig(r, f32::MAX) {
        Some(hit) => {
            let c = color(&hit.scattered_ray, bound_box, depth + 1);
            c.mul(&hit.color)
        },
        None => {
            let ud = r.direction.unit_vector();
            let t = (ud.y + 1.0) * 0.5;
            let init_c = 1.0 - t;
            Color {
                r: init_c,
                g: init_c,
                b: init_c,
            } + Color {
                r: 0.5 * t,
                g: 0.7 * t,
                b: 1.0 * t,
            }
        }
    }
}

fn calc_pixel(data: &(i32, i32, &Camera), bvh_box: &mut BvhBox) -> Color {
    let i = data.0;
    let j = data.1;
    let cam = data.2;
    let mut col = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
    };

    for _s in 0..NS {
        let u = (i as f32 + rnd()) / NX as f32;
        let v = (j as f32 + rnd()) / NY as f32;

        let ray = cam.get_ray(u, v);
        col += color(&ray, bvh_box, 0);
    }
    col / NS as f32
}

fn spheres_to_bounding_box(spheres: Vec<SphereThing>) -> BvhBox {
    let mut bounds = vec![];
    for a in spheres {
        let mut b = a.bounding_box();
        bounds.push(BvhLeaf { boxx: b, has_a: a });
    }
    get_bvh_box(&mut bounds)
}

const NX: i32 = 800;
const NY: i32 = 400;
const NS: i32 = 100;

fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    let mut buffer = File::create("out.ppm")?;
    buffer.write_all(format!("P3\n{} {}\n255\n", NX, NY).as_bytes())?;

    let cam = get_camera(
        Point {
            x: 13.0,
            y: 2.0,
            z: 3.0,
        },
        Point {
            x: 0.0,
            y: 0.5,
            z: 0.0,
        },
        Point {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        15.0,
        NX as f32 / NY as f32,
        0.01,
        0.0,
        1.0,
    );

    //let spherelist = get_spheres_many();
    let spherelist = get_old_spheres();

    let bound_box = spheres_to_bounding_box(spherelist.spheres.clone());

    let mut to_calc = vec![];

    for j in (0..NY - 1).rev() {
        for i in 0..NX {
            to_calc.push((i, j, &cam));
        }
    }
    println!("Built boxes");

    // Do fancy thread local storage of the BVH boxes
    thread_local!(static STORE: RefCell<Option<BvhBox>> = RefCell::new(None));

    let pixels: Vec<Color> = to_calc
        .par_iter()
        .map(|a| {
            STORE.with(|bvh| {
                let mut local_bvh = bvh.borrow_mut();
                if local_bvh.is_none() {
                    *local_bvh = Some(bound_box.clone());
                }
                calc_pixel(a, local_bvh.as_mut().unwrap())
            })
        }).collect();

    for row in pixels {
        buffer.write_all(row.as_color_str().as_bytes())?;
    }
    buffer.flush()?;

    Ok(())
}
