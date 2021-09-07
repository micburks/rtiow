
use crate::vec::Vec3;
use crate::ray::Ray;
use crate::math::degrees_to_radians;
use num_traits::Float;

pub struct Camera<T: Float> {
    origin: Vec3<T>,
    horizontal: Vec3<T>,
    vertical: Vec3<T>,
    lower_left_corner: Vec3<T>,
    u: Vec3<T>,
    v: Vec3<T>,
    // w: Vec3<T>,
    lens_radius: f32,
}

impl<T: Float> Camera<T> {
    pub fn new(
        origin: Vec3<T>,
        lookat: Vec3<T>,
        up: Vec3<T>,
        vfov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Camera<T> {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = origin.sub(&lookat).unit();
        let u = up.cross(&w).unit();
        let v = w.cross(&u);

        let horizontal = u.scale(viewport_width * focus_dist); // Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = v.scale(viewport_height * focus_dist); // Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner = origin.sub(&horizontal.scale(0.5))
            .sub(&vertical.scale(0.5))
            .sub(&w.scale(focus_dist));

        let lens_radius = aperture / 2.0;

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            // w,
            lens_radius,
        }
    }
    pub fn get_ray(&self, s: f32, t: f32) -> Ray<T> {
        let rd = Vec3::random_in_unit_disk().scale(self.lens_radius);
        let offset = self.u.scale(rd.x).add(&self.v.scale(rd.y));
        let offset_origin = self.origin.add(&offset);
        Ray::new(
            offset_origin.clone(),
            self.lower_left_corner.add(&self.horizontal.scale(s))
            .add(&self.vertical.scale(t))
            .sub(&offset_origin),
            )

    }
}
