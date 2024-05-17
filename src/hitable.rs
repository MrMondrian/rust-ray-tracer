use nalgebra::Vector3;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::material::Material;

pub struct HitRecord<'a> {
    pub p: Vector3<f64>,
    pub normal: Vector3<f64>,
    pub t: f64,
    pub front_face: bool,
    pub mat: &'a dyn Material,
}

impl<'a> HitRecord<'a> {

    pub fn new(p: Vector3<f64>, t: f64, ray: &Ray, out_normal: Vector3<f64>, mat: &'a dyn Material) -> Self {
        let front_face = (ray.direction.dot(&out_normal)) < 0.0;
        let normal = if front_face {out_normal} else {-out_normal};
        Self{p, normal, t, front_face, mat}
    }

}

pub trait Hitable {
    fn hit(&self, ray: &Ray, bounds: &Interval) -> Option<HitRecord>;
}

pub struct Sphere<M: Material> {
    pub center: Vector3<f64>,
    pub radius: f64,
    pub mat: M,
}

impl<M:Material> Sphere<M> {
    pub fn new(center:Vector3<f64>, radius: f64, mat: M) -> Self {
        Self{center,radius, mat}
    }
}

impl<M:Material> Hitable for Sphere<M> {
    

    fn hit(&self, ray: &Ray, bounds: &Interval) -> Option<HitRecord> {
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

        if !bounds.contains(root) {
            root = (half_b + sqrtd) / a;
            if !bounds.contains(root) {
                return None;
            } 
        }

        let p = ray.at(root);
        let normal = (p - self.center) / self.radius;
        let record = HitRecord::new(p,root,ray,normal,&self.mat);

        return Some(record);
    }
}
