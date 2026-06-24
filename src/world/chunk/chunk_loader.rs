use crate::world::chunk::chunk::ChunkPos;
use crate::world::world::World;
use bevy::prelude::*;
use std::collections::HashSet;

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
    pub fn update_chunks(
        loaders: Query<(&mut ChunkLoader, &GlobalTransform)>,
        mut world: ResMut<World>,
    ) {
        for (mut loader, transform) in loaders {
            let current_chunk = (transform.translation() / Vec3::splat(16 as f32)).as_ivec3();
            let previous_chunk = loader.previous_chunk;
            if current_chunk == previous_chunk {
                continue;
            }
            loader.previous_chunk = current_chunk;

            let chunks_to_load_set: HashSet<ChunkPos> =
                get_chunks_in_radius(ChunkPos(current_chunk.xz()), loader.distance)
                    .iter()
                    .copied()
                    .collect();
            let chunks_to_unload: HashSet<ChunkPos> =
                get_chunks_in_radius(ChunkPos(previous_chunk.xz()), loader.distance)
                    .iter()
                    .copied()
                    .collect();

            let mut chunks_to_load = chunks_to_load_set
                .difference(&chunks_to_unload)
                .collect::<Vec<_>>();
            chunks_to_load.sort_by_key(|pos| pos.0.distance_squared(current_chunk.xz()));
            for &pos in chunks_to_load {
                world.load_chunk(pos);
            }

            for &pos in chunks_to_unload.difference(&chunks_to_load_set) {
                world.unload_chunk(pos);
            }
        }
    }
}

fn get_chunks_in_radius(center: ChunkPos, radius: i32) -> Vec<ChunkPos> {
    let mut chunks = Vec::new();
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
