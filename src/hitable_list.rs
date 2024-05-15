use hitable::Hitable;
use hitable::HitRecord;
use std::vec::Vec;


pub struct HitableList {
    pub objects: Vec<Hitable>;
}

impl HitableList {
    fn new() -> Self {
        let mut objects = Vec::new();
        Self{objects};
    }

    fn clear(&self) -> () {
        self.objects.clear();
    }

    fn add(&self, object: Hitable) -> () {
        self.objects.push(object);
    }

    fn hit(&self, &ray: Ray, t_min: f64, t_max: f64) -> Option(HitRecord) {
        let hit_anything = false;
        let closest_so_far = t_max;
        let record: Option(HitRecord) = None;

        for object in self.objects {
            if let Some(x) = object.hit(ray, t_min, closest_so_far) {
                hit_anything = true;
                closest_so_far = x.t;
                record = x;
            }
        }
        return record;
    }
}
