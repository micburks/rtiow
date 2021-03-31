
use crate::vec::Vec3;
use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::math::clamp;

pub enum Material {
    Lambertian(Vec3),
    Metal(Vec3, f64),
}

impl Material {
    pub fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<ScatterResult> {
        match self {
            Material::Lambertian(color) => {
                // hemispherical scattering diffuse method
                // let mut scatter_direction = random_in_hemisphere(&record.normal);
                let mut scatter_direction = record.normal.add(&Vec3::random_unit_vector());

                if Vec3::near_zero(&scatter_direction) {
                    scatter_direction = record.normal.clone();
                }

                let scattered = Ray::new(record.point.clone(), scatter_direction);
                Some(ScatterResult {
                    ray: scattered,
                    attenuation: color.clone(),
                })
            },
            Material::Metal(color, fuzz) => {
                let fuzz = clamp(*fuzz, 0.0, 1.0);
                let fuzz_vector = Vec3::random_in_unit_sphere().scale(fuzz);
                let scatter_direction = ray.direction.unit().reflect(&record.normal).add(&fuzz_vector);
                if scatter_direction.dot(&record.normal) > 0.0 {
                    let scattered = Ray::new(record.point.clone(), scatter_direction);
                    Some(ScatterResult {
                        ray: scattered,
                        attenuation: color.clone(),
                    })
                } else {
                    None
                }
            },
        }
    }
    pub fn clone(&self) -> Material {
        match self {
            Material::Lambertian(color) => Material::Lambertian(color.clone()),
            Material::Metal(color, fuzz) => Material::Metal(color.clone(), *fuzz),
        }
    }
}

pub struct ScatterResult {
    pub ray: Ray,
    pub attenuation: Vec3,
}
