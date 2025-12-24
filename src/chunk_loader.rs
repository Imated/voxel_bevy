use std::collections::HashSet;
use bevy::app::{App, Plugin, PreUpdate};
use bevy::math::{IVec3, Vec3};
use bevy::prelude::{Component, GlobalTransform, Query, ResMut};
use crate::chunk::CHUNK_SIZE;
use crate::world::World;

#[derive(Component, Default)]
pub struct ChunkLoader {
    pub distance: i32,
    previous_chunk: IVec3,
    data_sampling_offsets: Vec<IVec3>,
}

impl ChunkLoader {
    pub fn new(distance: i32) -> Self {
        let data_sampling_offsets = make_offset_vec(distance);
        
        Self {
            distance,
            previous_chunk: IVec3::splat(9999),
            data_sampling_offsets,
        }
    }
}

pub struct ChunkLoaderPlugin;

impl Plugin for ChunkLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, Self::update_chunks);
    }
}

impl ChunkLoaderPlugin {
    pub fn update_chunks(mut loaders: Query<(&mut ChunkLoader, &GlobalTransform)>, mut world: ResMut<World>) {
        for (mut loader, transform) in loaders {
            let current_chunk = (transform.translation() - Vec3::splat((CHUNK_SIZE / 2) as f32)).as_ivec3();
            let previous_chunk = loader.previous_chunk;
            let has_moved = current_chunk != previous_chunk;
            loader.previous_chunk = current_chunk;
            if !has_moved {
                continue;
            }

            let load_area = loader
                .data_sampling_offsets
                .iter()
                .map(|&offset| current_chunk + offset)
                .collect::<HashSet<IVec3>>();

            let unload_area = loader
                .data_sampling_offsets
                .iter()
                .map(|&offset| previous_chunk + offset)
                .collect::<HashSet<IVec3>>();

            for &pos in load_area.difference(&unload_area) {
                world.load_chunk(pos, false);
            }

            for &pos in unload_area.difference(&load_area) {
                world.unload_chunk(pos);
            }
        }
    }
}

//https://github.com/TanTanDev/binary_greedy_mesher_demo/blob/main/src/scanner.rs#L182
fn make_offset_vec(half: i32) -> Vec<IVec3> {
    let k = (half * 2) + 1;
    let mut sampling_offsets = vec![];
    for i in 0..k * k * k {
        let x = i % k;
        let y = (i / k) % k;
        let z = i / (k * k);
        let mut pos =IVec3::new(x, y, z);
        pos -= IVec3::splat((k as f32 * 0.5) as i32);

        sampling_offsets.push(pos);
    }
    sampling_offsets.sort_by(|a, b| {
        a.distance_squared(IVec3::ZERO)
            .cmp(&b.distance_squared(IVec3::ZERO))
    });
    sampling_offsets
}