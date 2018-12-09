
use std::f32::consts::PI;
use Ray;
use Point;
use rnd;


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

pub struct Camera {
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
    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
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

pub fn get_camera(
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