
mod vec;
mod file_format;
mod ray;
mod sphere;
mod hittable;
mod camera;
mod material;
mod math;

use vec::{Vec3, random};
use file_format::Format;
use ray::Ray;
use sphere::Sphere;
use hittable::{HittableList};
use camera::Camera;
use material::Material;
use math::clamp;
use std::f64::consts::PI;
use std::f64;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height = ((image_width as f64) / aspect_ratio) as u32;
    let max_color = 255;
    let camera = Camera::new(aspect_ratio);
    let samples_per_pixel = 100;
    let max_depth = 50;

    let file_format = Format::PP3(image_width, image_height, max_color);
    println!("{}", file_format.print_header());

    let mut world = HittableList::new();

    let material_ground = Material::Lambertian(Vec3::new(0.8, 0.8, 0.0));
    let material_center = Material::Lambertian(Vec3::new(0.7, 0.3, 0.3));
    let material_left   = Material::Metal(Vec3::new(0.8, 0.8, 0.8), 0.3);
    let material_right  = Material::Metal(Vec3::new(0.8, 0.6, 0.2), 1.0);

    world.add(Sphere::new(Vec3::new( 0.0, -100.5, -1.0), 100.0, material_ground));
    world.add(Sphere::new(Vec3::new( 0.0,    0.0, -1.0), 0.5, material_center));
    world.add(Sphere::new(Vec3::new(-1.0,    0.0, -1.0), 0.5, material_left));
    world.add(Sphere::new(Vec3::new( 1.0,    0.0, -1.0), 0.5, material_right));

    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let mut color = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + random()) as f64 / (image_width as f64 - 1.0);
                let v = (j as f64 + random()) as f64 / (image_height as f64 - 1.0);
                let ray = camera.get_ray(u, v);
                color = color.add(&ray_color(&ray, &world, max_depth));
            }
            write_color(&color, &file_format, samples_per_pixel);
        }
    }
    eprintln!("Done.");
}

fn ray_color(ray: &Ray, world: &HittableList, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }
    let t_min = 0.001;
    let t_max = f64::INFINITY;
    if let Some(record) = world.hit(&ray, t_min, t_max) {
        if let Some(scattered) = record.material.scatter(&ray, &record) {
            return scattered.attenuation.mul(&ray_color(&scattered.ray, world, depth - 1));
        } else {
            return Vec3::new(0.0, 0.0, 0.0);
        }
    }
    let unit_dir = ray.direction.unit();
    let t = (unit_dir.y + 1.0) * 0.5; // 0.0 <= t <= 1.0
    let white = Vec3::new(1.0, 1.0, 1.0);
    let blue = Vec3::new(0.5, 0.7, 1.0);
    white.scale(1.0 - t).add(&blue.scale(t))
}

fn write_color(color: &Vec3, file_format: &Format, samples_per_pixel: u32) {
    let color = color.scale(1.0 / samples_per_pixel as f64)
        .apply(|val| val.sqrt()) // gamma = 2 correction
        .apply(|val| clamp(val, 0.0, 0.999))
        .scale(file_format.max_color() as f64);
    println!("{} {} {}", color.x as i32, color.y as i32, color.z as i32);
}

fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
