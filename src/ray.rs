
use crate::vec::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(orig: Vec3, dir: Vec3) -> Ray {
        Ray {
            origin: orig,
            direction: dir,
        }
    }
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin.add(&self.direction.scale(t))
    }
}
