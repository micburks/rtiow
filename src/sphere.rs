
use crate::hittable::{Hittable, HitRecord};
use crate::vec::Vec3;
use crate::ray::Ray;
use crate::material::Material;
use num_traits::Float;

pub struct Sphere<T: Float> {
    pub center: Vec3<T>,
    pub radius: T,
    pub material: Material<T>,
}

impl<T: Float> Sphere<T> {
    pub fn new(center: Vec3<T>, radius: T, material: Material<T>) -> Sphere<T> {
        Sphere { center, radius, material }
    }
}

impl<T: Float> Hittable<T> for Sphere<T> {
    fn hit(&self, ray: &Ray<T>, t_min: f32, t_max: f32) -> Option<HitRecord<T>> {
        let oc = ray.origin.sub(&self.center);
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - (self.radius * self.radius);
        let discriminant = (half_b * half_b) - (a * c);
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt() as f32;
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }
        let point = ray.at(root);
        Some(
            HitRecord::new(
                ray,
                root,
                (point.sub(&self.center)).scale(1.0 / self.radius),
                self.material.clone(),
                )
            )
    }
}
