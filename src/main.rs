use std::collections::HashMap;

use rayon::prelude::*;

use photon::arena::{CopyArena};
use photon::bvh;
use photon::geometry::{Ray, Vec3};
use photon::material::{Material, Diffuse, Metal, Dielectric};
use photon::model::obj;
use photon::surface::{Surface, Sphere, Hit};
use photon::camera::Camera;
use photon::preview::Preview;

/// Main ray tracing function.
/// Intersects `ray` with `scene`, potentially recursing upon reflecting or refracting.
fn color(ray: &mut Ray, scene: &Surface, depth: i32) -> Vec3 {
    let mut hit = Hit::default();
    if scene.hit(ray, &mut hit) {
        let mut attenuation = Vec3::default();
        let mut scattered = Ray::default();
        if depth < 50 && hit.m.unwrap().scatter(ray, &hit, &mut attenuation, &mut scattered) {
            color(&mut scattered, scene, depth + 1) * attenuation
        } else {
            Vec3::default()
        }
    } else {
        let dir = ray.d.normalize();
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
                let mut r = camera.get(u, v);
                c += color(&mut r, scene, 0) / ns as f32;
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
    let ns = 100;  // Samples per pixel

    let (tx, rx) = crossbeam::channel::unbounded();
    let preview = Preview::new(nx, ny, rx);
    let handle = std::thread::spawn(|| preview.run());
    let arena = CopyArena::new(96 * 1024 * 1024);

    // Camera setup
    let origin = Vec3::new(-15.0, 7.0, -30.0);
    let toward = Vec3::new(0.0, 3.0, 0.0);
    let up = Vec3::new(0.0, 1.0, 0.0);
    let fov = 25.0;
    let aspect = nx as f32 / ny as f32;
    let focus = 34.0;
    let aperture = 0.20;
    let open = 0.0;
    let shut = 1.0;
    let camera = Camera::new(origin, toward, up, fov, aspect, aperture, focus, open, shut);

    let metal = Metal::new(Vec3::new(0.83, 0.68, 0.21), 0.1);
    let floor = Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, &metal);

    let glass = Dielectric::new(1.5);
    let mut scene = obj::parse("models/dragon.obj", &arena, &glass);
    scene.push(&floor);

    let scene = bvh::Linear::new(&arena, &scene, 0.0, 1.0);

    render(nx, ny, ns, tx, &camera, &scene);

    if cfg!(feature = "stats") {
        println!("{}", photon::stats::ARENA_MEMORY);
        println!("{}", photon::stats::LEAF_NODES);
        println!("{}", photon::stats::TOTAL_NODES);
        println!("{}", photon::stats::INTERSECTION_TESTS);
        println!("{}", photon::stats::BOUNDING_BOX_INTERSECTION_TESTS);
        println!("{}", photon::stats::SPHERE_INTERSECTION_TESTS);
        println!("{}", photon::stats::LIST_INTERSECTION_TESTS);
    }

    handle.join().unwrap();
    Ok(())
}
