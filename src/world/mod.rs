use crate::location::Location;

use self::{chunk::Chunks, player::Players, voxel::Voxel};

pub use self::{
    chunk::Chunk,
    generator::{GeneratorConfig, WorldGenerator},
};

mod chunk;
mod generator;
mod player;
mod voxel;

pub struct World {
    chunks: Chunks,
    players: Players,
}

impl World {
    pub fn new<C: Into<Chunks>, P: Into<Players>>(chunks: C, players: P) -> Self {
        Self {
            chunks: chunks.into(),
            players: players.into(),
        }
    }

    pub fn players(&self) -> &Players {
        &self.players
    }

    // Get a voxel at a given location in the world
    pub fn get_location(&self, location: &Location) -> Option<&Voxel> {
        if let Some(chunk) = self.chunks.iter().find(|chunk| location.is_inside(chunk)) {
            chunk.get(location)
        } else {
            None
        }
    }
}
