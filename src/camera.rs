
use crate::vec::Vec3;
use crate::ray::Ray;

pub struct Camera {
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64) -> Camera {
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let origin = Vec3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let focal_length = 1.0;
        let lower_left_corner = origin.sub(&horizontal.scale(0.5))
            .sub(&vertical.scale(0.5))
            .sub(&Vec3::new(0.0, 0.0, focal_length));

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }
    pub fn get_ray(&self, x: f64, y: f64) -> Ray {
        Ray::new(
            self.origin.clone(),
            self.lower_left_corner.add(&self.horizontal.scale(x))
            .add(&self.vertical.scale(y))
            .sub(&self.origin)
            )

    }
}
