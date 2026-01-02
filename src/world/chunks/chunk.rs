use crate::block::Block;
use crate::constants::{CHUNK_SIZE, CHUNK_SIZE2};
use crate::world::chunks::chunk_section::ChunkSection;
use bevy::prelude::IVec3;
use std::sync::{Arc, RwLock};

#[derive(Default, Debug)]
pub struct Chunk {
    pub sections: Vec<Arc<RwLock<ChunkSection>>>,
}

impl Chunk {
    pub fn new() -> Self {
        Self { sections: vec![] }
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

                        let voxel = if dx * dx + dy * dy + dz * dz < 9.0 * 9.0 {
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

    pub fn get_by_xyz(&self, x: i32, y: i32, z: i32) -> Option<Block> {
        let section = y / CHUNK_SIZE;
        if section >= self.sections.len() as i32 {
            return None;
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
