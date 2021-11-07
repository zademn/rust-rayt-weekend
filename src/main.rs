mod camera;
mod color;
mod hittable;
mod material;
mod ray;
mod sphere;
mod utils;
mod vec3;
use camera::Camera;
use color::write_color;
use hittable::{Hittable, HittableList};
use indicatif::ProgressBar;
use material::{Dielectric, Lambertian, Material, Metal};

use ray::Ray;
use rayon::prelude::*;
use sphere::Sphere;
use std::io;
use std::rc::Rc;
use std::sync::Arc;
use vec3::{Color, Point3, Vec3};

const ASPECT_RATIO: f32 = 1. / 1.;
const IMAGE_WIDTH: i32 = 512;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;
const SAMPLES_PER_PIXEL: u32 = 100;
const MAX_DEPTH: u32 = 50;

fn random_scene() -> HittableList {
    let mut world = HittableList::new();
    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let ground_sphere = Sphere::new(Point3::new(0., -1000., 0.), 1000., ground_material);
    world.add(Box::new(ground_sphere));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand::random::<f64>();
            let center: Point3 = Point3::new(
                a as f64 + 0.9 * rand::random::<f64>(),
                0.2,
                b as f64 + rand::random::<f64>(),
            );
            if (center - Point3::new(4., 0.2, 0.)).norm() > 0.9 {
                let sphere_material: Arc<dyn Material>;
                if choose_mat < 0.8 {
                    // Diffuse
                    let albedo = Color::random() * Color::random();
                    sphere_material = Arc::new(Lambertian::new(albedo));
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = Color::random_interval(0.5, 1.);
                    let fuzz = rand::random::<f64>() / 2.;
                    sphere_material = Arc::new(Metal::new(albedo, fuzz));
                } else {
                    // Glass
                    let ir = 1.5;
                    sphere_material = Arc::new(Dielectric::new(ir));
                }
                let world_sphere = Sphere::new(center, 0.2, sphere_material);
                world.add(Box::new(world_sphere));
            }
        }
    }
    let material1 = Arc::new(Dielectric::new(1.5));
    let sphere1 = Sphere::new(Point3::new(0., 1., 0.), 1., material1);
    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    let sphere2 = Sphere::new(Point3::new(-4., 1., 0.), 1., material2);
    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    let sphere3 = Sphere::new(Point3::new(0., 1., 0.), 1., material3);
    world.add(Box::new(sphere1));
    world.add(Box::new(sphere2));
    world.add(Box::new(sphere3));
    world
}
/// Given a ray and a list of objects, returns the color of the ray.
fn ray_color(r: &Ray, world: &HittableList, depth: u32) -> Color {
    // Check recursion depth
    if depth == 0 {
        return Color::zero();
    }
    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        if let Some((attenuation, scattered)) = rec.material.scatter(r, &rec) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        } else {
            return Color::zero();
        }
    }

    let unit_direction: Vec3 = r.dir().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.);
    (1.0 - t) * Color::one() + t * Color::new(0.5, 0.7, 1.)
}
/// Shoots a ray on the pixel and returns the color
fn shoot_ray(i: i32, j: i32, world: &HittableList, cam: &Camera) -> Color {
    let u = (i as f64 + rand::random::<f64>()) / (IMAGE_WIDTH - 1) as f64;
    let v = (j as f64 + rand::random::<f64>()) / (IMAGE_HEIGHT - 1) as f64;
    let r = cam.get_ray(u, v);
    ray_color(&r, &world, MAX_DEPTH)
}
fn main() {
    // Image

    // World
    // let r = (std::f64::consts::PI / 4.).cos();
    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.0, 0.0, 0.25)));
    let material_left = Arc::new(Lambertian::new(Color::new(0.0, 0.0, 0.5)));
    let material_right = Arc::new(Lambertian::new(Color::new(0.0, 0.0, 1.)));

    // let mut world = HittableList::new();

    // world.add(Box::new(Sphere::new(
    //     Point3::new(0., -100.5, -1.),
    //     100.,
    //     material_ground,
    // )));
    // world.add(Box::new(Sphere::new(
    //     Point3::new(0., 0., -1.),
    //     0.5,
    //     material_center,
    // )));
    // world.add(Box::new(Sphere::new(
    //     Point3::new(-1., 0., -1.),
    //     0.5,
    //     material_left,
    // )));
    // world.add(Box::new(Sphere::new(
    //     Point3::new(1., 0., -1.),
    //     0.5,
    //     material_right,
    // )));

    let world = random_scene();
    // Camera
    //let cam = Camera::default();
    let origin = Point3::new(11., 2., 7.);
    let lookat = Point3::new(0., 0., 0.);
    let vup = Vec3::new(0., 1., 0.);
    let vfov = 20.;
    let dist_to_focus = 10.;
    let aperture = 0.1;

    let cam = Camera::new(
        ASPECT_RATIO as f64,
        vfov,
        aperture,
        dist_to_focus,
        origin,
        lookat,
        vup,
    );

    // Render

    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
    let pb = ProgressBar::new(IMAGE_HEIGHT as u64);
    for j in (0..IMAGE_HEIGHT).rev() {
        pb.inc(1);
        for i in 0..IMAGE_WIDTH {
            // let mut pixel_color = Color::zero();
            // for _ in 0..SAMPLES_PER_PIXEL {
            //     let u = (i as f64 + rand::random::<f64>()) / (IMAGE_WIDTH - 1) as f64;
            //     let v = (j as f64 + rand::random::<f64>()) / (IMAGE_HEIGHT - 1) as f64;
            //     let r = cam.get_ray(u, v);
            //     pixel_color += ray_color(&r, &world, MAX_DEPTH);
            // }
            let pixel_color = (0..SAMPLES_PER_PIXEL)
                .into_par_iter()
                .map(|_| shoot_ray(i, j, &world, &cam))
                .sum();

            write_color(&mut io::stdout(), pixel_color, SAMPLES_PER_PIXEL);
        }
    }
}
