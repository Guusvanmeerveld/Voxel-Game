use std::{
    io::{Error, ErrorKind},
    thread,
};

use log::{info, trace};

use rand::{rngs::StdRng, Rng};

use crate::{
    error::Result,
    location::Location,
    world::{
        chunk::Chunk,
        generator::perlin::PerlinNoiseGen,
        voxel::{Voxel, VoxelType},
    },
};

use super::GeneratorConfig;

pub struct ChunkGenerator;

impl ChunkGenerator {
    pub fn generate_chunk(location: (isize, isize), seed: u32, config: &GeneratorConfig) -> Chunk {
        let perlin = PerlinNoiseGen::new(seed, 256);

        let mut chunk = Chunk::new(location, config.chunk_size());

        let (size_x, _, size_z) = chunk.size();

        info!("Creating chunk at ({}, {})", location.0, location.1);

        for x in 0..size_x {
            for z in 0..size_z {
                let world_location = Location::from_chunk(&chunk, (x, 0, z));

                let surface = perlin.get((world_location.x(), world_location.z()));

                trace!("Surface level is {}", surface);

                chunk.set(&surface, Voxel::new(VoxelType::Ground));
            }
        }

        chunk
    }

    pub fn generate_chunks(rng: &mut StdRng, config: &GeneratorConfig) -> Result<Vec<Chunk>> {
        let mut chunks = Vec::new();

        let world_size = config.world_size();

        let edge = (world_size as isize) / 2;

        let seed: u32 = rng.gen();

        thread::scope(|scope| -> Result<()> {
            let mut handles = Vec::new();

            for x in -edge..edge {
                for z in -edge..edge {
                    let handle = scope.spawn(move || Self::generate_chunk((z, x), seed, config));

                    handles.push(handle);
                }
            }

            for handle in handles {
                let chunk = handle.join().map_err(|_| {
                    Error::new(ErrorKind::Interrupted, "A thread generating a chunk failed")
                })?;

                chunks.push(chunk);
            }

            Ok(())
        })?;

        Ok(chunks)
    }
}
