
mod vec;
mod file_format;
mod ray;
mod sphere;
mod hittable;
mod camera;
mod material;
mod math;

use vec::Vec3;
use file_format::Format;
use ray::Ray;
use sphere::Sphere;
use hittable::{HittableList};
use camera::Camera;
use material::Material;
use math::{clamp, random, random_clamped};
use std::f64;

fn main() {
    let aspect_ratio = 3.0 / 2.0;
    let image_width: u32 = 1200;
    let image_height = ((image_width as f64) / aspect_ratio) as u32;
    let max_color = 255;
    let samples_per_pixel = 10;
    let max_depth = 50;

    let origin = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let up = Vec3::new(0.0, 1.0, 0.0);
    let vfov = 20.0;
    let aperture = 0.1;
    let dist_to_focus = 10.0;
    let camera = Camera::new(origin, lookat, up, vfov, aspect_ratio, aperture, dist_to_focus);

    let world = random_scene();

    let file_format = Format::PP3(image_width, image_height, max_color);
    println!("{}", file_format.print_header());

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

fn random_scene<'a>() -> HittableList<'a> {
    let mut world = HittableList::new();

    let ground_material = Material::Lambertian(Vec3::new(0.5, 0.5, 0.5));
    world.add(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, ground_material));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random();
            let center = Vec3::new(a as f64 + 0.9 * random(), 0.2, b as f64 + 0.9 * random());

            if center.sub(&Vec3::new(4.0, 0.2, 0.0)).len() > 0.9 {
                let sphere_material;
                if choose_mat < 0.8 {
                    // diffuse
                    let color = Vec3::random_in_unit_sphere().mul(&Vec3::random_in_unit_sphere());
                    sphere_material = Material::Lambertian(color);
                    world.add(Sphere::new(center, 0.2, sphere_material));
                } else if choose_mat < 0.95 {
                    // metal
                    let color = Vec3::random_in_unit_sphere().mul(&Vec3::new(0.5, 1.0, 1.0));
                    let fuzz = random_clamped(0.0, 0.5);
                    sphere_material = Material::Metal(color, fuzz);
                    world.add(Sphere::new(center, 0.2, sphere_material));
                } else {
                    // glass
                    sphere_material = Material::Dielectric(1.5);
                    world.add(Sphere::new(center, 0.2, sphere_material));
                }
            }
        }
    }

    let material1 = Material::Dielectric(1.5);
    world.add(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, material1));

    let material2 = Material::Lambertian(Vec3::new(0.4, 0.2, 0.1));
    world.add(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, material2));

    let material3 = Material::Metal(Vec3::new(0.7, 0.6, 0.5), 0.0);
    world.add(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, material3));

    world
}
