use crate::ray::Ray;
use crate::hitable::HitRecord;
use nalgebra::Vector3;
use rand::prelude::*;


pub enum Material {
    Lambertian{albedo: Vector3<f64>},
    Metal{albedo: Vector3<f64>}
}


impl Material {
    pub fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<(Ray,Vector3<f64>)> {
        match self  {
            Material::Lambertian{albedo} =>   {
                let mut scatter_direction = record.normal + random_unit_vector();
                if scatter_direction.iter().all(|&x| x < 1e-8) {
                    scatter_direction = record.normal;
                }
                let scattered = Ray::new(record.p, scatter_direction);
                return Some((scattered, *albedo));
            }
            Material::Metal{albedo} => {
                let reflected = reflect(&ray_in.direction, &record.normal);
                let scattered = Ray::new(record.p, reflected);
                return Some((scattered, *albedo));
            }
        }
    }
}


fn random_vector() -> Vector3<f64> {
    let mut rng = rand::thread_rng();
    return Vector3::new(2.0*rng.gen::<f64>()-1.0, 2.0*rng.gen::<f64>()-1.0,2.0*rng.gen::<f64>()-1.0)
}

fn random_unit_vector() -> Vector3<f64> {
    let vec = random_vector();
    return vec / vec.norm();
}

fn reflect(v: &Vector3<f64>, n: &Vector3<f64>) -> Vector3<f64> {
    v - 2.0*v.dot(&n)*n
}
