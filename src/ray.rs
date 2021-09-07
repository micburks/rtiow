
use crate::vec::Vec3;
use num_traits::Float;

pub struct Ray<T: Float> {
    pub origin: Vec3<T>,
    pub direction: Vec3<T>,
}

impl<T: Float> Ray<T> {
    pub fn new(orig: Vec3<T>, dir: Vec3<T>) -> Ray<T> {
        Ray {
            origin: orig,
            direction: dir,
        }
    }
    pub fn at(&self, t: f32) -> Vec3<T> {
        self.origin.add(&self.direction.scale(t))
    }
}
