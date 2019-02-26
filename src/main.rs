use photon::geometry::{Ray, Vec3};
use photon::material::{Diffuse, Metal, Dielectric};
use photon::surface::{Surface, Sphere, List, Hit};
use photon::camera::Camera;
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
    let nx = 1600;
    let ny = 800;
    let ns = 100;
    let mut ppm = PPM::new(nx, ny, "test.ppm")?;

    let camera = Camera::new(90.0, nx as f32 / ny as f32);

    let r = std::f32::consts::PI / 4.0;
    let dl = Diffuse::new(Vec3::new(0.0, 0.0, 1.0));
    let dr = Diffuse::new(Vec3::new(1.0, 0.0, 0.0));

    let left = Sphere::new(Vec3::new(-r, 0.0, -1.0), r, &dl);
    let right = Sphere::new(Vec3::new(r, 0.0, -1.0), r, &dr);

    let mut scene = List::default();
    scene.push(&left);
    scene.push(&right);

    for y in (0..ny).rev() {
        for x in 0..nx {
            let mut c = Vec3::default();
            for _ in 0..ns {
                let u = (x as f32 + rand::random::<f32>()) / nx as f32;
                let v = (y as f32 + rand::random::<f32>()) / ny as f32;
                let r = camera.get(u, v);
                c += color(&r, &scene, 0);
            }
            c /= ns as f32;
            ppm.write(c[0].sqrt(), c[1].sqrt(), c[2].sqrt())?;
        }
        if y % 10 == 0 { println!("{}", y) }
    }

    Ok(())
}
