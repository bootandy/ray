use std::fs::File;
use std::io::prelude::*;
use std::f32;
use std::f32::consts::PI;
use rand::random;
use rayon::prelude::*;

use data::vec3::*;
use data::material::*;
use data::sphere::*;
use data::ray::Ray;

pub mod data;


#[macro_use]
extern crate derive_more;
extern crate rand;
extern crate rayon;

static mut counter: u64 = 0;

fn color(r: &Ray, spheres: &SphereList, depth: u8) -> Color {
    unsafe {
        counter += 1;
    }
    match spheres.hit(r, 0.001, f32::MAX){
        Some(hit) => {
            if depth < 50 {
                let scattered = hit.material.scatter(r, hit.normal.clone(), hit.p);
                if let Some(scatter_ray) = scattered {
                    let albedo = hit.material.get_albedo();
                    let c = color(&scatter_ray, spheres, depth+1);
                    return c.mul(albedo);
                }
            }
            return Color{r:0.0, g:0.0, b:0.0};
        },
        None => {
            let ud = r.direction.unit_vector();
            let t = (ud.y + 1.0) * 0.5;
            let init_c = 1.0 - t;
            return Color{r:init_c, g: init_c, b:init_c} + Color{r:0.5*t, g:0.7*t, b:1.0*t}
        }
    }
}

fn random_in_unit_disk() -> Point {
    loop {
        let p = Point{
            x: random::<f32>()*2.0 - 1.0, 
            y: random::<f32>()*2.0 - 1.0,
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
    fn get_ray(&self, s : f32, t: f32) -> Ray {
        let rd = random_in_unit_disk() * self.lens_radius;
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
        lower_left:lower_left,
        horizontal:horizontal,
        vertical:vertical,
        origin:look_from,
        lens_radius: lens_radius,
        u: u,
        v: v,
    }
}

fn get_old_spheres() -> SphereList {
    return SphereList{spheres: vec![
        Sphere{center:Point{x:3.0, y:0.0, z:0.5}, radius:0.5, material:Material::Lambertian(Lambertian{albedo:Color{r:0.1, g:0.2, b:0.5}})},
        Sphere{center:Point{x:0.0, y:-100.5, z:0.0}, radius:100.0, material:Material::Lambertian(Lambertian{albedo:Color{r:0.8, g:0.8, b:0.0}})},
        Sphere{center:Point{x:2.0, y:0.0, z:-0.5}, radius:0.5, material:Material::Metal(Metal{albedo:Color{r:0.8, g:0.6, b:0.2}})},
        Sphere{center:Point{x:1.0, y:0.0, z:1.0}, radius:0.5, material:Material::Dielectric(Dielectric{reflective_index:1.5})},
        Sphere{center:Point{x:1.0, y:0.0, z:1.0}, radius:-0.45, material:Material::Dielectric(Dielectric{reflective_index:1.5})},
    ]};
}

fn get_spheres_many() -> SphereList {
    let mut v : Vec<Sphere> = vec![];
 
    v.push( Sphere{center:Point{x:-0.0, y:-1000.0, z:0.0}, radius:1000.0, material:Material::Lambertian(Lambertian{albedo:Color{r:0.5, g:0.5, b:0.5}})});
    v.push( Sphere{center:Point{x:4.0, y:0.7, z:0.0}, radius:0.7, material:Material::Dielectric( Dielectric{reflective_index:1.5}) });
    v.push( Sphere{center:Point{x:0.0, y:1.0, z:0.0}, radius:1.0, material:Material::Dielectric( Dielectric{reflective_index:1.5}) });
    v.push( Sphere{center:Point{x:-4.0, y:1.0, z:0.0}, radius:1.0, material:Material::Metal( Metal{albedo:Color{r:0.7, g:0.6, b:0.5} })});
 
    for a in -7..7 {
        for b in -7..7 {
            let center = Point{x:a as f32 + 0.9 * random::<f32>(), y:0.2, z:b as f32 + 0.9*random::<f32>()};
            let sphere = match random::<f32>() {
                x if x < 0.7 => {
                    Sphere{center:center, radius:0.2, material:Material::Lambertian(Lambertian{albedo:Color{r:random(), g:random(), b:random()}})}
                },
                x if x < 0.85 => {
                    Sphere{center:center, radius:0.2, material:Material::Metal(Metal{albedo:Color{r:random(), g:random(), b:random()}})}
                },
                _ => {
                    Sphere{center:center, radius:0.2, material:Material::Dielectric(Dielectric{reflective_index:1.5})}
                },
            };
            v.push(sphere);
        }
    }
    
    return SphereList{spheres:v}
}

fn calc_pixel(data : &(i32, i32, &Camera, &SphereList)) -> Color {
    let i = data.0;
    let j = data.1;
    let cam = data.2;
    let spheres = data.3;
    let mut col = Color{r:0.0, g:0.0, b:0.0};

    for _s in 0..ns {
        let u = (i as f32 + random::<f32>()) / nx as f32;
        let v = (j as f32 + random::<f32>()) / ny as f32;

        let ray = cam.get_ray(u, v);
        col += color(&ray, &spheres, 0);
    }
    col / ns as f32
}

const nx : i32 = 1200;
const ny : i32 = 800;
const ns : i32 = 100;

fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    let mut buffer = File::create("out.ppm")?;
    buffer.write(format!("P3\n{} {}\n255\n", nx, ny).as_bytes())?;

    let cam = get_camera(
        Point{x:13.0, y: 2.0, z: 3.0},
        Point{x:0.0, y: 0.5, z: 0.0},
        Point{x:0.0, y: 1.0, z: 0.0},
        10.0,
        nx as f32 / ny as f32,
        0.01,
    );
    let spheres = get_spheres_many();
    //let spheres = get_old_spheres();

    let mut to_calc = vec![];

    for j in (0..ny-1).rev() {
        for i in 0..nx {
            to_calc.push((i, j, &cam, &spheres));
        }
    }
    let pixels : Vec<Color> = to_calc.par_iter().map(calc_pixel).collect();
    // sort pixels
    for row in pixels {
        buffer.write(row.as_color_str().as_bytes())?;
    }
    buffer.flush()?;

    unsafe {
        println!("{:?}", counter);
    }
    Ok(())
}
