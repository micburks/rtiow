
use rand::Rng;
use std::f32::consts::PI;

pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x <= min {
        min
    } else if x >= max {
        max
    } else {
        x
    }
}

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

pub fn random() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

pub fn random_clamped(min: f32, max: f32) -> f32 {
    min + ((max - min) * random())
}

pub fn reflectance(cos_theta: f32, refraction_index: f32) -> f32 {
    // Use Schlick's approximation for reflectance.
    let r0 = ((1.0 - refraction_index) / (1.0 + refraction_index)).powi(2);
    r0 + ((1.0 - r0) * (1.0 - cos_theta).powi(5))
}


