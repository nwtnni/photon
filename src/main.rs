use std::collections::HashMap;

use rayon::prelude::*;

use photon::geometry::{Ray, Vec3};
use photon::material::{Material, Diffuse, Metal, Dielectric};
use photon::surface::{Surface, Sphere, List, Hit};
use photon::camera::Camera;
use photon::preview::Preview;
use photon::ppm::PPM;

fn color(ray: &Ray, scene: &Surface, depth: i32) -> Vec3 {
    let mut hit = Hit::default();
    if scene.hit(ray, 0.001, std::f32::MAX, &mut hit) {
        let mut attenuation = Vec3::default();
        let mut scattered = Ray::default();
        if depth < 50 && hit.m.unwrap().scatter(ray, &hit, &mut attenuation, &mut scattered) {
            color(&scattered, scene, depth + 1) * attenuation
        } else {
            Vec3::default()
        }
    } else {
        let dir = ray.d().normalize();
        let t = 0.5 * (dir.y() + 1.0);
        let white = Vec3::new(1.0, 1.0, 1.0);
        let blue = Vec3::new(0.5, 0.7, 1.0);
        white.lerp(&blue, t)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let nx = 1920;
    let ny = 1080;
    let ns = 100;

    let mut ppm = PPM::new(nx, ny, "test.ppm")?;
    let (tx, rx) = crossbeam::channel::unbounded();
    let preview = Preview::new(nx, ny, rx);
    let handle = std::thread::spawn(|| preview.run());

    let origin = Vec3::new(10.0, 3.0, 10.0);
    let toward = Vec3::new(0.0, 0.0, 0.0);
    let up = Vec3::new(0.0, 1.0, 0.0);
    let focus = 12.0;
    let aperture = 0.5;
    let camera = Camera::new(origin, toward, up, 20.0, nx as f32 / ny as f32, aperture, focus);

    let mut scene = List::default();
    let mut diffuse = Vec::new();
    let mut dielectric = Vec::new();
    let mut metallic = Vec::new();
    let mut spheres = Vec::new();
    let mut materials = Vec::new();
    let mut centers = Vec::new();

    macro_rules! rand { () => { rand::random::<f32>() } };

    for x in (-10..=10).map(|x| x as f32) {
        for y in (-10..=10).map(|y| y as f32) {
            let material_chance = rand::random::<f32>();
            let center = Vec3::new(x + 0.9 * rand!(), 0.2, y + 0.9 * rand!());
            if (center - Vec3::new(4.0, 0.2, 0.0)).len() > 0.9 {
                centers.push(center);
                if material_chance < 0.8 {
                    diffuse.push(Diffuse::new(Vec3::new(
                        rand!() * rand!(),
                        rand!() * rand!(),
                        rand!() * rand!(),
                    )));
                    materials.push(0u8);
                } else if material_chance < 0.95 {
                    metallic.push(Metal::new(
                        Vec3::new(
                            (rand!() + 1.0) * 0.5,
                            (rand!() + 1.0) * 0.5,
                            (rand!() + 1.0) * 0.5,
                        ),
                        rand!() * 0.5,
                    ));
                    materials.push(1u8);
                } else {
                    dielectric.push(Dielectric::new(rand!() * 2.0));
                    materials.push(2u8);
                }
            }
        }
    }

    let mut diffuse_iter = diffuse.iter();
    let mut dielectric_iter = dielectric.iter();
    let mut metallic_iter = metallic.iter();
    for (m, c) in materials.into_iter().zip(centers.into_iter()) {
        let material = match m {
        | 0 => diffuse_iter.next().unwrap() as &dyn Material,
        | 1 => metallic_iter.next().unwrap() as &dyn Material,
        | 2 => dielectric_iter.next().unwrap() as &dyn Material,
        | _ => unreachable!(),
        };
        spheres.push(Sphere::new(c, 0.2, material));
    }

    for sphere in &spheres { scene.push(sphere); }

    let gray = Diffuse::new(Vec3::new(0.5, 0.5, 0.5));
    let diffuse = Diffuse::new(Vec3::new(0.2, 0.5, 0.6));
    let glass = Dielectric::new(1.5);
    let metal = Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0);

    let floor = Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, &gray);
    let far = Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, &diffuse);
    let mid = Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, &glass);
    let close = Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, &metal);

    scene.push(&floor);
    scene.push(&far);
    scene.push(&mid);
    scene.push(&close);

    let mut buffers: HashMap<usize, Vec<(u8, u8, u8)>> = HashMap::with_capacity(ny);
    buffers.par_extend((0..ny).into_par_iter().map(move |y| {
        let mut buffer = Vec::with_capacity(nx);
        for x in 0..nx {
            let mut c = Vec3::default();
            for _ in 0..ns {
                let u = (x as f32 + rand::random::<f32>()) / nx as f32;
                let v = (y as f32 + rand::random::<f32>()) / ny as f32;
                let r = camera.get(u, v);
                c += color(&r, &scene, 0) / ns as f32;
            }
            let rgb = (
                (c[0].sqrt() * 255.99) as u8,
                (c[1].sqrt() * 255.99) as u8,
                (c[2].sqrt() * 255.99) as u8,
            );
            tx.send((x, y, rgb)).ok();
            buffer.push(rgb);
        }
        (y, buffer)
    }));

    for y in (0..ny).rev() {
        for (r, g, b) in buffers.remove(&y).unwrap().into_iter() {
            ppm.write(r, g, b)?;
        }
    }

    handle.join().unwrap();
    Ok(())
}
