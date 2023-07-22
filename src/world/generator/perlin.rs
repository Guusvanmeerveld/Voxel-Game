use noise::{NoiseFn, Perlin};

use crate::location::Location;

pub struct PerlinNoiseGen {
    gen: Perlin,
    scale: usize,
}

impl PerlinNoiseGen {
    pub fn new(seed: u32, scale: usize) -> Self {
        let gen = Perlin::new(seed);

        Self { gen, scale }
    }

    fn make_fraction(coord: isize) -> f64 {
        (coord as f64) * 0.9
    }

    // Get the y level at a given location
    pub fn get(&self, (x, z): (isize, isize)) -> Location {
        let value = self
            .gen
            .get([Self::make_fraction(x), Self::make_fraction(z)]);

        let normalized_value = ((value + 1.0) / 2.0) * (self.scale as f64);

        Location::new(x, normalized_value as isize, z)
    }
}
