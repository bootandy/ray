
use super::vec::Point;
use super::ray::Ray;
use super::materials::Material;

pub struct Sphere {
    pub center: Point,
    pub radius: f32,
    pub material: Box<Material>,
}

// should be an iface:
//    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &HItRecord) -> bool{

impl Sphere {
    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitableRecord> {
        let oc = r.origin.sub(&self.center);
        let a = r.direction.dot(&r.direction);
        let b = oc.dot(&r.direction);
        let c = oc.dot(&oc) - (self.radius * self.radius);
        let discriminant = (b * b) - (a * c);

        if discriminant > 0.0 {
            let temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = r.point_at_parameter(temp);
                let normal = (p.sub(&self.center)).flat_div(self.radius);
                return Some(HitableRecord{t:temp, p:p, normal:normal, material:&self.material});
            }
            let temp = (-b + discriminant.sqrt()) / a;
            // dup code
            if temp < t_max && temp > t_min {
                let p = r.point_at_parameter(temp);
                let normal = (p.sub(&self.center)).flat_div(self.radius);
                return Some(HitableRecord{t:temp, p:p, normal:normal, material:&self.material});
            }
        }
        return None
    }
}

// add hitable iface
pub struct SphereList {
    pub spheres : Vec<Sphere>
}

impl SphereList {
    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitableRecord> {
        let mut closest = t_max;
        let mut temp_rec = None;

        for s in &self.spheres {
            match s.hit(r, t_min, closest) {
                Some(rec) =>  {
                    closest = rec.t;
                    temp_rec = Some(rec);
                },
                None => {},
            }
        }
        return temp_rec;
    }
}

pub struct HitableRecord<'a> {
    pub t: f32,
    pub p: Point,
    pub normal: Point,
    pub material: &'a Box<Material>,
}
