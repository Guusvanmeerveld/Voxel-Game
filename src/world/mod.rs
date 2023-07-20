use rand::{rngs::StdRng, Rng, SeedableRng};

use self::chunk::Chunk;

const CHUNK_COUNT: usize = 1;

mod chunk;
mod player;
mod voxel;

pub struct World {
    chunks: Vec<Chunk>,
}

impl World {
    pub fn new(seed: u64) -> Self {
        let mut rng: StdRng = SeedableRng::seed_from_u64(seed);

        let mut chunks = Vec::new();

        for _ in 0..CHUNK_COUNT {
            let seed: u32 = rng.gen();

            let chunk = Chunk::generate(seed);

            chunks.push(chunk)
        }

        Self { chunks }
    }
}
