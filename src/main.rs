use std::io::prelude::*;
use std::fs::File;
use std::f32;
use std::f32::consts::PI;
use rand::random;

use self::data::ray::Ray;
use self::data::vec::Point;
use self::data::shapes::{Sphere, SphereList};
use self::data::materials::{Material, Metal, Lambertian, Dielectric };

pub mod data;

const ORIGIN: Point = Point{x:0.0, y:0.0, z:0.0};
const CENTER: Point = Point{x:0.0, y:0.0, z:-1.0};


const LOWER_LEFT_CORNER: Point = Point{x:-2.0, y:-1.0, z:-1.0};
const HORIZONTAL: Point = Point{x:4.0, y:0.0, z:0.0};
const VERTICAL: Point = Point{x:0.0, y:2.0, z:0.0};

fn get_simple_camera() -> Camera {
    Camera{
        lower_left_corner:LOWER_LEFT_CORNER, 
        horizontal:Point{x:4.0, y:0.0, z:0.0}, 
        vertical:Point{x:0.0, y:2.0, z:0.0}, 
        origin:ORIGIN,
    }
}

fn get_camera(
    look_from: Point, look_at: Point, vup: Point, vfov: f32, aspect: f32
) -> Camera {
    let theta = vfov * PI / 180.0;
    let half_height = (theta/2.0).tan();
    let half_width = aspect * half_height;
    let w = look_from.sub(&look_at).unit_vector();
    let u = vup.cross(&w).unit_vector();
    let v = w.cross(&u);
    let lower_left_corner = look_from.sub(&u.flat_mul(half_width)).sub(&v.flat_mul(half_height)).sub(&w);
    let horizontal = u.flat_mul(2.0 * half_width);
    let vertical = v.flat_mul(2.0 * half_height);

    Camera{origin:look_from, lower_left_corner, horizontal, vertical}
}

struct Camera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: Point,
    vertical: Point,
}

impl Camera {
    fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray{
            origin: self.origin.clone(),
            direction: self.lower_left_corner.add(
                &self.horizontal.flat_mul(u).add(
                    &self.vertical.flat_mul(v).sub(&self.origin)
                )
            )
        }
    }
}



fn color(ray: &Ray, sphere_list: &SphereList, depth :u8) -> Point {
    match sphere_list.hit(ray, 0.001, f32::MAX) {
        Some(rec) => {
            if depth < 5 { //edit from 50
                match rec.material.scatter(ray, &rec) {
                    Some(scattered_ray) => {
                        return color(&scattered_ray, sphere_list, depth+1).mul(rec.material.get_albedo());
                    },
                    None => {},
                }
            }
            return Point{x:0.0, y:0.0, z:0.0};
        },
        None => {
            let unit_direction = &ray.direction;
            let t = 0.5 * (unit_direction.y + 1.0);
            return Point{x:1.0, y:1.0, z:1.0}.flat_mul(1.0 - t).add(
                &Point{x:0.5, y:0.7, z:1.0}.flat_mul(t)
            );
        }
    }
}

fn get_spheres() -> SphereList {

    let s =  Sphere{center:Point{x:0.9, y:0.0, z:-1.0}, radius:0.5, material:Box::new(Lambertian{albedo:Point{x:0.8, y:0.3, z:0.3}})};

    let mut c :Vec<Sphere> = vec![];
    c.push(s);
    c.push(Sphere{center:Point{x:-3.0, y:0.0, z:-2.0}, radius:0.5, material:Box::new(Metal{albedo:Point{x:0.8, y:0.6, z:0.2}, fuzz:0.0})});

    c.push(Sphere{center:Point{x:0.0, y:-100.5, z:-1.0}, radius:100.0, material:Box::new(Lambertian{albedo:Point{x:0.8, y:0.8, z:0.0}})});
    return SphereList{spheres:c};
}

fn get_spheres_many() -> SphereList {
    let mut v : Vec<Sphere> = vec![];

    v.push( Sphere{center:Point{x:-0.0, y:-1000.0, z:0.0}, radius:1000.0, material:Box::new(Lambertian{albedo:Point{x:0.5, y:0.5, z:0.5}})});
    v.push( Sphere{center:Point{x:-1.5, y:1.0, z:-0.5}, radius:1.0, material:Box::new( Lambertian{albedo:Point{x:0.4, y:0.2, z:0.1}})});
    v.push( Sphere{center:Point{x:1.3, y:1.0, z:0.7}, radius:1.0, material:Box::new( Metal{albedo:Point{x:0.7, y:0.6, z:0.5}, fuzz:0.0})});

    // for a in -5..5 {
    //     for b in -5..5 {
    //         let center = Point{x:a as f32 + 0.9 * random::<f32>(), y:0.2, z:b as f32 + 0.9*random::<f32>()};
    //         let sphere = match random::<f32>() {
    //             x if x < 0.8 => {
    //                 Sphere{center:center, radius:0.2, material:Box::new(Lambertian{albedo:Point{x:random(), y:random(), z:random()}})}
    //             },
    //             _ => {
    //                 Sphere{center:center, radius:0.2, material:Box::new(Metal{albedo:Point{x:random(), y:random(), z:random()}, fuzz:0.0})}
    //             },
    //         };
    //         v.push(sphere);
    //     }
    // }
    
    return SphereList{spheres:v}
}


fn main() -> std::io::Result<()>  {
    println!("building!");
    let mut buffer = File::create("out.ppm")?;
    let nx = 400 as u16;
    let ny = 200 as u16;
    let n_anti_alias = 10 as u8;
    buffer.write(format!("P3\n{} {}\n255\n", nx, ny).as_bytes())?;
    let cam = get_camera(
        Point{x:0.0, y:0.5, z:6.0},
        Point{x:0.0, y:0.5, z:-2.0},
        Point{x:0.0, y:1.0, z:0.0},
        90.0,
        nx as f32 / ny as f32,
    );
    let cam = get_simple_camera();
    let sphere_list = get_spheres();
    println!("spheres done!");

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Point{x:0.0, y:0.0, z:0.0};
            for _s in 0..n_anti_alias {
                let u = (i as f32 + random::<f32>()) /  nx as f32;
                let v = (j as f32 + random::<f32>()) /  ny as f32;
                let r = cam.get_ray(u, v);
                col = col.add(&color(&r, &sphere_list, 0));
            }
            col = col.flat_div(n_anti_alias as f32);
            // Gamma correction for earlier dropping color value on ray bounces:
            col = Point{x:col.x.sqrt(), y:col.y.sqrt(), z:col.z.sqrt()};
            let ir = (255.99 * col.x) as u8;
            let ig = (255.99 * col.y) as u8;
            let ib = (255.99 * col.z) as u8;
            buffer.write(format!("{} {} {}\n", ir, ig, ib).as_bytes())?;
        }
    };
    Ok(())
}
