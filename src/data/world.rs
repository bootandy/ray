//use data::material::{Lambertian, Material};
//use data::material::Metal;
//use data::material::Dielectric;
//use data::old_vec3::Color;
//use data::old_vec3::Point;
//use data::sphere::Sphere;
//use data::sphere::{Hittable, HittableObjects};
//use rand::random;
//use std::sync::Arc;
//
//pub fn build_world() -> (HittableObjects) {
//    let s = Sphere {
//        center: Point {
//            x: 0.0,
//            y: 0.0,
//            z: -1.0,
//        },
//        radius: 0.5,
//        material: Arc::new(Lambertian {
//            albedo: Color {
//                r: 0.8,
//                g: 0.3,
//                b: 0.3,
//            },
//        }),
//    };
//    let s2 = Sphere {
//        center: Point {
//            x: 1.0,
//            y: 0.0,
//            z: -1.0,
//        },
//        radius: 0.5,
//        material: Arc::new(Metal {
//            albedo: Color {
//                r: 0.8,
//                g: 0.6,
//                b: 0.2,
//            },
//            fuzz: 0.1
//        }),
//    };
//    let s3 = Sphere {
//        center: Point {
//            x: -1.0,
//            y: 0.0,
//            z: -1.0,
//        },
//        radius: 0.6,
//        material: Arc::new(Dielectric {
//            reflective_index: 1.5,
//        }),
//    };
//    let s3i = Sphere {
//        center: Point {
//            x: -1.0,
//            y: 0.0,
//            z: -1.0,
//        },
//        radius: -0.5,
//        material: Arc::new(Dielectric {
//            reflective_index: 1.5,
//        }),
//    };
//    let world = Sphere {
//        center: Point {
//            x: 0.0,
//            y: -100.5,
//            z: -1.0,
//        },
//        radius: 100.0,
//        material: Arc::new(Lambertian {
//            albedo: Color {
//                r: 0.8,
//                g: 0.8,
//                b: 0.0,
//            },
//        }),
//    };
//
//    HittableObjects {
//        objects: vec![
//            Hittable::Sphere(s),
//            Hittable::Sphere(s2),
//            Hittable::Sphere(s3),
//            Hittable::Sphere(s3i),
//            Hittable::Sphere(world),
//        ],
//    }
//}
//
//fn rnd_material<'a>() -> Arc<dyn Material> {
//    match random::<f32>() {
//        d if d < 0.333 => {
//            return Arc::new(Lambertian {
//                albedo: Color { r: random::<f32>(), b: random::<f32>(), g: random::<f32>() }
//            });
//        },
//        d if d < 0.666 => {
//            return Arc::new(Dielectric{
//                reflective_index: random::<f32>() + 1.0,
//            });
//        },
//        _ => {
//            return Arc::new(Metal {
//                albedo: Color { r: random::<f32>(), b: random::<f32>(), g: random::<f32>() },
//                fuzz: 0.0,
//            });
//        }
//    }
//}
//
//pub fn build_many() -> HittableObjects {
//      let big_base = Sphere {
//        center: Point {
//            x: 0.0,
//            y: -1000.0,
//            z: -1.0,
//        },
//        radius: 1000.0,
//        material: Arc::new(Lambertian {
//            albedo: Color {
//                r: 0.8,
//                g: 0.8,
//                b: 0.0,
//            },
//        }),
//    };
//
//    let mut objects = vec![Hittable::Sphere(big_base)];
//    for i in -10..10 {
//        for j in -10..10 {
//            let how_high = random::<f32>()/2.0 + 0.1;
//            let center = Point{
//                x: 40.0 *random::<f32>() - 20.0,
//                y: how_high,
//                z: 30.0 *random::<f32>() - 20.0,
//            };
//            let material = rnd_material();
//            let s = Sphere {center, radius: how_high, material};
//            objects.push(Hittable::Sphere(s));
//        }
//    }
//    let material = Arc::new(Dielectric{ reflective_index: 1.5 });
//    let s = Sphere {center: Point{x:4.0, y: 4.0, z:3.0}, radius: 4.0, material};
//    objects.push(Hittable::Sphere(s));
//
//    let material = Arc::new(Metal{ albedo: Color{r:0.7, g:0.6, b:0.5}, fuzz: 0.0});
//    let s = Sphere {center: Point{x:-4.0, y: 4.0, z:3.0}, radius: 4.0, material};
//    objects.push(Hittable::Sphere(s));
//
//    HittableObjects { objects }
//}