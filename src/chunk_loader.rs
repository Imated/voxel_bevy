use std::collections::HashSet;
use bevy::app::{App, Plugin, PreUpdate};
use bevy::log::info;
use bevy::math::{IVec2, IVec3, Vec3, Vec3Swizzles};
use bevy::prelude::{Component, GlobalTransform, Query, ResMut};
use crate::chunk::{ChunkPos, CHUNK_SIZE};
use crate::world::World;

#[derive(Component, Default)]
pub struct ChunkLoader {
    pub distance: i32,
    previous_chunk: IVec3,
}

impl ChunkLoader {
    pub fn new(distance: i32) -> Self {
        Self {
            distance,
            previous_chunk: IVec3::splat(9999),
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
            let current_chunk = (transform.translation() / Vec3::splat(CHUNK_SIZE as f32)).as_ivec3();
            //info!("{:?}", current_chunk);
            //info!("{:?}", transform.translation());
            let previous_chunk = loader.previous_chunk;
            let has_moved = current_chunk != previous_chunk;
            loader.previous_chunk = current_chunk;
            if !has_moved {
                continue;
            }

            let chunks_to_load: HashSet<ChunkPos> = get_chunks_in_radius(ChunkPos(current_chunk.xz()), loader.distance).iter().copied().collect();
            let chunks_to_unload: HashSet<ChunkPos> = get_chunks_in_radius(ChunkPos(previous_chunk.xz()), loader.distance).iter().copied().collect();

            for &pos in chunks_to_load.difference(&chunks_to_unload) {
                world.load_chunk(pos);
            }

            for &pos in chunks_to_unload.difference(&chunks_to_load) {
                world.unload_chunk(pos);
            }
        }
    }
}

fn get_chunks_in_radius(center: ChunkPos, radius: i32) -> Vec<ChunkPos> {
    let mut chunks = vec![];
    let radius_sq = radius * radius;

    for x in -radius..=radius {
        for z in -radius..=radius {
            let dist_sq = x * x + z * z;
            if dist_sq <= radius_sq {
                chunks.push(ChunkPos(center.0 + IVec2::new(x, z)));
            }
        }
    }

    chunks.sort_by_key(|pos| pos.0.distance_squared(center.0));
    chunks
}