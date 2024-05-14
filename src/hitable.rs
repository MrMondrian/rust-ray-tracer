use nalgebra::Vector3;
use ray::Ray;

pub struct HitRecord {
    pub p: Vector3;
    pub normal: Vector3;
    pub t: f64;
}

pub trait Hitable {
    fn hit(&self, Ray, f64, f64) -> HitRecord;
}
