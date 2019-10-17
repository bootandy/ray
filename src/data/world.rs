use data::material::Lambertian;
use data::material::Metal;
use data::material::Dielectric;
use data::old_vec3::Color;
use data::old_vec3::Point;
use data::sphere::Sphere;
use data::sphere::{Hittable, HittableObjects};

pub fn build_world<'b>() -> (HittableObjects<'b>) {
    let s = Sphere {
        center: Point {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
        material: &(Lambertian {
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
        material: &(Metal {
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
        material: &(Dielectric {
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
        material: &(Dielectric {
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
        material: &(Lambertian {
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
