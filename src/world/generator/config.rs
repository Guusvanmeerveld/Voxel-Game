use rand::rngs::StdRng;

use crate::location::Location;

pub struct GeneratorConfig {
    seed: u64,
    /// The world size in chunks
    world_size: usize,
    chunk_size: (usize, usize, usize),
}

impl GeneratorConfig {
    pub fn new(seed: u64, world_size: usize, chunk_size: (usize, usize, usize)) -> Self {
        Self {
            seed,
            world_size,
            chunk_size,
        }
    }

    /// Get a random location within the specified world
    pub fn random_location(&self, rng: &mut StdRng) -> Location {
        let size = self.world_size;

        let x = ((size / 2) * self.chunk_size.0) as isize;
        let y = ((size / 2) * self.chunk_size.1) as isize;
        let z = ((size / 2) * self.chunk_size.2) as isize;

        Location::random(rng, -x..x, -y..y, -z..z)
    }

    pub fn world_size(&self) -> usize {
        self.world_size
    }

    pub fn chunk_size(&self) -> (usize, usize, usize) {
        self.chunk_size
    }

    pub fn seed(&self) -> u64 {
        self.seed
    }

    pub fn set_seed(&mut self, seed: u64) {
        self.seed = seed;
    }
}
