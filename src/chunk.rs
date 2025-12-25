use std::sync::{Arc, RwLock};
use bevy::math::IVec2;
use crate::block::Block;
use bevy::prelude::{Component, IVec3};

pub const CHUNK_SIZE: i32 = 16;
pub const CHUNK_SIZE2: i32 = 256;
pub const CHUNK_SIZE3: i32 = 4096;
pub const PADDED_CHUNK_SIZE: i32 = 18;
pub const PADDED_CHUNK_SIZE_USIZE: usize = 18;
pub const PADDED_CHUNK_SIZE2: i32 = 324;
pub const PADDED_CHUNK_SIZE2_USIZE: usize = 324;
pub const PADDED_CHUNK_SIZE3: i32 = 5832;
pub const PADDED_CHUNK_SIZE3_USIZE: usize = 5832;

#[derive(Component, Copy, Clone, Eq, PartialEq, Debug, Default, Hash)]
pub struct ChunkPos(pub IVec2);

#[derive(Default, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct ChunkSection {
    blocks: Vec<Block>,
}

impl ChunkSection {
    pub fn new() -> Self {
        Self {
            blocks: vec![Block(0); (CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE) as usize],
        }
    }

    pub fn get_by_xyz(&self, x: i32, y: i32, z: i32) -> Block {
        if x < 0 || x >= CHUNK_SIZE || y < 0 || y >= CHUNK_SIZE || z < 0 || z >= CHUNK_SIZE {
            return Block(0);
        }

        self.blocks[(x + (y * CHUNK_SIZE) + (z * CHUNK_SIZE2)) as usize]
    }

    pub fn set_by_xyz(&mut self, x: i32, y: i32, z: i32, id: Block) {
        if x < 0 || x >= CHUNK_SIZE || y < 0 || y >= CHUNK_SIZE || z < 0 || z >= CHUNK_SIZE {
            return;
        }

        self.blocks[(x + (y * CHUNK_SIZE) + (z * CHUNK_SIZE2)) as usize] = id;
    }
}

#[derive(Default, Debug)]
pub struct Chunk {
    pub sections: Vec<Arc<RwLock<ChunkSection>>>
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            sections: vec![],
        }
    }

    pub fn generate(&mut self) {
        for _ in 0..2 {
            let mut section = ChunkSection::new();

            for x in 0..CHUNK_SIZE {
                for y in 0..CHUNK_SIZE {
                    for z in 0..CHUNK_SIZE {
                        let dx = x as f32 - 8.0;
                        let dy = y as f32 - 8.0;
                        let dz = z as f32 - 8.0;

                        let voxel = if dx * dx + dy * dy + dz * dz < 64.0 {
                            Block(1)
                        } else {
                            Block(0)
                        };

                        section.set_by_xyz(x, y, z, voxel);
                    }
                }
            }

            self.sections.push(Arc::new(RwLock::new(section)));
        }
    }

    pub fn coords_by_index(mut index: i32) -> IVec3 {
        let z = index / CHUNK_SIZE2;
        index -= z * CHUNK_SIZE2;

        let y = index / CHUNK_SIZE;
        index -= y * CHUNK_SIZE;

        let x = index;

        IVec3 { x, y, z }
    }

    pub fn get_by_xyz(&self, x: i32, y: i32, z: i32) -> Block {
        let section = y / CHUNK_SIZE;
        if section >= self.sections.len() as i32 {
            return Block(0);
        }
        let y_in_section = y % CHUNK_SIZE;
        let guard = self.sections[section as usize].read().unwrap();
        guard.get_by_xyz(x, y_in_section, z)
    }

    pub fn set_by_xyz(&self, x: i32, y: i32, z: i32, id: Block) {
        let section = y / CHUNK_SIZE;
        if section >= self.sections.len() as i32 {
            return;
        }
        let y_in_section = y % CHUNK_SIZE;
        let mut guard = self.sections[section as usize].write().unwrap();
        guard.set_by_xyz(x, y_in_section, z, id);
    }

    pub fn get(&self, coords: IVec3) -> Block {
        let x = coords.x;
        let y = coords.y;
        let z = coords.z;
        self.get_by_xyz(x, y, z)
    }

    pub fn set(&self, coords: IVec3, id: Block) {
        let x = coords.x;
        let y = coords.y;
        let z = coords.z;
        self.set_by_xyz(x, y, z, id);
    }
}
