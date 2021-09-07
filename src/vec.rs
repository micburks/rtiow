
use crate::math::random_clamped;
use num_traits::Float;

pub struct Vec3<T: Float> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Float> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Vec3<T> {
        Vec3 { x, y, z }
    }
    pub fn add(&self, other: &Vec3<T>) -> Vec3<T> {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
    pub fn sub(&self, other: &Vec3<T>) -> Vec3<T> {
        let other_neg = other.neg();
        self.add(&other_neg)
    }
    pub fn mul(&self, other: &Vec3<T>) -> Vec3<T> {
        Vec3::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
    pub fn scale(&self, scalar: f32) -> Vec3<T> {
        Vec3::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
    pub fn neg(&self) -> Vec3<T> {
        self.scale(-1.0)
    }
    pub fn length_squared(&self) -> f32 {
        self.dot(self)
    }
    pub fn len(&self) -> f32 {
        self.length_squared().sqrt()
    }
    pub fn unit(&self) -> Vec3<T> {
        let length = self.len();
        self.scale((1.0 / length) as T)
    }
    pub fn dot(&self, other: &Vec3<T>) -> f32 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }
    pub fn cross(&self, other: &Vec3<T>) -> Vec3<T> {
        Vec3::new(
            (self.y * other.z) - (self.z * other.y),
            (self.z * other.x) - (self.x * other.z),
            (self.x * other.y) - (self.y * other.x),
            )
    }
    pub fn clone(&self) -> Vec3<T> {
        Vec3::new(self.x, self.y, self.z)
    }
    pub fn apply<R: Fn(T) -> T>(&self, fun: R) -> Vec3<T> {
        Vec3::new(fun(self.x), fun(self.y), fun(self.z))
    }
    pub fn reflect(&self, other: &Vec3<T>) -> Vec3<T> {
        self.sub(&other.scale(2.0 * self.dot(&other)))
    }
    pub fn refract(&self, normal: &Vec3<T>, etai_over_etat: T) -> Vec3<T> {
        let cos_theta = self.neg().dot(&normal).min(1.0);
        let perpendicular = normal.scale(cos_theta).add(&self).scale(etai_over_etat);
        let coefficient = (1.0 - perpendicular.length_squared()).abs().sqrt();
        let parallel = normal.scale(-coefficient);
        perpendicular.add(&parallel)
    }
}

impl<T: Float> Vec3<T> {
    pub fn near_zero(v: &Vec3<T>) -> bool {
        let tolerance = 1e-8;
        (v.x.abs() < tolerance)
            && (v.y.abs() < tolerance)
            && (v.z.abs() < tolerance)
    }
    pub fn random_unit_vector() -> Vec3<f32> {
        Vec3::random_in_unit_sphere().unit()
    }
    /*
    pub fn random_in_hemisphere(normal: &Vec3<T>) -> Vec3<T> {
        let r = Vec3<f32>::random_in_unit_sphere();
        if r.dot(&normal) > 0.0 {
            r
        } else {
            r.neg()
        }
    }
    */
    pub fn random_in_unit_sphere() -> Vec3<f32> {
        loop {
            let v = Vec3::new(
                random_clamped(-1.0, 1.0),
                random_clamped(-1.0, 1.0),
                random_clamped(-1.0, 1.0),
                );
            if v.length_squared() < 1.0 {
                return v;
            }
        }
    }
    pub fn random_in_unit_disk() -> Vec3<f32> {
        loop {
            let v = Vec3::new(
                random_clamped(-1.0, 1.0),
                random_clamped(-1.0, 1.0),
                0.0,
                );
            if v.length_squared() < 1.0 {
                return v;
            }
        }
    }
}
