use crate::hitable::Hitable;
use crate::hitable::HitRecord;
use crate::ray::Ray;
use std::vec::Vec;


pub struct HitableList {
    pub objects: Vec<Box<dyn Hitable>>,
}

impl HitableList {
    pub fn new() -> Self {
        let objects = Vec::new();
        Self{objects}
    }

    // pub fn clear(&mut self) -> () {
    //     self.objects.clear();
    // }

    pub fn add(&mut self, object: Box<dyn Hitable>) -> () {
        self.objects.push(object);
    }

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut record: Option<HitRecord> = None;

        for object in self.objects.iter() {
            if let Some(x) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = x.t;
                record = Some(x);
            }
        }
        return record;
    }
}
