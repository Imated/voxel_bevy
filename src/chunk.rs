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

#[derive(Component, Default, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Chunk {
    blocks: Vec<Block>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            blocks: vec![Block(0); (CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE) as usize],
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

    pub fn get(&self, coords: IVec3) -> Block {
        if coords.x < 0
            || coords.x >= CHUNK_SIZE
            || coords.y < 0
            || coords.y >= CHUNK_SIZE
            || coords.z < 0
            || coords.z >= CHUNK_SIZE
        {
            return Block(0);
        }
        self.blocks[(coords.x + (coords.y * CHUNK_SIZE) + (coords.z * CHUNK_SIZE2)) as usize]
    }

    pub unsafe fn get_unchecked(&self, coords: IVec3) -> Block {
        self.blocks[(coords.x + (coords.y * CHUNK_SIZE) + (coords.z * CHUNK_SIZE2)) as usize]
    }

    pub unsafe fn get_by_xyz_unchecked(&self, x: i32, y: i32, z: i32) -> Block {
        self.blocks[(x + (y * CHUNK_SIZE) + (z * CHUNK_SIZE2)) as usize]
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
            || coords.y >= CHUNK_SIZE
            || coords.z < 0
            || coords.z >= CHUNK_SIZE
        {
            return;
        }
        self.blocks[(coords.x + (coords.y * CHUNK_SIZE) + (coords.z * CHUNK_SIZE2)) as usize] = id;
    }
}
