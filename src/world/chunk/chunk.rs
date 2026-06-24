use crate::world::block::Block;
use crate::world::chunk::chunk_section::ChunkSection;
use bevy::math::{IVec2, IVec3};
use bevy::prelude::Component;
use std::ops::Add;
use std::sync::atomic::AtomicU8;
use std::sync::{Arc, RwLock};

pub const CHUNK_SIZE: f32 = 16.0;
pub const CHUNK_SIZEI: i32 = 16;

#[derive(Component, Copy, Clone, Eq, PartialEq, Debug, Default, Hash)]
pub struct ChunkPos(pub IVec2);

impl Add<IVec2> for ChunkPos {
    type Output = ChunkPos;

    fn add(self, rhs: IVec2) -> Self::Output {
        ChunkPos(self.0 + rhs)
    }
}

#[derive(Debug, Default)]
pub struct Chunk {
    pub sections: Vec<Arc<RwLock<ChunkSection>>>,
    pub(crate) neighbors: AtomicU8,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            sections: vec![],
            neighbors: AtomicU8::new(0),
        }
    }

    pub fn generate(&mut self) {
        for _ in 0..2 {
            let mut section = ChunkSection::new();

            for x in 0..CHUNK_SIZEI {
                for y in 0..CHUNK_SIZEI {
                    for z in 0..CHUNK_SIZEI {
                        let dx = x - 8;
                        let dy = y - 8;
                        let dz = z - 8;

                        let voxel = if dx * dx + dy * dy + dz * dz < 9 * 9 {
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

    pub fn get_by_xyz(&self, x: i32, y: i32, z: i32) -> Option<Block> {
        let section = y / CHUNK_SIZEI;
        if section >= self.sections.len() as i32 {
            return None;
        }
        let y_in_section = y % CHUNK_SIZEI;
        let guard = self.sections[section as usize].read().unwrap();
        guard.get_by_xyz(x, y_in_section, z)
    }

    pub fn set_by_xyz(&self, x: i32, y: i32, z: i32, id: Block) {
        let section = y / CHUNK_SIZEI;
        if section >= self.sections.len() as i32 {
            return;
        }
        let y_in_section = y % CHUNK_SIZEI;
        let mut guard = self.sections[section as usize].write().unwrap();
        guard.set_by_xyz(x, y_in_section, z, id);
    }

    pub fn get(&self, coords: IVec3) -> Option<Block> {
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
