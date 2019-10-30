use data::material::{Lambertian, Material};
use data::material::Metal;
use data::material::Dielectric;
use data::old_vec3::Color;
use data::old_vec3::Point;
use data::sphere::{Sphere, MovingSphere};
use data::sphere::{Hittable, HittableObjects};
use std::boxed::Box;
use rand::random;

pub fn build_world() -> (HittableObjects) {
    let s = Sphere {
        center: Point {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
        material: Box::new(Lambertian {
            albedo: Color {
                r: 0.8,
                g: 0.3,
                b: 0.3,
            },
        }),
    };
    let s2 = Sphere {
        center: Point {
            x: 1.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
        material: Box::new(Metal {
            albedo: Color {
                r: 0.8,
                g: 0.6,
                b: 0.2,
            },
            fuzz: 0.1
        }),
    };
    let s3 = Sphere {
        center: Point {
            x: -1.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.6,
        material: Box::new(Dielectric {
            reflective_index: 1.5,
        }),
    };
    let s3i = Sphere {
        center: Point {
            x: -1.0,
            y: 0.0,
            z: -1.0,
        },
        radius: -0.5,
        material: Box::new(Dielectric {
            reflective_index: 1.5,
        }),
    };
    let world = Sphere {
        center: Point {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        },
        radius: 100.0,
        material: Box::new(Lambertian {
            albedo: Color {
                r: 0.8,
                g: 0.8,
                b: 0.0,
            },
        }),
    };

    HittableObjects {
        objects: vec![
            Hittable::Sphere(s),
            Hittable::Sphere(s2),
            Hittable::Sphere(s3),
            Hittable::Sphere(s3i),
            Hittable::Sphere(world),
        ],
    }
}

fn rnd_material<'a>() -> Box<dyn Material> {
    match random::<f32>() {
        d if d < 0.5 => {
            return Box::new(Lambertian {
                albedo: Color { r: random::<f32>(), b: random::<f32>(), g: random::<f32>() }
            });
        },
        d if d < 0.6 => {
            return Box::new(Dielectric{
                reflective_index: random::<f32>() + 1.0,
            });
        },
        _ => {
            return Box::new(Metal {
                albedo: Color { r: random::<f32>(), b: random::<f32>(), g: random::<f32>() },
                fuzz: 0.0,
            });
        }
    }
}

pub fn build_many() -> HittableObjects {
      let big_base = Sphere {
        center: Point {
            x: 0.0,
            y: -10000.0,
            z: -1.0,
        },
        radius: 10000.0,
        material: Box::new(Lambertian {
            albedo: Color {
                r: 0.5,
                g: 0.5,
                b: 0.5,
            },
        }),
    };

    let mut objects = vec![Hittable::Sphere(big_base)];
    for _i in -10..10 {
        for _j in -10..10 {
            let how_high = random::<f32>()/3.0 + 0.3;
            let center = Point{
                x: 40.0 *random::<f32>() - 20.0,
                y: how_high,
                z: 40.0 *random::<f32>() - 10.0,
            };
            let material = rnd_material();

                if random::<f32>() < 0.5 {
                    objects.push(Hittable::Sphere(Sphere { center, radius: how_high, material }));
                } else {
                    let center1 = center + Point{x:0.0, y: random::<f32>(), z:0.0};
                    objects.push(Hittable::MovingSphere(MovingSphere{center0:center, center1, radius: how_high, material, time0:0.0, time1:1.0}));
                }
        }
    }
    let material = Box::new(Lambertian{ albedo: Color{r:0.4, g:0.2, b:0.1} });
    let s = Sphere {center: Point{x:0.0, y: 2.0, z:3.0}, radius: 2.0, material};
    objects.push(Hittable::Sphere(s));

    let material = Box::new(Dielectric{ reflective_index: 1.5 });
    let s = Sphere {center: Point{x:-3.0, y: 2.0, z:0.0}, radius: 2.0, material};
    objects.push(Hittable::Sphere(s));

    let material = Box::new(Metal{ albedo: Color{r:0.7, g:0.6, b:0.5}, fuzz: 0.0});
    let s = Sphere {center: Point{x:-6.0, y: 2.0, z:-3.0}, radius: 2.0, material};
    objects.push(Hittable::Sphere(s));

//    let material = Box::new(Metal{ albedo: Color{r:0.9, g:0.9, b:0.9}, fuzz:0.0});
//    let forward_center_glass = Sphere {center: Point{x:3.0, y: 1.4, z:-5.0}, radius: 1.4, material};
//    objects.push(Hittable::Sphere(forward_center_glass));

    HittableObjects { objects }
}
