use std::collections::HashMap;

use rayon::prelude::*;

use photon::prelude::*;
use photon::arena::Arena;
use photon::bxdf;
use photon::geom;
use photon::integrator;
use photon::math::{Ray, Vec3};
use photon::model;
use photon::geom::{Sphere, Translate, Record};
use photon::camera::Camera;
use photon::light;
use photon::scene;

fn render<'scene, I: Integrator<'scene>>(
    nx: usize, 
    ny: usize,
    ns: usize,
    camera: &Camera,
    scene: &scene::Scene<'scene>,
) {
    #[cfg(feature = "preview")]
    let (tx, preview) = {
        let (tx, rx) = crossbeam::channel::unbounded();
        let preview = std::thread::spawn(move || photon::preview::Preview::new(nx, ny, rx).run());
        (tx, preview)
    };

    #[cfg(feature = "progress")]
    let progress = std::thread::spawn(move || photon::progress::run(nx * ny));

    // Row to pixel buffer map
    let mut buffers: HashMap<usize, Vec<(u8, u8, u8)>> = HashMap::with_capacity(ny);

    // Allow each thread exclusive access to a single row 
    buffers.par_extend((0..ny).into_par_iter().map(move |y| {
        let mut buffer = Vec::with_capacity(nx);
        let mut hit = Record::default();
        for x in 0..nx {
            let mut c = Vec3::default();
            for _ in 0..ns {
                let u = (x as f32 + rand::random::<f32>()) / nx as f32;
                let v = (y as f32 + rand::random::<f32>()) / ny as f32;
                let mut r = camera.get(u, v);
                // FIXME: move logic inside Scene?
                if scene.hit(&mut r, &mut hit) {
                    c += I::shade(scene, &r, &hit, 0);
                }
            }
            c /= ns as f32;
            let rgb = (
                (c[0].sqrt() * 255.99) as u8,
                (c[1].sqrt() * 255.99) as u8,
                (c[2].sqrt() * 255.99) as u8,
            );
            buffer.push(rgb);
            #[cfg(feature = "preview")] tx.send((x, y, rgb)).ok();
            #[cfg(feature = "progress")] photon::stats::PIXELS_RENDERED.inc();
        }
        (y, buffer)
    }));

    // Collect buffers for PNG encoding
    let buffer = (0..ny).rev()
        .flat_map(|y| buffers.remove(&y).unwrap().into_iter())
        .collect::<Vec<_>>();

    lodepng::encode24_file("out.png", &buffer, nx, ny).unwrap();

    #[cfg(feature = "progress")] {
        progress.join().unwrap().ok();
    }
    #[cfg(feature = "stats")] {
        println!("{}", photon::stats::ARENA_MEMORY);
        println!("{}", photon::stats::INTERSECTION_TESTS);
        println!("{}", photon::stats::BOUNDING_BOX_INTERSECTION_TESTS);
        println!("{}", photon::stats::BVH_HITS);
        println!("{}", photon::stats::BVH_MISSES);
        println!("{}", photon::stats::SPHERE_INTERSECTION_TESTS);
        println!("{}", photon::stats::TRI_INTERSECTION_TESTS);
        println!("{}", photon::stats::LIST_INTERSECTION_TESTS);
    }
    #[cfg(feature = "preview")] {
        preview.join().unwrap();
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let nx = 1920; // Width
    let ny = 1080; // Height
    let ns = 100;  // Samples per pixel

    let arena = Arena::new(96 * 1024 * 1024);

    // Camera setup
    // let origin = Vec3::new(4.0, 6.0, 8.0);
    // let toward = Vec3::new(-4.0, -6.0, -8.0);
    // let up = Vec3::new(0.0, 1.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 5.0);
    let toward = Vec3::new(0.0, 0.0, 0.0);
    let up = Vec3::new(0.0, 1.0, 0.0);
    let fov = 45.0;
    let aspect = nx as f32 / ny as f32;
    let focus = 4.5;
    let aperture = 0.0001;
    let camera = Camera::new(origin, toward, up, fov, aspect, aperture, focus);

    let light = &light::Point::new(
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(1.0, 1.0, 1.0),
    ) as &dyn light::Light;

    let bxdf = &bxdf::Lambertian::new(
        Vec3::new(0.0, 1.0, 0.0)
    ) as &dyn bxdf::BXDF;

    // let surface = &geom::Sphere::new(
    //     Vec3::new(0.0, 0.0, 0.0),
    //     0.25,
    //     bxdf,
    // ) as &dyn geom::Surface;
      
    let surface = &model::obj::parse(
        "models/buddha.obj",
        &arena,
        bxdf,
    ) as &dyn geom::Surface;

    let scene = scene::Scene::new(
        camera,
        vec![],
        surface,
    );

    render::<integrator::Normal>(nx, ny, ns, &camera, &scene);

    Ok(())
}
