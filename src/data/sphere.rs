
use Point;
use Ray;
use Material;

pub struct Hit<'a> {
    pub t: f32,
    pub p: Point,
    pub normal: Point,
    pub material: &'a Material,
}

// TODO: Add stationary Sphere class

pub struct Sphere {
    pub center0: Point,
    pub center1: Point,
    pub radius: f32,
    pub material: Material,
    pub time0: f32,
    pub time1: f32,
}

impl Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let oc = r.origin.clone() - self.get_center(&r.time);
        let a = r.direction.dot(&r.direction);
        let b = oc.dot(&r.direction);
        let c = oc.dot(&oc) - self.radius.clone() * self.radius;
        let d = b * b - (a * c);
        if d > 0.0 {
            let temp = (-b - (b*b-a*c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = r.point_at_parameter(temp);
                let normal = (p.clone() - self.get_center(&r.time)) / self.radius;
                return Some(Hit{t:temp, p:p, normal:normal, material:&self.material})
            }
            let temp = (-b + (b*b-a*c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = r.point_at_parameter(temp);
                let normal = (p.clone() - self.get_center(&r.time)) / self.radius;
                return Some(Hit{t:temp, p:p, normal:normal, material:&self.material})
            }
        }
        return None
    }

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
    pub spheres: Vec<Sphere>,
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
