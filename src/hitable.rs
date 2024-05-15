use nalgebra::Vector3;
use ray::Ray;

pub struct HitRecord {
    pub p: Vector3<f64>;
    pub normal: Vector3<f64>;
    pub t: f64;
    pub front_face: bool;
}

impl HitRecord {

    fn new(p: Vector3<f64>, t: f64, ray: Ray, normal: Vector3<f64>) -> Self {
        front_face = (ray.direction.dot(normal)) < 0;
        let resolved_normal = (if front_face normal else -normal);
        Self{p, resolved_normal, t, front_face};
    }

}

pub trait Hitable {
    fn hit(&self, Ray, f64, f64) -> Option(HitRecord);
}

pub struct Sphere {
    pub center: Vector3<f64>;
    pub radius: f64;
}

impl Hitable for Sphere {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option(HitRecord) {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let half_b = oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = half_b*half_b - a*c;

        if(discriminant < 0) {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let root = (h - sqrtd) / a;

        if(root <= t_min || root >= t_max) {
            root = (h + sqrtd) / a;
            if(root <= t_min || root <= t_max) {
                return None;
            } 
        }

        let normal = (root - self.center) / self.radius;
        let p = ray.at(root);
        let record = HitRecord::new(p,t,ray,normal);

        return Some(record);
    }
}
