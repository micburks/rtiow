
use crate::vec::Vec3;
use crate::ray::Ray;
use crate::material::Material;
use num_traits::Float;

pub trait Hittable<T: Float> {
    fn hit(&self, ray: &Ray<T>, t_min: f32, t_max: f32) -> Option<HitRecord<T>>;
}

pub struct HitRecord<T: Float> {
    pub point: Vec3<T>,
    pub normal: Vec3<T>,
    pub t: f32,
    pub front_face: bool,
    pub material: Material<T>,
}

impl<T: Float> HitRecord<T> {
    pub fn new(ray: &Ray<T>, t: f32, normal: Vec3<T>, material: Material<T>) -> HitRecord<T> {
        let point = ray.at(t);
        let outward_normal;
        let front_face;
        if ray.direction.dot(&normal) < 0.0 {
            front_face = true;
            outward_normal = normal;
        } else {
            front_face = false;
            outward_normal = normal.neg();
        }
        HitRecord { point, normal: outward_normal, t, front_face, material }
    }
}

pub struct HittableList<'a, T>(
    Vec<Box<dyn Hittable<T> + 'a>>
);

impl<'a, T: Float> HittableList<'a, T> {
    pub fn new() -> HittableList<'a, T> {
        HittableList(vec![])
    }
    pub fn add(&mut self, i: impl Hittable<T> + 'a) {
        self.0.push(Box::new(i))
    }
    pub fn hit(&self, ray: &Ray<T>, t_min: f32, t_max: f32) -> Option<HitRecord<T>> {
        let mut closest = t_max;
        let mut closest_hit = None;
        for hittable in &self.0 {
            if let Some(hit_record) = hittable.hit(ray, t_min, closest) {
                closest = hit_record.t;
                closest_hit = Some(hit_record);
            }
        }
        closest_hit
    }
}
