use std::collections::HashMap;

use rayon::prelude::*;

use photon::prelude::*;
use photon::arena;
use photon::bxdf;
use photon::bvh;
use photon::geom;
use photon::integrator;
use photon::math;
use photon::math::Vec3;
use photon::camera::Camera;
use photon::light;
use photon::scene;

fn render<'scene, I: Integrator<'scene>>(
    nx: usize, 
    ny: usize,
    ns: usize,
    camera: &Camera,
    scene: &scene::Scene<'scene>,
    mut integrator: I,
) {
    let progress = std::thread::spawn(move || photon::progress::run(nx * ny));

    let mut buffer = vec![0; nx * ny * 3];

    buffer.par_chunks_mut(nx * 3)
        .enumerate()
        .map(|(y, row)| (ny - y - 1, row))
        .for_each(|(y, row)| {
            let mut hit = geom::Record::default();
            for x in 0..nx {
                let mut c = Vec3::default();
                for _ in 0..ns {
                    let u = (x as f32 + rand::random::<f32>()) / nx as f32;
                    let v = (y as f32 + rand::random::<f32>()) / ny as f32;
                    let mut r = camera.get(u, v);
                    // FIXME: move logic inside Scene?
                    if scene.hit(&mut r, &mut hit) {
                        c += integrator.shade(scene, &r, &hit, 0);
                    } else {
                        c += scene.background();
                    }
                }
                c /= ns as f32;
                row[x * 3 + 0] = (math::clamp(c[0].sqrt(), 0.0, 1.0) * 255.99) as u8;
                row[x * 3 + 1] = (math::clamp(c[1].sqrt(), 0.0, 1.0) * 255.99) as u8;
                row[x * 3 + 2] = (math::clamp(c[2].sqrt(), 0.0, 1.0) * 255.99) as u8;
                photon::stats::PIXELS_RENDERED.inc();
            }
        });
    
    lodepng::encode24_file("out.png", &buffer, nx, ny).unwrap();

    progress.join().unwrap().ok();

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
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let nx = 1920; // Width
    // let ny = 1080; // Height
    // let ns = 1024;  // Samples per pixel

    // // Camera setup
    // let origin = Vec3::new(15.0, 15.0, -15.0);
    // let toward = Vec3::new(0.0, 0.0, 0.0);
    // let up = Vec3::new(0.0, 1.0, 0.0);
    // let fov = 45.0;
    // let aspect = nx as f32 / ny as f32;
    // let focus = 0.035;
    // let aperture = 0.0001;
    // let camera = Camera::new(origin, toward, up, fov, aspect, aperture, focus);

    // let background = Vec3::new(0.0, 0.0, 0.0);

    // let white = bxdf::Lambertian::new(
    //     Vec3::new(1.0, 1.0, 1.0)
    // );

    // let red = bxdf::Lambertian::new(
    //     Vec3::new(1.0, 0.0, 0.0)
    // );

    // let green = bxdf::Lambertian::new(
    //     Vec3::new(0.0, 1.0, 0.0)
    // );

    // let blue = bxdf::Lambertian::new(
    //     Vec3::new(0.0, 0.0, 1.0)
    // );

    // let floor = geom::Quad::new(
    //     Vec3::new(-7.5, 0.0, 7.5),
    //     Vec3::new(0.0, 0.0, -15.0),
    //     Vec3::new(15.0, 0.0, 0.0),
    //     &white,
    //     None,
    // );

    // let light = geom::Quad::new(
    //     Vec3::new(-2.5, 0.0, 2.5),
    //     Vec3::new(0.0, 0.0, -5.0),
    //     Vec3::new(0.0, 5.0, 0.0),
    //     &white,
    //     Some(Vec3::new(1.0, 1.0, 1.0)),
    // );

    // let lights = [&light as &dyn light::Light];

    // let bvh = bvh::Linear::new(
    //     &[&floor, &light],
    // );

    // let scene = scene::Scene::new(
    //     background,
    //     camera,
    //     &lights[..],
    //     &bvh,
    // );

    // render::<integrator::Light>(nx, ny, ns, &camera, &scene);

    Ok(())
}
