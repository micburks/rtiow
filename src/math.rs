
use rand::Rng;
use std::f64::consts::PI;

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x <= min {
        min
    } else if x >= max {
        max
    } else {
        x
    }
}

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

pub fn random_clamped(min: f64, max: f64) -> f64 {
    min + ((max - min) * random())
}

pub fn reflectance(cos_theta: f64, refraction_index: f64) -> f64 {
    // Use Schlick's approximation for reflectance.
    let r0 = ((1.0 - refraction_index) / (1.0 + refraction_index)).powi(2);
    r0 + ((1.0 - r0) * (1.0 - cos_theta).powi(5))
}


