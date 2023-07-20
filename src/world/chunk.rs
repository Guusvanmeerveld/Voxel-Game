use ndarray::Array3;

use noise::{NoiseFn, Perlin};

use crate::{
    location::{self, Location},
    world::voxel::VoxelType,
};

use super::voxel::Voxel;

fn scale_coord(coord: usize) -> f64 {
    (coord as f64) / (5 as f64)
}

pub struct Chunk {
    voxels: Array3<Voxel>,
}

impl Default for Chunk {
    fn default() -> Self {
        Self {
            voxels: Array3::<Voxel>::default((16, 16, 256)),
        }
    }
}

impl Chunk {
    pub fn set(&mut self, loc: &Location, voxel: Voxel) {
        self.voxels[[loc.x(), loc.z(), loc.y()]] = voxel;
    }

    pub fn get(&mut self, loc: &Location) -> &Voxel {
        &self.voxels[[loc.x(), loc.z(), loc.y()]]
    }

    pub fn empty() -> Self {
        Self::default()
    }

    pub fn generate(seed: u32) -> Self {
        let perlin = Perlin::new(seed);

        let mut chunk = Self::default();

        let dim = chunk.voxels.dim();

        for x in 0..dim.0 {
            for z in 0..dim.1 {
                let value = perlin.get([scale_coord(x), scale_coord(z)]);

                let normalized_value = ((value + 1.0) / 2.0) * 256.0;

                println!("Height level at ({}, {}) is {}", x, z, normalized_value);

                let location = Location::new(x, (normalized_value as usize).saturating_sub(1), z);

                chunk.set(&location, Voxel::new(VoxelType::Ground));
            }
        }

        chunk
    }
}
