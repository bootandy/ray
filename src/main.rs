
use std::fs::File;
use std::io::prelude::*;
use std::f32;
use std::f32::consts::PI;
use rand::{random, Rng};
use rayon::prelude::*;

use data::old_vec3::*;
use data::sphere::*;
use data::ray::Ray;
use data::world::*;

use rand::prng::XorShiftRng;
use rand::FromEntropy;


pub mod data;


#[macro_use]
extern crate derive_more;
extern crate rand;
extern crate rayon;


const BLACK: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
};
static UP:Point = Point{x:0.0, y: 1.0, z: 0.0};

fn color(rng: &mut XorShiftRng, r: &Ray, spheres: &HittableObjects, depth: u8) -> Color {
    match spheres.hit_all(r){
        Some(hit) => {
            if depth < 50 {
                let albedo = hit.material_hit.get_albedo();
                let scattered = hit.material_hit.scatter(rng, r, hit);
                if let Some(scatter_ray) = scattered {
                    let c = color(rng, &scatter_ray, spheres, depth+1);
                    return c.mul(albedo);
                }
            }
            return BLACK
        },
        None => {
            let ud = r.direction.unit_vector();
            let t = (ud.y + 1.0) * 0.5;
            let init_c = 1.0 - t;
            return Color{r:init_c, g: init_c, b:init_c} + Color{r:0.5*t, g:0.7*t, b:1.0*t}
        }
    }
}

fn random_in_unit_disk(rng : &mut impl Rng) -> Point {
    loop {
        let p = Point {
            x: rng.gen::<f32>() * 2.0 - 1.0,
            y: rng.gen::<f32>() * 2.0 - 1.0,
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
    fn get_ray(&self, rng: &mut impl Rng, s : f32, t: f32) -> Ray {
        let rd = random_in_unit_disk(rng) * self.lens_radius;
        let offset = self.u * rd.x + self.v *rd.y;
        let end = self.origin - offset;
        let d = self.lower_left.clone() + self.horizontal.clone()*s + self.vertical.clone()*t - end;
        Ray{origin:self.origin.clone()+ offset , direction:d}
    }
}

fn get_camera(look_from : Point, look_at : Point, up: Point, vfov: f32, aspect : f32, aperture: f32) -> Camera {
    let focus_dist = (look_from - look_at).len();
    let lens_radius = aperture / 2.0;
    let theta = vfov * PI / 180.0;
    let half_height = f32::tan(theta/2.0);
    let half_width = aspect * half_height;
    let w = (look_from - look_at).unit_vector();
    let u = (up.cross(&w)).unit_vector();
    let v = w.cross(&u);

    let lower_left = look_from - (u * half_width*focus_dist) - (v * half_height*focus_dist) - w*focus_dist;
    let horizontal = u * (2.0 * focus_dist * half_width);
    let vertical = v * (2.0 * focus_dist * half_height);
    Camera {
        lower_left,
        horizontal,
        vertical,
        origin:look_from,
        lens_radius,
        u,
        v,
    }
}


fn calc_pixel(data : (i32, i32, &Camera, &HittableObjects)) -> Color {
    let i = data.0;
    let j = data.1;
    let cam = data.2;
    let spheres = data.3;
    let mut rng = rand::prng::XorShiftRng::from_entropy();

    let mut col = Color{r:0.0, g:0.0, b:0.0};

    for _s in 0..NS {
        let u = (i as f32 + rng.gen::<f32>()) / NX as f32;
        let v = (j as f32 + rng.gen::<f32>()) / NY as f32;

        let ray = cam.get_ray(&mut rng, u, v);
        col += color(&mut rng, &ray, &spheres, 0);
    }
    col / NS as f32
}

const NX: i32 = 200;
const NY: i32 = 100;
const NS : i32 = 100;


fn main() -> std::io::Result<()> {
    println!("Rendering! x: {} y: {} {} times", NX, NY, NS);
    let mut buffer = File::create("out.ppm")?;
    buffer.write(format!("P3\n{} {}\n255\n", NX, NY).as_bytes())?;


//    let cam = get_camera(
//        Point{x:13.0, y: 2.0, z: 1.0},
//        Point{x:0.0, y: 0.5, z: 0.0},
//        UP,
//        10.0,
//        NX as f32 / NY as f32,
//        0.00001,
//    );
//    let spheres = build_world();

    let cam = get_camera(
        Point{x:0.0, y: 2.5, z: -10.0},
        Point{x:0.0, y: 0.5, z: -5.0},
        UP,
        60.0,
        NX as f32 / NY as f32,
        0.01,
    );
    let spheres = build_many();

    let mut to_calc = vec![];

    for j in (0..NY -1).rev() {
        for i in 0..NX {
            to_calc.push((i, j, &cam, &spheres));
        }
    }
    let pixels : Vec<Color> = to_calc.into_par_iter().map(calc_pixel).collect();
    for row in pixels {
        buffer.write(row.as_color_str().as_bytes())?;
    }
    buffer.flush()?;

    Ok(())
}

