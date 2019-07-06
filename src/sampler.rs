/// Sobol sampler
#[derive(Clone, Debug)]
pub struct Sobol {
    samples: usize, 
}

impl Sobol {
    pub fn start(&mut self, x: usize, y: usize) {
        unimplemented!()
    }

    pub fn get_1d(&self) -> f32 {
        unimplemented!()
    }

    pub fn get_2d(&self) -> (f32, f32) {
        unimplemented!()
    }

    pub fn get_1d_array(&mut self, n: usize) {
        unimplemented!()
    }

    pub fn get_2d_array(&mut self, n: usize) {
        unimplemented!()
    }

}
