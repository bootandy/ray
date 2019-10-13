
use data::ray::Ray;
use data::sphere::HittableObjects;
use data::vec3::Vec3;
use data::sphere::Sphere;
use data::material::Lambertian;
use data::material::Metal;


pub fn build_world() -> HittableObjects {
    let s = Sphere {
        center: Vec3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
        material: Box::new(Lambertian {
            albedo: Vec3 {
                x: 0.8,
                y: 0.3,
                z: 0.3,
            },
        }),
    };
    let s2 = Sphere {
        center: Vec3 {
            x: 1.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
        material: Box::new(Metal {
            albedo: Vec3 {
                x: 0.8,
                y: 0.6,
                z: 0.2,
            },
        }),
    };
    let s3 = Sphere {
        center: Vec3 {
            x: -1.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
        material: Box::new(Metal {
            albedo: Vec3 {
                x: 0.8,
                y: 0.8,
                z: 0.8,
            },
        }),
    };
    let world = Sphere {
        center: Vec3 {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        },
        radius: 100.0,
        material: Box::new(Lambertian {
            albedo: Vec3 {
                x: 0.8,
                y: 0.8,
                z: 0.0,
            },
        }),
    };

    HittableObjects {
        objects: vec![Box::new(s), Box::new(s2), Box::new(s3), Box::new(world)],
    }
}

