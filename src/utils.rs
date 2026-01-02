use std::hash::{Hash, Hasher};
use bevy::math::UVec3;
use std::ops::{Range, Sub};

pub fn pack_vertex(position: UVec3, normal: u32, block_type: u32, section_y: u32) -> u32 {
    position.x
        | position.y << 5u32
        | position.z << 10u32
        | normal << 15u32
        | block_type << 18u32
        | section_y << 28u32
}

pub fn hash_u64(seed: u64, x: i32, z: i32) -> u64 {
    let mut hasher = fxhash::FxHasher::default();
    (seed, x, z).hash(&mut hasher);
    hasher.finish()
}

pub trait RangeExtensions<T> {
    fn mid(&self) -> T;
}

impl<T> RangeExtensions<T> for Range<T>
where
    T: Sub<Output = T> + Copy,
{
    fn mid(&self) -> T {
        self.end - self.start
    }
}
