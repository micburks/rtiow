
use crate::vec::Vec3;
use crate::ray::Ray;
use crate::math::degrees_to_radians;

pub struct Camera {
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
    u: Vec3,
    v: Vec3,
    // w: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        origin: Vec3,
        lookat: Vec3,
        up: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
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
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
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
