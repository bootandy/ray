
use Point;
use Ray;
use Material;

fn hit<'a>(r: &Ray, t_min: f32, t_max: f32, radius: f32, material :&'a Material, center: &Point) -> Option<Hit<'a>> {
    let oc = r.origin.clone() - *center;
    let a = r.direction.dot(&r.direction);
    let b = oc.dot(&r.direction);
    let c = oc.dot(&oc) - radius * radius;
    let d = b * b - (a * c);
    if d > 0.0 {
        let temp = (-b - (b*b-a*c).sqrt()) / a;
        if temp < t_max && temp > t_min {
            let p = r.point_at_parameter(temp);
            let normal = (p.clone() - *center) / radius;
            return Some(Hit{t:temp, p:p, normal:normal, material:material})
        }
        let temp = (-b + (b*b-a*c).sqrt()) / a;
        if temp < t_max && temp > t_min {
            let p = r.point_at_parameter(temp);
            let normal = (p.clone() - *center) / radius;
            return Some(Hit{t:temp, p:p, normal:normal, material:material})
        }
    }
    return None
}

pub struct Hit<'a> {
    pub t: f32,
    pub p: Point,
    pub normal: Point,
    pub material: &'a Material,
}

trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit>;
}

pub enum SphereThing {
    S(Sphere),
    SM(SphereMoving),
}

impl Hittable for SphereThing {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit>{
        match self {
            SphereThing::S(s) => s.hit(r, t_min, t_max),
            SphereThing::SM(s) => s.hit(r, t_min, t_max),
        }
    }
}


pub struct Sphere {
    pub center: Point,
    pub radius: f32,
    pub material: Material,
}
impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        hit(r, t_min, t_max, self.radius, &self.material, &self.center)
    }
}


pub struct SphereMoving {
    pub center0: Point,
    pub center1: Point,
    pub radius: f32,
    pub material: Material,
    pub time0: f32,
    pub time1: f32,
}

impl Hittable for SphereMoving {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        hit(r, t_min, t_max, self.radius, &self.material, &self.get_center(&r.time))
    }
}
impl SphereMoving{
    fn get_center(&self, time : &f32) -> Point {
        if self.time0 == self.time1 {
            return self.center0.clone()
        } else {
            let t_diff = (time - self.time0) / (self.time1 - self.time0);
            return self.center0 + ((self.center1 - self.center0) * t_diff)
        }
    }
}

pub struct SphereList {
    pub spheres: Vec<SphereThing>,
}
impl SphereList {
    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let mut what_we_hit = None;
        let mut closest_so_far = t_max;
        for s in self.spheres.iter() {
            match s.hit(r, t_min, closest_so_far) {
                Some(h) => {
                    closest_so_far = h.t;
                    what_we_hit = Some(h);
                },
                None => {},
            }
        }
        return what_we_hit
    }
}
