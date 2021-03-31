
use crate::vec::Vec3;
use crate::ray::Ray;
use crate::material::Material;

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Material,
}

impl HitRecord {
    pub fn new(ray: &Ray, t: f64, normal: Vec3, material: Material) -> HitRecord {
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

pub struct HittableList<'a>(
    Vec<Box<dyn Hittable + 'a>>
);

impl<'a> HittableList<'a> {
    pub fn new() -> HittableList<'a> {
        HittableList(vec![])
    }
    pub fn add(&mut self, i: impl Hittable + 'a) {
        self.0.push(Box::new(i))
    }
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
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
