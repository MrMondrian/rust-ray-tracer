mod material;
mod ray;
mod hitable_list;
mod hitable;
mod interval;
mod camera;
use crate::camera::Camera;
use crate::hitable_list::HitableList;
use crate::hitable::Hitable::*;
use nalgebra::Vector3;
use crate::material::Material::*;

fn main() {

    let mut world = HitableList::new();


    let mat_ground =  Lambertian{albedo: Vector3::new(0.8,0.8,0.0)};
    let mat_center = Lambertian{albedo: Vector3::new(0.1,0.2,0.5)};
    let mat_left = Metal{albedo: Vector3::new(0.8,0.8,0.8)};
    let mat_right = Metal{albedo: Vector3::new(0.8,0.6,0.2)};

    world.add(Sphere{center: Vector3::new(0.0,-100.5,-1.0),radius: 100.0,mat: mat_ground});
    world.add(Sphere{center: Vector3::new(0.0,0.0,-1.2),radius: 0.5,mat: mat_center});
    world.add(Sphere{center: Vector3::new(-1.0,0.0,-1.0),radius: 0.5,mat: mat_left});
    world.add(Sphere{center: Vector3::new(1.0,0.0,-1.0),radius: 0.5,mat: mat_right});

    let mut cam = Camera::new();

    cam.render(&world) 
}
