use crate::hitable::Hitable;
use crate::interval::Interval;
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

    pub fn hit(&self, ray: &Ray, mut bounds: Interval) -> Option<HitRecord> {
        let mut record: Option<HitRecord> = None;

        for object in self.objects.iter() {
            if let Some(x) = object.hit(ray, &bounds) {
                bounds.shrink_right(x.t);
                record = Some(x);
            }
        }
        return record;
    }
}
