use std::io::Write;

#[derive(Debug)]
pub struct PPM(std::io::BufWriter<std::fs::File>);

impl PPM {
    pub fn new<P: Into<std::path::PathBuf>>(nx: usize, ny: usize, path: P) -> Result<Self, std::io::Error> {
        let file = std::fs::File::create(path.into())?;
        let mut writer = std::io::BufWriter::new(file);
        write!(writer, "P3\n{} {}\n{}\n", nx, ny, 255)?;
        Ok(PPM(writer))
    }

    pub fn write(&mut self, r: u8, g: u8, b: u8) -> Result<(), std::io::Error> {
        write!(self.0, "{} {} {} ", r, g, b)
    }
}
