use ndarray::Array3;

use crate::{constants::CHUNK_SIZE, location::Location};

use super::voxel::Voxel;

pub struct Chunks {
    chunks: Vec<Chunk>,
}

impl From<Vec<Chunk>> for Chunks {
    fn from(chunks: Vec<Chunk>) -> Self {
        Self::new(chunks)
    }
}

impl Chunks {
    pub fn new<C: IntoIterator<Item = Chunk>>(iter: C) -> Self {
        let chunks: Vec<_> = iter.into_iter().collect();

        Self { chunks }
    }

    pub fn get_location(&self, location: (isize, isize)) -> Option<&Chunk> {
        self.chunks
            .iter()
            .find(|chunk| chunk.location() == &location)
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Chunk> {
        self.chunks.iter()
    }
}

pub struct Chunk {
    voxels: Array3<Voxel>,
    location: (isize, isize),
}

impl Default for Chunk {
    fn default() -> Self {
        Self::new((0, 0), CHUNK_SIZE)
    }
}

impl Chunk {
    pub fn new(location: (isize, isize), size: (usize, usize, usize)) -> Self {
        Self {
            voxels: Array3::<Voxel>::default((size.0, size.2, size.1)),
            location,
        }
    }

    pub fn set(&mut self, loc: &Location, voxel: Voxel) -> Option<()> {
        if loc.is_inside(self) {
            let (x, y, z) = loc.relative_inside(self);

            self.voxels[[x, z, y]] = voxel;

            Some(())
        } else {
            None
        }
    }

    pub fn get(&self, loc: &Location) -> Option<&Voxel> {
        if loc.is_inside(self) {
            let (x, y, z) = loc.relative_inside(self);

            Some(&self.voxels[[x, z, y]])
        } else {
            None
        }
    }

    pub fn empty() -> Self {
        Self::default()
    }

    pub fn location(&self) -> &(isize, isize) {
        &self.location
    }

    pub fn size(&self) -> (usize, usize, usize) {
        let dim = self.voxels.dim();

        (dim.0, dim.2, dim.1)
    }
}
