mod material;
mod ray;
mod hitable_list;
mod hitable;
mod interval;
mod camera;
use crate::camera::Camera;
use crate::hitable_list::HitableList;
use crate::hitable::Sphere;
use nalgebra::Vector3;
use crate::material::Lambertian;
use crate::material::Metal;

fn main() {

    let mut world = HitableList::new();


    let mat_ground =  Lambertian::new(Vector3::new(0.8,0.8,0.0));
    let mat_center = Lambertian::new(Vector3::new(0.1,0.2,0.5));
    let mat_left = Metal::new(Vector3::new(0.8,0.8,0.8));
    let mat_right = Metal::new(Vector3::new(0.8,0.6,0.2));

    world.add(Box::new(Sphere::new(Vector3::new(0.0,-100.5,-1.0),100.0,mat_ground)));
    world.add(Box::new(Sphere::new(Vector3::new(0.0,0.0,-1.2),0.5,mat_center)));
    world.add(Box::new(Sphere::new(Vector3::new(-1.0,0.0,-1.0),0.5,mat_left)));
    world.add(Box::new(Sphere::new(Vector3::new(1.0,0.0,-1.0),0.5,mat_right)));

    let mut cam = Camera::new();

    cam.render(&world) 
}
