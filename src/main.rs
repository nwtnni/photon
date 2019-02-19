use photon::*;

fn color(ray: &Ray) -> Vec3 {
    let dir = ray.d().normalize();
    let t = 0.5 * (dir.y() + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let nx = 200;
    let ny = 100;
    let mut ppm = photon::PPM::new(nx, ny);
    let mut outfile = std::fs::File::create("test.ppm")?;

    let ll = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::default();

    for y in 0..ny {
        for x in 0..nx {
            let u = x as f32 / nx as f32;
            let v = y as f32 / ny as f32;
            let r = Ray::new(origin, ll + horizontal * u + vertical * v);
            let c = color(&r);
            ppm.set(x, y, (c[0], c[1], c[2]));
        }
    }

    ppm.write(&mut outfile)?; 
    Ok(())
}
