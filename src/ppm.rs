use std::io::Write;

#[derive(Clone, Debug)]
pub struct PPM {
    nx: usize,
    ny: usize,
    buffer: Vec<u8>,
}

impl PPM {
    pub fn new(nx: usize, ny: usize) -> Self {
        PPM { nx, ny, buffer: vec![0; nx * ny * 3] }
    }

    fn index(&self, x: usize, y: usize) -> usize {
        3 * (self.nx * y + x)
    }

    pub fn set(&mut self, x: usize, y: usize, rgb: (f32, f32, f32)) {
        let i = self.index(x, self.ny - y - 1);
        self.buffer[i + 0] = (rgb.0 * 255.99) as u8;
        self.buffer[i + 1] = (rgb.1 * 255.99) as u8;
        self.buffer[i + 2] = (rgb.2 * 255.99) as u8;
    }

    pub fn write<W: Write>(&self, mut out: W) -> Result<(), std::io::Error> {
        write!(out, "P3\n{} {}\n{}\n", self.nx, self.ny, 255)?;
        for y in 0..self.ny {
            for x in 0..self.nx {
                let i = self.index(x, y);
                write!(
                    out,
                    "{} {} {}\n",
                    self.buffer[i + 0],
                    self.buffer[i + 1],
                    self.buffer[i + 2],
                )?;
            }
        }
        Ok(())
    }
}
