
use rand::Rng;

pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }
    pub fn add(&self, other: &Vec3) -> Vec3 {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
    pub fn sub(&self, other: &Vec3) -> Vec3 {
        let other_neg = other.neg();
        self.add(&other_neg)
    }
    pub fn mul(&self, other: &Vec3) -> Vec3 {
        Vec3::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
    pub fn scale(&self, scalar: f64) -> Vec3 {
        Vec3::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
    pub fn neg(&self) -> Vec3 {
        self.scale(-1.0)
    }
    pub fn length_squared(&self) -> f64 {
        self.dot(self)
    }
    pub fn len(&self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn unit(&self) -> Vec3 {
        let length = self.len();
        self.scale((1.0 / length) as f64)
    }
    pub fn dot(&self, other: &Vec3) -> f64 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }
    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3::new(
            (self.y * other.z) - (self.z * other.y),
            (self.z * other.x) - (self.x * other.z),
            (self.x * other.y) - (self.y * other.x),
            )
    }
    pub fn clone(&self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }
    pub fn apply<T: Fn(f64) -> f64>(&self, fun: T) -> Vec3 {
        Vec3::new(fun(self.x), fun(self.y), fun(self.z))
    }
    pub fn reflect(&self, other: &Vec3) -> Vec3 {
        self.sub(&other.scale(2.0 * self.dot(&other)))
    }
}

impl Vec3 {
    pub fn near_zero(v: &Vec3) -> bool {
        let tolerance = 1e-8;
        (v.x.abs() < tolerance)
            && (v.y.abs() < tolerance)
            && (v.z.abs() < tolerance)
    }
    pub fn random_unit_vector() -> Vec3 {
        Vec3::random_in_unit_sphere().unit()
    }
    pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
        let r = Vec3::random_in_unit_sphere();
        if r.dot(&normal) > 0.0 {
            r
        } else {
            r.neg()
        }
    }
    pub fn random_in_unit_sphere() -> Vec3 {
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
}

pub fn random() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

fn random_clamped(min: f64, max: f64) -> f64 {
    min + ((max - min) * random())
}

