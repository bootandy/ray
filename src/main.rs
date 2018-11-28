use rand::random;
use rayon::prelude::*;
use std::cell::RefCell;
use std::f32;
use std::f32::consts::PI;
use std::fs::File;
use std::io::prelude::*;

use data::material::*;
use data::ray::Ray;
use data::sphere::*;
use data::vec3::*;

pub mod data;

#[allow(dead_code)]
#[macro_use]
extern crate derive_more;
extern crate rand;
extern crate rayon;
//#[macro_use]
//extern crate itertools;

fn color(r: &Ray, bound_box: &BvhBox, depth: u8) -> Color {
    if depth >= 50 {
        return Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        };
    }

    match bound_box.dig(r) {
        Some(hit) => {
            let scattered = hit.material.scatter(r, hit.normal.clone(), hit.p);
            if let Some(scatter_ray) = scattered {
                let albedo = hit.material.get_albedo();
                let c = color(&scatter_ray, bound_box, depth + 1);
                return c.mul(albedo);
            }
            return Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
            };
        }
        None => {
            let ud = r.direction.unit_vector();
            let t = (ud.y + 1.0) * 0.5;
            let init_c = 1.0 - t;
            return Color {
                r: init_c,
                g: init_c,
                b: init_c,
            } + Color {
                r: 0.5 * t,
                g: 0.7 * t,
                b: 1.0 * t,
            };
        }
    }
}

fn random_in_unit_disk() -> Point {
    loop {
        let p = Point {
            x: random::<f32>() * 2.0 - 1.0,
            y: random::<f32>() * 2.0 - 1.0,
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
    time1: f32,
    time0: f32,
    lens_radius: f32,
}

impl Camera {
    fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        let end = self.origin - offset;
        let time = self.time0 + (random::<f32>() * (self.time1 - self.time0));
        let d =
            self.lower_left.clone() + self.horizontal.clone() * s + self.vertical.clone() * t - end;
        Ray {
            origin: self.origin.clone() + offset,
            direction: d,
            time: time,
        }
    }
}

fn get_camera(
    look_from: Point,
    look_at: Point,
    up: Point,
    vfov: f32,
    aspect: f32,
    aperture: f32,
    t0: f32,
    t1: f32,
) -> Camera {
    let focus_dist = (look_from - look_at).len();
    let lens_radius = aperture / 2.0;
    let theta = vfov * PI / 180.0;
    let half_height = f32::tan(theta / 2.0);
    let half_width = aspect * half_height;
    let w = (look_from - look_at).unit_vector();
    let u = (up.cross(&w)).unit_vector();
    let v = w.cross(&u);

    let lower_left =
        look_from - (u * half_width * focus_dist) - (v * half_height * focus_dist) - w * focus_dist;
    let horizontal = u * (2.0 * focus_dist * half_width);
    let vertical = v * (2.0 * focus_dist * half_height);

    Camera {
        lower_left: lower_left,
        horizontal: horizontal,
        vertical: vertical,
        origin: look_from,
        lens_radius: lens_radius,
        u: u,
        v: v,
        time0: t0,
        time1: t1,
    }
}

fn get_old_spheres() -> SphereList {
    return SphereList {
        spheres: vec![
            SphereThing::S(Sphere {
                center: Point {
                    x: 3.0,
                    y: 0.0,
                    z: 0.5,
                },
                radius: 0.5,
                material: Material::Lambertian(Lambertian {
                    albedo: Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.5,
                    },
                }),
            }),
            SphereThing::S(Sphere {
                center: Point {
                    x: 0.0,
                    y: -100.5,
                    z: 0.0,
                },
                radius: 100.0,
                material: Material::Lambertian(Lambertian {
                    albedo: Color {
                        r: 0.8,
                        g: 0.8,
                        b: 0.0,
                    },
                }),
            }),
            SphereThing::SM(SphereMoving {
                center0: Point {
                    x: 2.0,
                    y: 0.2,
                    z: -0.5,
                },
                center1: Point {
                    x: 2.0,
                    y: 0.0,
                    z: -0.5,
                },
                radius: 0.5,
                material: Material::Metal(Metal {
                    albedo: Color {
                        r: 0.8,
                        g: 0.6,
                        b: 0.2,
                    },
                }),
                time0: 0.0,
                time1: 1.0,
            }),
            SphereThing::S(Sphere {
                center: Point {
                    x: 1.0,
                    y: 0.0,
                    z: 1.0,
                },
                radius: 0.5,
                material: Material::Dielectric(Dielectric {
                    reflective_index: 1.5,
                }),
            }),
            SphereThing::S(Sphere {
                center: Point {
                    x: 1.0,
                    y: 0.0,
                    z: 1.0,
                },
                radius: -0.45,
                material: Material::Dielectric(Dielectric {
                    reflective_index: 1.5,
                }),
            }),
        ],
    };
}

fn get_spheres_many() -> SphereList {
    let mut v: Vec<SphereThing> = vec![];

    v.push(SphereThing::S(Sphere {
        center: Point {
            x: -0.0,
            y: -1000.0,
            z: 0.0,
        },
        radius: 1000.0,
        material: Material::Lambertian(Lambertian {
            albedo: Color {
                r: 0.5,
                g: 0.5,
                b: 0.5,
            },
        }),
    }));
    v.push(SphereThing::S(Sphere {
        center: Point {
            x: 4.0,
            y: 0.7,
            z: 0.0,
        },
        radius: 0.7,
        material: Material::Dielectric(Dielectric {
            reflective_index: 1.5,
        }),
    }));
    v.push(SphereThing::S(Sphere {
        center: Point {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        radius: 1.0,
        material: Material::Dielectric(Dielectric {
            reflective_index: 1.5,
        }),
    }));
    v.push(SphereThing::S(Sphere {
        center: Point {
            x: -4.0,
            y: 1.0,
            z: 0.0,
        },
        radius: 1.0,
        material: Material::Metal(Metal {
            albedo: Color {
                r: 0.7,
                g: 0.6,
                b: 0.5,
            },
        }),
    }));

    for a in -7..7 {
        for b in -7..7 {
            let center = Point {
                x: a as f32 + 0.9 * random::<f32>(),
                y: 0.2,
                z: b as f32 + 0.9 * random::<f32>(),
            };
            let sphere = match random::<f32>() {
                x if x < 0.7 => Sphere {
                    center: center,
                    radius: 0.2,
                    material: Material::Lambertian(Lambertian {
                        albedo: Color {
                            r: random(),
                            g: random(),
                            b: random(),
                        },
                    }),
                },
                x if x < 0.85 => Sphere {
                    center: center,
                    radius: 0.2,
                    material: Material::Metal(Metal {
                        albedo: Color {
                            r: random(),
                            g: random(),
                            b: random(),
                        },
                    }),
                },
                _ => Sphere {
                    center: center,
                    radius: 0.2,
                    material: Material::Dielectric(Dielectric {
                        reflective_index: 1.5,
                    }),
                },
            };
            v.push(SphereThing::S(sphere));
        }
    }

    return SphereList { spheres: v };
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
        let u = (i as f32 + random::<f32>()) / NX as f32;
        let v = (j as f32 + random::<f32>()) / NY as f32;

        let ray = cam.get_ray(u, v);
        col += color(&ray, bvh_box, 0);
    }
    col / NS as f32
}

const NX: i32 = 400;
const NY: i32 = 200;
const NS: i32 = 100;

fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    let mut buffer = File::create("out.ppm")?;
    buffer.write(format!("P3\n{} {}\n255\n", NX, NY).as_bytes())?;

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
        10.0,
        NX as f32 / NY as f32,
        0.01,
        0.0,
        1.0,
    );
    let spherelist = get_spheres_many();
    //let spherelist = get_old_spheres();

    let mut to_calc = vec![];

    let bound_box = get_bvh_box2(spherelist.spheres.clone());
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
        buffer.write(row.as_color_str().as_bytes())?;
    }
    buffer.flush()?;

    Ok(())
}
