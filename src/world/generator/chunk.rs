use std::{
    io::{Error, ErrorKind},
    thread,
};

use log::info;
use noise::{NoiseFn, Perlin};

use rand::{rngs::StdRng, Rng};

use crate::{
    error::Result,
    location::Location,
    world::{
        chunk::Chunk,
        voxel::{Voxel, VoxelType},
    },
};

use super::GeneratorConfig;

pub struct ChunkGenerator;

impl ChunkGenerator {
    fn scale_coord(coord: isize) -> f64 {
        (coord as f64) / (5 as f64)
    }

    pub fn generate_chunk(location: (isize, isize), seed: u32, config: &GeneratorConfig) -> Chunk {
        let perlin = Perlin::new(seed);

        let mut chunk = Chunk::new(location, config.chunk_size());

        let (size_x, _, size_z) = chunk.size();

        info!("Creating chunk at ({}, {})", location.0, location.1);

        for x in 0..size_x {
            for z in 0..size_z {
                let world_location = Location::from_chunk(&chunk, (x, 0, z));

                let value = perlin.get([
                    Self::scale_coord(world_location.x()),
                    Self::scale_coord(world_location.y()),
                ]);

                let normalized_value = ((value + 1.0) / 2.0) * 256.0;

                let location = Location::new(
                    x as isize,
                    (normalized_value as isize).saturating_sub(1),
                    z as isize,
                );

                chunk.set(&location, Voxel::new(VoxelType::Ground));
            }
        }

        chunk
    }

    pub fn generate_chunks(rng: &mut StdRng, config: &GeneratorConfig) -> Result<Vec<Chunk>> {
        let mut chunks = Vec::new();

        let world_size = config.world_size();

        let upper_bound = (world_size as isize) / 2;
        let lower_bound = -((world_size as isize) / 2);

        let seed: u32 = rng.gen();

        thread::scope(|scope| -> Result<()> {
            let mut handles = Vec::new();

            for x in lower_bound..upper_bound {
                for z in lower_bound..upper_bound {
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
