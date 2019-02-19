fn main() -> Result<(), Box<dyn std::error::Error>> {
    let nx = 200;
    let ny = 100;
    let mut ppm = photon::PPM::new(nx, ny);
    let mut outfile = std::fs::File::create("test.ppm")?;

    for y in 0..ny {
        for x in 0..nx {
            let r = x as f32 / nx as f32;
            let g = y as f32 / ny as f32;
            let b = 0.2;
            ppm.set(x, y, (r, g, b));
        }
    }

    ppm.write(&mut outfile)?; 
    Ok(())
}
