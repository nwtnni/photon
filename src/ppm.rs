use std::io::Write;

#[derive(Debug)]
pub struct PPM(std::io::BufWriter<std::fs::File>);

impl PPM {
    pub fn new<P: Into<std::path::PathBuf>>(nx: usize, ny: usize, path: P) -> Result<Self, std::io::Error> {
        let file = std::fs::File::open(path.into())?;
        let mut writer = std::io::BufWriter::new(file);
        write!(writer, "P3\n{} {}\n{}\n", nx, ny, 255)?;
        Ok(PPM(writer))
    }

    pub fn write(&mut self, r: f32, g: f32, b: f32) -> Result<(), std::io::Error> {
        write!(
            self.0,
            "{} {} {} ",
            (r * 255.99) as u8,
            (g * 255.99) as u8,
            (b * 255.99) as u8,
        )
    }
}
