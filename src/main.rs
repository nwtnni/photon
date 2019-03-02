use std::collections::HashMap;

use rayon::prelude::*;

use photon::arena::{Arena, CopyArena};
use photon::bvh;
use photon::geometry::{Ray, Vec3};
use photon::material::{Material, Diffuse, Metal, Dielectric};
use photon::surface::{Surface, Sphere, List, Hit};
use photon::camera::Camera;
use photon::preview::Preview;

/// Main ray tracing function.
/// Intersects `ray` with `scene`, potentially recursing upon reflecting or refracting.
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

fn render(
    nx: usize, 
    ny: usize,
    ns: usize,
    tx: crossbeam::channel::Sender<(usize, usize, (u8, u8, u8))>,
    camera: &Camera,
    scene: &Surface
) {

    // Row to pixel buffer map
    let mut buffers: HashMap<usize, Vec<(u8, u8, u8)>> = HashMap::with_capacity(ny);

    // Allow each thread exclusive access to a single row 
    buffers.par_extend((0..ny).into_par_iter().map(move |y| {
        let mut buffer = Vec::with_capacity(nx);
        for x in 0..nx {
            let mut c = Vec3::default();
            for _ in 0..ns {
                let u = (x as f32 + rand::random::<f32>()) / nx as f32;
                let v = (y as f32 + rand::random::<f32>()) / ny as f32;
                let r = camera.get(u, v);
                c += color(&r, scene, 0) / ns as f32;
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

    // Collect buffers for PNG encoding
    let buffer = (0..ny).rev()
        .flat_map(|y| buffers.remove(&y).unwrap().into_iter())
        .collect::<Vec<_>>();

    lodepng::encode24_file("out.png", &buffer, nx, ny)
        .unwrap();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let nx = 1920; // Width
    let ny = 1080; // Height
    let ns = 250;  // Samples per pixel

    let (tx, rx) = crossbeam::channel::unbounded();
    let preview = Preview::new(nx, ny, rx);
    let handle = std::thread::spawn(|| preview.run());
    let arena = CopyArena::new(2 * 16 * 1024);

    // Camera setup
    let origin = Vec3::new(10.0, 3.0, 10.0);
    let toward = Vec3::new(0.0, 0.0, 0.0);
    let up = Vec3::new(0.0, 1.0, 0.0);
    let fov = 20.0;
    let aspect = nx as f32 / ny as f32;
    let focus = 12.0;
    let aperture = 0.20;
    let open = 0.0;
    let shut = 1.0;
    let camera = Camera::new(origin, toward, up, fov, aspect, aperture, focus, open, shut);

    let mut surfaces = Vec::new();

    macro_rules! rand { () => { rand::random::<f32>() } };

    for x in (-10..=10).map(|x| x as f32) {
        for y in (-10..=10).map(|y| y as f32) {
            let material_chance = rand::random::<f32>();
            let center = Vec3::new(x + 0.9 * rand!(), 0.2, y + 0.9 * rand!());
            if (center - Vec3::new(4.0, 0.2, 0.0)).len() <= 0.9 { continue }
            let material = if material_chance < 0.6 {
                arena.alloc(Diffuse::new(Vec3::new(
                    rand!() * rand!(),
                    rand!() * rand!(),
                    rand!() * rand!(),
                ))) as &dyn Material
            } else if material_chance < 0.8 {
                arena.alloc(Metal::new(
                    Vec3::new(
                        (rand!() + 1.0) * 0.5,
                        (rand!() + 1.0) * 0.5,
                        (rand!() + 1.0) * 0.5,
                    ),
                    rand!() * 0.5,
                )) as &dyn Material
            } else {
                arena.alloc(
                    Dielectric::new(rand!() * 2.0)
                ) as &dyn Material
            };
            let sphere = Sphere::new(center, 0.2, material);
            let moving = if rand!() < 0.5 {
                sphere.with_velocity(Vec3::new(0.0, 0.5 * rand!(), 0.0))
            } else {
                sphere
            };
            surfaces.push(arena.alloc(moving) as &dyn Surface);
        }
    }

    let gray = Diffuse::new(Vec3::new(0.5, 0.5, 0.5));
    let diffuse = Diffuse::new(Vec3::new(0.2, 0.5, 0.6));
    let glass = Dielectric::new(1.5);
    let metal = Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0);

    let floor = Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, &gray);
    let far = Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, &diffuse);
    let mid = Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, &glass);
    let close = Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, &metal);

    surfaces.push(&floor);
    surfaces.push(&far);
    surfaces.push(&mid);
    surfaces.push(&close);

    let bvh_arena = Arena::<bvh::Tree>::new(1000);

    {{
        let scene = bvh::Tree::new(&bvh_arena, surfaces.as_slice(), 6, 0.0, 1.0);
    }}

    handle.join().unwrap();
    Ok(())
}
