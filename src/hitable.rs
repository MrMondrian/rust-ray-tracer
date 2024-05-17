use nalgebra::Vector3;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::material::Material;

pub struct HitRecord<'a> {
    pub p: Vector3<f64>,
    pub normal: Vector3<f64>,
    pub t: f64,
    pub front_face: bool,
    pub mat: &'a Material,
}

impl<'a> HitRecord<'a> {

    pub fn new(p: Vector3<f64>, t: f64, ray: &Ray, out_normal: Vector3<f64>, mat: &'a Material) -> Self {
        let front_face = (ray.direction.dot(&out_normal)) < 0.0;
        let normal = if front_face {out_normal} else {-out_normal};
        Self{p, normal, t, front_face, mat}
    }

}

pub enum Hitable {
    Sphere{center: Vector3<f64>, radius: f64, mat: Material}
}


impl Hitable {
    pub fn hit(&self, ray: &Ray, bounds: &Interval) -> Option<HitRecord> {
        match self {
            Hitable::Sphere{center, radius, mat} => {
                let oc = center - ray.origin;
                let a = ray.direction.dot(&ray.direction);
                let half_b = oc.dot(&ray.direction);
                let c = oc.dot(&oc) - (radius * radius);
                let discriminant = (half_b*half_b) - (a*c);

                if discriminant < 0.0 {
                    return None;
                }

                let sqrtd = discriminant.sqrt();
                let mut root = (half_b - sqrtd) / a;

                if !bounds.contains(root) {
                    root = (half_b + sqrtd) / a;
                    if !bounds.contains(root) {
                        return None;
                    } 
                }

                let p = ray.at(root);
                let normal = (p - center) / *radius;
                let record = HitRecord::new(p,root,ray,normal,&mat);

                return Some(record);
            }
        }
    }
}
