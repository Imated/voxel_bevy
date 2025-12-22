use bevy::prelude::{Component, IVec3};
use std::num::NonZeroU8;
use crate::block::Block;

pub const CHUNK_SIZE: i32 = 16;
pub const CHUNK_HEIGHT: i32 = 16;

#[derive(Component, Default, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Chunk {
    blocks: Vec<Block>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            blocks: vec![Block(0); (CHUNK_SIZE * CHUNK_SIZE * CHUNK_HEIGHT) as usize],
        }
    }

    pub fn coords_by_index(mut index: i32) -> IVec3 {
        let z = index / (CHUNK_SIZE * CHUNK_HEIGHT);
        index -= z * (CHUNK_SIZE * CHUNK_HEIGHT);

        let y = index / CHUNK_SIZE;
        index -= y * CHUNK_SIZE;

        let x = index;

        IVec3 { x, y, z }
    }

    pub fn get_by_index(&self, index: i32) -> Block {
        if index >= self.blocks.len() as i32 {
            return Block(0);
        }
        self.blocks[index as usize]
    }

    pub fn set_by_index(&mut self, index: i32, id: Block) {
        if index >= self.blocks.len() as i32 {
            return;
        }
        self.blocks[index as usize] = id;
    }

    pub fn get_by_xyz(&self, x: i32, y: i32, z: i32) -> Block {
        if x < 0 || x >= CHUNK_SIZE || y < 0 || y >= CHUNK_HEIGHT || z < 0 || z >= CHUNK_SIZE {
            return Block(0);
        }
        self.blocks[(x + (y * CHUNK_SIZE) + (z * CHUNK_SIZE * CHUNK_HEIGHT)) as usize]
    }

    pub fn set_by_xyz(&mut self, x: i32, y: i32, z: i32, id: Block) {
        if x < 0 || x >= CHUNK_SIZE || y < 0 || y >= CHUNK_HEIGHT || z < 0 || z >= CHUNK_SIZE {
            return;
        }
        self.blocks[(x + (y * CHUNK_SIZE) + (z * CHUNK_SIZE * CHUNK_HEIGHT)) as usize] = id;
    }

    pub fn get(&self, coords: IVec3) -> Block {
        if coords.x < 0
            || coords.x >= CHUNK_SIZE
            || coords.y < 0
            || coords.y >= CHUNK_HEIGHT
            || coords.z < 0
            || coords.z >= CHUNK_SIZE
        {
            return Block(0);
        }
        self.blocks
            [(coords.x + (coords.y * CHUNK_SIZE) + (coords.z * CHUNK_SIZE * CHUNK_HEIGHT)) as usize]
    }

    pub fn get_2(&self, coords: IVec3, offset: IVec3) -> (Block, Block) {
        let first = self.get(coords);
        let second = self.get(coords + offset);
        (first, second)
    }

    pub fn set(&mut self, coords: IVec3, id: Block) {
        if coords.x < 0
            || coords.x >= CHUNK_SIZE
            || coords.y < 0
            || coords.y >= CHUNK_HEIGHT
            || coords.z < 0
            || coords.z >= CHUNK_SIZE
        {
            return;
        }
        self.blocks[(coords.x + (coords.y * CHUNK_SIZE) + (coords.z * CHUNK_SIZE * CHUNK_HEIGHT))
            as usize] = id;
    }
}
