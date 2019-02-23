use photon::*;

fn uniform_sphere() -> Vec3 {
    let ones = Vec3::new(1.0, 1.0, 1.0);
    loop {
        let p = Vec3::new(
            rand::random(), 
            rand::random(), 
            rand::random(),
        ) * 2.0 - ones;
        if p.len_sq() < 1.0 { break p }
    }
}

fn color(ray: &Ray, scene: &Surface) -> Vec3 {
    let mut hit = Hit::default();
    if scene.hit(ray, 0.0, std::f32::MAX, &mut hit) {
        let bounce = hit.p + hit.n + uniform_sphere();
        color(&Ray::new(hit.p, bounce - hit.p), scene) * 0.5
    } else {
        let dir = ray.d().normalize();
        let t = 0.5 * (dir.y() + 1.0);
        let white = Vec3::new(1.0, 1.0, 1.0);
        let blue = Vec3::new(0.5, 0.7, 1.0);
        white.lerp(&blue, t)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let nx = 200;
    let ny = 100;
    let ns = 100;
    let mut ppm = photon::PPM::new(nx, ny);
    let mut outfile = std::fs::File::create("test.ppm")?;

    let camera = Camera::new(
        Vec3::new(-2.0, -1.0, -1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        Vec3::default(),
    );

    let small = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);
    let large = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0);
    let mut scene = List::default();
    scene.push(&small);
    scene.push(&large);

    for y in 0..ny {
        for x in 0..nx {
            let mut c = Vec3::default();
            for _ in 0..ns {
                let u = (x as f32 + rand::random::<f32>()) / nx as f32;
                let v = (y as f32 + rand::random::<f32>()) / ny as f32;
                let r = camera.get(u, v);
                c += color(&r, &scene);
            }
            c /= ns as f32;
            ppm.set(x, y, (c[0].sqrt(), c[1].sqrt(), c[2].sqrt()));
        }
    }

    ppm.write(&mut outfile)?; 
    Ok(())
}
