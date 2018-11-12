
use Point;
use Ray;
use Material;

pub struct Hit<'a> {
    pub t: f32,
    pub p: Point,
    pub normal: Point,
    pub material: &'a Material,
}

pub struct Sphere {
    pub center: Point,
    pub radius: f32,
    pub material: Material,
}

impl Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let oc = r.origin.clone() - self.center.clone();
        let a = r.direction.dot(&r.direction);
        let b = oc.dot(&r.direction);
        let c = oc.dot(&oc) - self.radius.clone() * self.radius;
        let d = b * b - (a * c);
        if d > 0.0 {
            let temp = (-b - (b*b-a*c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = r.point_at_parameter(temp);
                let normal = (p.clone() - self.center.clone()) / self.radius;
                return Some(Hit{t:temp, p:p, normal:normal, material:&self.material})
            }
            let temp = (-b + (b*b-a*c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = r.point_at_parameter(temp);
                let normal = (p.clone() - self.center.clone()) / self.radius;
                return Some(Hit{t:temp, p:p, normal:normal, material:&self.material})
            }
        }
        return None
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
