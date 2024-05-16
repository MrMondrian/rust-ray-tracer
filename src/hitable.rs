use nalgebra::Vector3;
use crate::ray::Ray;

pub struct HitRecord {
    pub p: Vector3<f64>,
    pub normal: Vector3<f64>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {

    pub fn new(p: Vector3<f64>, t: f64, ray: &Ray, out_normal: Vector3<f64>) -> Self {
        let front_face = (ray.direction.dot(&out_normal)) < 0.0;
        let normal = if front_face {out_normal} else {-out_normal};
        Self{p, normal, t, front_face}
    }

}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f64, t_min: f64) -> Option<HitRecord>;
}

pub struct Sphere {
    pub center: Vector3<f64>,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center:Vector3<f64>, radius: f64) -> Self {
        Self{center,radius}
    }
}

impl Hitable for Sphere {
    

    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = self.center - ray.origin;
        let a = ray.direction.dot(&ray.direction);
        let half_b = oc.dot(&ray.direction);
        let c = oc.dot(&oc) - (self.radius * self.radius);
        let discriminant = (half_b*half_b) - (a*c);

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (half_b - sqrtd) / a;

        if root <= t_min || root >= t_max {
            root = (half_b + sqrtd) / a;
            if root <= t_min || root >= t_max {
                return None;
            } 
        }

        let p = ray.at(root);
        let normal = (p - self.center) / self.radius;
        let record = HitRecord::new(p,root,ray,normal);

        return Some(record);
    }
}
