use std::{fmt::Display, ops::Range};

use log::trace;
use rand::{rngs::StdRng, Rng};

use crate::world::Chunk;

pub struct Location {
    x: isize,
    y: isize,
    z: isize,
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Location {
    pub fn random(rng: &mut StdRng, x: Range<isize>, y: Range<isize>, z: Range<isize>) -> Self {
        let x = rng.gen_range(x);
        let y = rng.gen_range(y);
        let z = rng.gen_range(z);

        Location::new(x, y, z)
    }

    pub fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }

    pub fn x(&self) -> isize {
        self.x
    }

    pub fn y(&self) -> isize {
        self.y
    }

    pub fn z(&self) -> isize {
        self.z
    }

    pub fn is_inside(&self, chunk: &Chunk) -> bool {
        let (size_x, size_y, size_z) = chunk.size();

        // Get the x and z values of the chunk from the location to make sure it is this chunk.
        let chunk_x = ((self.x() as f64) / (size_x as f64)).floor() as isize;
        let chunk_z = ((self.z() as f64) / (size_z as f64)).floor() as isize;

        let (x, z) = chunk.location();

        &chunk_z == z && &chunk_x == x && self.y() <= size_y as isize
    }

    pub fn from_chunk(chunk: &Chunk, relative: (usize, usize, usize)) -> Location {
        let (chunk_x, chunk_z) = chunk.location();
        let (size_x, _, size_z) = chunk.size();

        let x_in_chunk = if chunk_x.is_negative() {
            size_x.saturating_sub(relative.0) as isize
        } else {
            relative.0 as isize
        };

        let x = chunk_x * (size_x as isize) + x_in_chunk;

        let z_in_chunk = if chunk_z.is_negative() {
            size_z.saturating_sub(relative.2) as isize
        } else {
            relative.2 as isize
        };

        let z = chunk_z * (size_z as isize) + z_in_chunk;

        let location = Location::new(x, relative.1 as isize, z);

        trace!(
            "Relative location ({}, {}, {}) in chunk ({}, {}) is {}",
            relative.0,
            relative.1,
            relative.2,
            chunk_x,
            chunk_z,
            location
        );

        location
    }

    /// The position of this location relative to the given chunk
    pub fn relative_inside(&self, chunk: &Chunk) -> (usize, usize, usize) {
        let (size_x, _, size_z) = chunk.size();
        let (abs_x, abs_y, abs_z) = self.abs();

        let (x, z) = (abs_x % size_x, abs_z % size_z);

        (x, abs_y, z)
    }

    pub fn abs(&self) -> (usize, usize, usize) {
        (
            self.x.abs() as usize,
            self.y.abs() as usize,
            self.z.abs() as usize,
        )
    }
}
