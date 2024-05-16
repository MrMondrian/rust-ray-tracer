
mod ray;
mod hitable_list;
mod hitable;
mod interval;
mod camera;
use crate::camera::Camera;
use crate::hitable_list::HitableList;
use crate::hitable::Sphere;
use nalgebra::Vector3;

fn main() {

    let mut world = HitableList::new();

    world.add(Box::new(Sphere::new(Vector3::new(0.0,0.0,-1.0),0.5)));
    world.add(Box::new(Sphere::new(Vector3::new(0.0,-100.5,-1.0),100.0)));

    let mut cam = Camera::new();

    cam.render(&world) 
}
