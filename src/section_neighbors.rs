use crate::chunk::{Chunk, ChunkPos, ChunkSection};
use bevy::math::IVec2;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub struct SectionNeighbors {
    pub center: Arc<RwLock<ChunkSection>>,
    pub up: Option<Arc<RwLock<ChunkSection>>>,
    pub down: Option<Arc<RwLock<ChunkSection>>>,
    pub north: Option<Arc<RwLock<ChunkSection>>>,
    pub south: Option<Arc<RwLock<ChunkSection>>>,
    pub east: Option<Arc<RwLock<ChunkSection>>>,
    pub west: Option<Arc<RwLock<ChunkSection>>>,
}

impl SectionNeighbors {
    pub fn new(
        world_data: &HashMap<ChunkPos, Arc<Chunk>>,
        middle_chunk: ChunkPos,
        section_y: usize,
    ) -> Self {
        let center_chunk = world_data.get(&middle_chunk).unwrap();
        let center = center_chunk.sections.get(section_y).unwrap().clone();

        let up = center_chunk.sections.get(section_y + 1).cloned();
        let down = section_y.checked_sub(1).and_then(|y| center_chunk.sections.get(y).cloned());

        let north = world_data
            .get(&ChunkPos(middle_chunk.0 + IVec2::new(0, 1)))
            .and_then(|chunk| chunk.sections.get(section_y).cloned());
        let south = world_data
            .get(&ChunkPos(middle_chunk.0 + IVec2::new(0, -1)))
            .and_then(|chunk| chunk.sections.get(section_y).cloned());
        let east = world_data
            .get(&ChunkPos(middle_chunk.0 + IVec2::new(1, 0)))
            .and_then(|chunk| chunk.sections.get(section_y).cloned());
        let west = world_data
            .get(&ChunkPos(middle_chunk.0 + IVec2::new(-1, 0)))
            .and_then(|chunk| chunk.sections.get(section_y).cloned());

        Self {
            center,
            up,
            down,
            north,
            south,
            east,
            west,
        }
    }
}
