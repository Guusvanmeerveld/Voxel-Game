pub struct Voxel {
    r#type: VoxelType,
}

impl Default for Voxel {
    fn default() -> Self {
        Self::new(VoxelType::Air)
    }
}

impl Voxel {
    pub fn new(r#type: VoxelType) -> Self {
        Self { r#type }
    }
}

pub enum VoxelType {
    Air,
    Ground,
}
