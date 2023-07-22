use log::info;
use rand::{rngs::StdRng, SeedableRng};

mod chunk;
mod config;
mod perlin;
mod player;

use self::{chunk::ChunkGenerator, player::PlayerGenerator};
use crate::error::Result;

pub use config::GeneratorConfig;

use super::World;

pub struct WorldGenerator {
    config: GeneratorConfig,
    rng: StdRng,
}

impl WorldGenerator {
    pub fn new(config: GeneratorConfig) -> Self {
        let rng = Self::create_rng_from_seed(config.seed());

        Self { rng, config }
    }

    fn create_rng_from_seed(seed: u64) -> StdRng {
        SeedableRng::seed_from_u64(seed)
    }

    pub fn set_seed(&mut self, seed: u64) {
        self.config.set_seed(seed);
        self.rng = Self::create_rng_from_seed(seed)
    }

    pub fn generate(&mut self) -> Result<World> {
        info!(
            "Generating a new world using the seed: {}",
            self.config.seed()
        );

        let chunks = ChunkGenerator::generate_chunks(&mut self.rng, &self.config)?;
        let players = PlayerGenerator::generate_players(&mut self.rng, &self.config);

        let world = World::new(chunks, players);

        // Reset the rng so we could create the same world again if the function was called again.
        self.set_seed(self.config.seed());

        Ok(world)
    }
}
