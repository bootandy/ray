use Color;
use Point;

use Sphere;
use SphereThing;
use SphereMoving;
use Texture;
use ConstantTexture;
use Metal;
use Lambertian;
use Dielectric;
use Material;
use SphereList;
use build_image_texture;
use build_noise;

use rnd;

#[allow(dead_code)]
pub fn get_old_spheres() -> SphereList {
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
                    y: 0.8,
                    z: 2.0,
                },
                radius: 1.5,
                material: Material::Dielectric(Dielectric {
                    reflective_index: 1.5,
                }),
            }),
            SphereThing::S(Sphere {
                center: Point {
                    x: 1.0,
                    y: 0.8,
                    z: 2.0,
                },
                radius: -1.45,
                material: Material::Dielectric(Dielectric {
                    reflective_index: 1.5,
                }),
            }),
        ],
    }
}

#[allow(dead_code)]
pub fn get_spheres_many() -> SphereList {
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