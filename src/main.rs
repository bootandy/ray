use rand::random;
use rayon::prelude::*;
use std::cell::RefCell;
use std::f32;
use std::f32::consts::PI;
use std::fs::File;
use std::io::prelude::*;

use data::bounding::*;
use data::material::*;
use data::ray::Ray;
use data::sphere::*;
use data::textures::*;
use data::vec3::*;
use NO_COLOR;

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
            let scattered = hit.material.scatter(r, hit.normal, hit.p);
            let emitted = hit.material.emitted(&hit.p, hit.u, hit.v);
            /*if emitted.is_pure_light() {
                return emitted
            }*/

            if let Some(scatter_ray) = scattered {
                let albedo = hit.material.get_albedo(&hit.p, hit.u, hit.v);
                let c = color(&scatter_ray, bound_box, depth + 1);
                return emitted + c.mul(&albedo);
            } else {
                return emitted;
            }
        }
        None => {
            return NO_COLOR;
        }
    }
}

fn random_in_unit_disk() -> Point {
    loop {
        let p = Point {
            x: rnd() * 2.0 - 1.0,
            y: rnd() * 2.0 - 1.0,
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
        let time = self.time0 + (rnd() * (self.time1 - self.time0));
        let direction = self.lower_left + self.horizontal * s + self.vertical * t - end;
        Ray {
            origin: self.origin + offset,
            direction,
            time,
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
    time0: f32,
    time1: f32,
) -> Camera {
    let focus_dist = (look_from - look_at).length();
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
        lower_left,
        horizontal,
        vertical,
        origin: look_from,
        lens_radius,
        u,
        v,
        time0,
        time1,
    }
}

// cornell box
pub fn get_light_room() -> SphereList {
    let green = Material::Lambertian(Lambertian {
        texture: Texture::T(ConstantTexture {
            color: Color {
                r: 0.12,
                g: 0.45,
                b: 0.15,
            },
        }),
    });
    let red = Material::Lambertian(Lambertian {
        texture: Texture::T(ConstantTexture {
            color: Color {
                r: 0.65,
                g: 0.05,
                b: 0.05,
            },
        }),
    });
    let white = Material::Lambertian(Lambertian {
        texture: Texture::T(ConstantTexture {
            color: Color {
                r: 0.73,
                g: 0.73,
                b: 0.73,
            },
        }),
    });
    let light = Material::DiffuseLight(DiffuseLight { brightness: 15.0 });

    let mut spheres = vec![
        SphereThing::R(Rectangle::new_yz(
            0.0, 555.0, 0.0, 555.0, 555.0, green, true,
        )),
        SphereThing::R(Rectangle::new_yz(0.0, 555.0, 0.0, 555.0, 0.0, red, false)),
        SphereThing::R(Rectangle::new_xz(
            213.0, 343.0, 227.0, 332.0, 554.0, light, false,
        )),
        SphereThing::R(Rectangle::new_xz(
            0.0,
            555.0,
            0.0,
            555.0,
            0.0,
            white.clone(),
            false,
        )),
        SphereThing::R(Rectangle::new_xz(
            0.0,
            555.0,
            0.0,
            555.0,
            555.0,
            white.clone(),
            true,
        )),
        SphereThing::R(Rectangle::new_xy(
            0.0,
            555.0,
            0.0,
            555.0,
            555.0,
            white.clone(),
            true,
        )),
    ];

    spheres.extend(new_box(
        Point {
            x: 130.0,
            y: 0.0,
            z: 65.0,
        },
        Point {
            x: 295.0,
            y: 165.0,
            z: 230.0,
        },
        white.clone(),
    ));
    spheres.extend(new_box(
        Point {
            x: 265.0,
            y: 0.0,
            z: 295.0,
        },
        Point {
            x: 430.0,
            y: 330.0,
            z: 460.0,
        },
        white.clone(),
    ));

    SphereList { spheres: spheres }
}

pub fn get_lights() -> SphereList {
    SphereList {
        spheres: vec![
            SphereThing::S(Sphere {
                center: Point {
                    x: 0.0,
                    y: -100.0,
                    z: 0.0,
                },
                radius: 100.0,
                material: Material::Lambertian(Lambertian {
                    texture: Texture::NT(build_noise()),
                }),
            }),
            SphereThing::S(Sphere {
                center: Point {
                    x: 0.0,
                    y: 2.0,
                    z: 0.0,
                },
                radius: 2.0,
                material: Material::Lambertian(Lambertian {
                    texture: Texture::IT(build_image_texture()),
                }),
            }),
            SphereThing::S(Sphere {
                center: Point {
                    x: 0.0,
                    y: 7.0,
                    z: 0.0,
                },
                radius: 2.0,
                material: Material::DiffuseLight(DiffuseLight { brightness: 8.0 }),
            }),
            SphereThing::R(Rectangle::new_xy(
                3.0,
                5.0,
                0.2,
                3.0,
                -2.0,
                Material::DiffuseLight(DiffuseLight { brightness: 8.0 }),
                false,
            )),
        ],
    }
}

#[allow(dead_code)]
fn get_old_spheres() -> SphereList {
    SphereList {
        spheres: vec![
            SphereThing::S(Sphere {
                center: Point {
                    x: 3.0,
                    y: 0.8,
                    z: 0.5,
                },
                radius: 1.5,
                material: Material::Lambertian(Lambertian {
                    texture: Texture::IT(build_image_texture()),
                    /*texture: Texture::T(ConstantTexture {
                        color: Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.5,
                        },
                    }),*/
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
                    texture: Texture::NT(build_noise()),
                    //texture: Texture::IT(build_image_texture()),
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
    }
}

#[allow(dead_code)]
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
            texture: Texture::NT(build_noise()),
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
                x: a as f32 + 0.9 * rnd(),
                y: 0.2,
                z: b as f32 + 0.9 * rnd(),
            };
            let material = match rnd() {
                x if x < 0.7 => Material::Lambertian(Lambertian {
                    texture: Texture::T(ConstantTexture {
                        color: Color {
                            r: rnd(),
                            g: rnd(),
                            b: rnd(),
                        },
                    }),
                }),
                x if x < 0.85 => Material::Metal(Metal {
                    albedo: Color {
                        r: rnd(),
                        g: rnd(),
                        b: rnd(),
                    },
                }),
                _ => Material::Dielectric(Dielectric {
                    reflective_index: 1.5,
                }),
            };

            let sphere = match rnd() {
                // Lets not have moving spheres
                x if x < 1.8 => SphereThing::S(Sphere {
                    center,
                    radius: 0.2,
                    material,
                }),
                _ => SphereThing::SM(SphereMoving {
                    center0: center,
                    center1: center + Point {
                        x: 0.0,
                        y: rnd() / 2.0,
                        z: 0.0,
                    },
                    radius: 0.2,
                    material,
                    time0: 0.0,
                    time1: 1.0,
                }),
            };
            v.push(sphere);
        }
    }

    SphereList { spheres: v }
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
const NS: i32 = 500;

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

    //let spherelist = get_spheres_many();
    //let spherelist = get_old_spheres();
    let spherelist = get_light_room();

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
