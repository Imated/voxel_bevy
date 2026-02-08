use crate::block::Block;
use crate::constants::{CHUNK_SIZE, CHUNK_SIZE2};
use crate::world::chunks::chunk_pos::ChunkPos;
use crate::world::chunks::chunk_section::ChunkSection;
use crate::world::world_gen::biome::Biome;
use crate::world::world_gen::biome_generator::BiomeGenerator;
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

    pub fn generate(&mut self, biome_generator: Arc<BiomeGenerator>, chunk_pos: ChunkPos) {
        let mut heights = [[0; CHUNK_SIZE as usize]; CHUNK_SIZE as usize];

        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let world_x = chunk_pos.0.x * CHUNK_SIZE + x;
                let world_z = chunk_pos.0.y * CHUNK_SIZE + z;
                let biomes = biome_generator.get_biomes_at(world_x as f64, world_z as f64);

                let mut total_height = 0.0;
                for (weight, biome) in biomes {
                    let height = match biome {
                        Biome::Plains => 20.0,
                        Biome::Desert => 10.0,
                        Biome::Ice => 50.0,
                        Biome::Tundra => 35.0,
                        Biome::Tropical => 70.0,
                        _ => 0.0,
                    };
                    total_height += height * weight;
                }

                heights[x as usize][z as usize] = total_height.round() as i32;
            }
        }

        let max_height = heights.iter().flatten().copied().max().unwrap_or(0);
        let needed_sections = ((max_height / CHUNK_SIZE) + 1).max(1);

        for section_index in 0..needed_sections {
            let mut section = ChunkSection::new();
            let section_base_y = section_index * CHUNK_SIZE;

            for x in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let height = heights[x as usize][z as usize];

                    for y in 0..CHUNK_SIZE {
                        let world_y = section_base_y + y;
                        if world_y <= height {
                            section.set_by_xyz(x, y, z, Block(1));
                        }
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
