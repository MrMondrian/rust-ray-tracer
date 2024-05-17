use crate::ray::Ray;
use crate::hitable::HitRecord;
use nalgebra::Vector3;
use rand::prelude::*;


pub trait Material {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<(Ray, Vector3<f64>)>;
}

pub struct Lambertian {
    albedo: Vector3<f64>,
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<(Ray,Vector3<f64>)> {
        let mut scatter_direction = record.normal + self.random_unit_vector();
        if scatter_direction.iter().all(|&x| x > 1e-8) {
            scatter_direction = record.normal;
        }
        let scattered = Ray::new(record.p, scatter_direction);
        return Some((scattered, self.albedo));
    }
}

impl Lambertian{


   pub  fn new(albedo: Vector3<f64>) -> Self {
        Self{albedo}
    }

    fn random_vector(&self) -> Vector3<f64> {
        let mut rng = rand::thread_rng();
        return Vector3::new(rng.gen_range(-1.0..1.0),rng.gen_range(-1.0..1.0),rng.gen_range(-1.0..1.0))
    }

    fn random_unit_vector(&self) -> Vector3<f64> {
        let vec = self.random_vector();
        return vec / vec.norm();
    }
}

pub struct Metal {
    albedo: Vector3<f64>
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<(Ray,Vector3<f64>)> {
       let reflected = Metal::reflect(&ray_in.direction, &record.normal);
       let scattered = Ray::new(record.p, reflected);
       return Some((scattered, self.albedo));
    }
}

impl Metal {

    pub fn new(albedo: Vector3<f64>) -> Self {
        Self{albedo}
    }

    fn reflect(v: &Vector3<f64>, n: &Vector3<f64>) -> Vector3<f64> {
        v - 2.0*v.dot(&n)*n
    }
}
