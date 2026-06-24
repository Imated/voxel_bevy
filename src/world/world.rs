use crate::world::chunk::chunk::{CHUNK_SIZE, Chunk, ChunkPos};
use ahash::{HashMap, HashSet, RandomState};
use bevy::app::{App, Plugin, PreUpdate, Update};
use bevy::asset::Assets;
use bevy::color::palettes::basic::GREEN;
use bevy::input::ButtonInput;
use bevy::log::info;
use bevy::math::IVec2;
use bevy::mesh::{Mesh, Mesh3d, Meshable};
use bevy::pbr::{MeshMaterial3d, StandardMaterial};
use bevy::prelude::KeyCode::KeyT;
use bevy::prelude::{
    Commands, Entity, FromWorld, IntoScheduleConfigs, KeyCode, Plane3d, Res, ResMut,
    Resource, Transform, resource_changed,
};
use bevy::tasks::{AsyncComputeTaskPool, Task, block_on, poll_once};
use bevy::utils::default;
use indexmap::IndexSet;
use std::sync::Arc;
use std::time::Instant;

/// Main container for loaded and queued chunks.
/// Chunks are loaded async.
#[derive(Resource)]
pub struct World {
    loaded_chunks: HashMap<ChunkPos, Arc<Chunk>>,

    chunks_to_load: IndexSet<ChunkPos, RandomState>,
    chunks_to_mesh: IndexSet<ChunkPos, RandomState>,

    chunks_to_unload: HashSet<ChunkPos>,

    chunk_data_tasks: HashMap<ChunkPos, Task<Chunk>>,
    chunk_mesh_tasks: HashMap<ChunkPos, Task<Mesh>>,

    chunk_entities: HashMap<ChunkPos, Entity>,
}

impl FromWorld for World {
    fn from_world(_world: &mut bevy::prelude::World) -> Self {
        Self {
            loaded_chunks: Default::default(),
            chunks_to_load: Default::default(),
            chunks_to_mesh: Default::default(),
            chunks_to_unload: Default::default(),
            chunk_data_tasks: Default::default(),
            chunk_mesh_tasks: Default::default(),
            chunk_entities: Default::default(),
        }
    }
}

impl World {
    /// Queue a chunk to be loaded or generated async.
    pub fn load_chunk(&mut self, chunk_pos: ChunkPos) {
        if !self.chunk_exists(&chunk_pos) {
            self.chunks_to_load.insert(chunk_pos);
        }
    }

    /// Queue a chunk to be unloaded async.
    pub fn unload_chunk(&mut self, chunk_pos: ChunkPos) {
        self.chunks_to_unload.insert(chunk_pos);
    }

    /// Checks if a chunk is ready.
    ///
    /// A ready chunk is a chunk where all of its neighbors (excluding diagonal) and itself have completed generating
    /// TODO: make it return true if its in loaded data but not in chunk entities so its ready to mesh
    fn is_chunk_ready(loaded_chunks: &HashMap<ChunkPos, Arc<Chunk>>, chunk_pos: ChunkPos) -> bool {
        if !loaded_chunks.contains_key(&chunk_pos) {
            return false;
        }

        let mut completed_neighboring_chunks = 0;
        for neighboring_chunk_pos in NEIGHBORING_CHUNKS {
            if loaded_chunks.contains_key(&(chunk_pos + neighboring_chunk_pos)) {
                completed_neighboring_chunks += 1;
            }
        }

        completed_neighboring_chunks == NEIGHBORING_CHUNKS.len()
    }

    fn chunk_exists(&self, chunk_pos: &ChunkPos) -> bool {
        self.loaded_chunks.contains_key(chunk_pos)
            || self.chunk_data_tasks.contains_key(chunk_pos)
            || self.chunk_mesh_tasks.contains_key(chunk_pos)
            || self.chunks_to_mesh.contains(chunk_pos)
    }
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<World>()
            .add_systems(PreUpdate, unload)
            .add_systems(
                Update,
                (
                    join_data_tasks,
                    start_data_tasks,
                    join_mesh_tasks,
                    start_mesh_tasks,
                )
                    .chain(),
            )
            .add_systems(
                Update,
                unload_all.run_if(resource_changed::<ButtonInput<KeyCode>>),
            );
    }
}
pub const MAX_DATA_TASKS: usize = 64;
pub const MAX_MESH_TASKS: usize = 32;

// we dont care about diagonals for face culling
const NEIGHBORING_CHUNKS: [IVec2; 4] = [
    IVec2::new(0, -1),
    IVec2::new(-1, 0),
    IVec2::new(1, 0),
    IVec2::new(0, 1),
];

fn unload_all(mut world: ResMut<World>, mut commands: Commands, inputs: Res<ButtonInput<KeyCode>>) {
    if !inputs.pressed(KeyT) {
        return;
    }

    let World {
        loaded_chunks,
        chunk_data_tasks,
        chunk_mesh_tasks,
        chunks_to_mesh,
        chunks_to_load,
        chunk_entities,
        ..
    } = &mut *world;

    for (chunk_pos, _) in loaded_chunks.drain() {
        chunk_data_tasks.remove(&chunk_pos);
        chunk_mesh_tasks.remove(&chunk_pos);
        chunks_to_mesh.shift_remove(&chunk_pos);
        chunks_to_load.shift_remove(&chunk_pos);
        chunks_to_load.insert(chunk_pos);

        if let Some(chunk_entity_id) = chunk_entities.remove(&chunk_pos) {
            commands.entity(chunk_entity_id).despawn();
        };
    }
}

fn unload(mut world: ResMut<World>, mut commands: Commands) {
    let World {
        loaded_chunks,
        chunk_data_tasks,
        chunk_mesh_tasks,
        chunks_to_mesh,
        chunks_to_load,
        chunk_entities,
        chunks_to_unload,
        ..
    } = &mut *world;

    for chunk_pos in chunks_to_unload.drain() {
        loaded_chunks.remove(&chunk_pos);
        chunk_data_tasks.remove(&chunk_pos);
        chunk_mesh_tasks.remove(&chunk_pos);
        chunks_to_mesh.shift_remove(&chunk_pos);
        chunks_to_load.shift_remove(&chunk_pos);

        if let Some(chunk_entity_id) = chunk_entities.remove(&chunk_pos) {
            commands.entity(chunk_entity_id).despawn();
        };
    }
}

fn start_data_tasks(mut world: ResMut<World>) {
    if world.chunk_data_tasks.len() >= MAX_DATA_TASKS {
        return;
    }

    let World {
        chunk_data_tasks,
        chunks_to_load,
        ..
    } = &mut *world;

    let task_pool = AsyncComputeTaskPool::get();
    let count = (MAX_DATA_TASKS - chunk_data_tasks.len()).min(chunks_to_load.len());
    for chunk_pos in chunks_to_load.drain(..count) {
        let task = task_pool.spawn::<Chunk>(async move { generate_chunk_at(chunk_pos) });
        chunk_data_tasks.insert(chunk_pos, task);
    }
}

fn start_mesh_tasks(mut world: ResMut<World>) {
    if world.chunk_mesh_tasks.len() >= MAX_MESH_TASKS {
        return;
    }

    let World {
        chunk_mesh_tasks,
        loaded_chunks,
        chunks_to_mesh,
        ..
    } = &mut *world;

    let task_pool = AsyncComputeTaskPool::get();
    let count = (MAX_MESH_TASKS - chunk_mesh_tasks.len()).min(chunks_to_mesh.len());
    for chunk_pos in chunks_to_mesh.drain(..count) {
        let chunk = Arc::clone(&loaded_chunks[&chunk_pos]);
        let task = task_pool.spawn::<Mesh>(async move {
            Plane3d::default()
                .mesh()
                .size(CHUNK_SIZE, CHUNK_SIZE)
                .into()
        });
        chunk_mesh_tasks.insert(chunk_pos, task);
    }
}

fn join_data_tasks(mut world: ResMut<World>) {
    let World {
        chunk_data_tasks,
        loaded_chunks,
        chunks_to_mesh,
        chunk_entities,
        chunks_to_unload,
        ..
    } = &mut *world;

    chunk_data_tasks.retain(|&chunk_pos, task| {
        let status = block_on(poll_once(task));
        if let Some(chunk) = status {
            // verify that the chunk wasnt queued to unload when this thread got run
            // fixes stale entries when moving very fast
            if chunks_to_unload.contains(&chunk_pos) {
                return false;
            }

            loaded_chunks.insert(chunk_pos, Arc::new(chunk));

            // only insert into chunks to mesh when that chunk is "ready" (all of its neighbors also have chunk data so can properly cull)

            // center chunk
            if World::is_chunk_ready(loaded_chunks, chunk_pos)
                && !chunk_entities.contains_key(&chunk_pos)
            {
                chunks_to_mesh.insert(chunk_pos);
            }

            // update neighboring chunks
            for neighboring_chunk_pos in NEIGHBORING_CHUNKS {
                let absolute_chunk_pos = chunk_pos + neighboring_chunk_pos;
                if World::is_chunk_ready(loaded_chunks, absolute_chunk_pos)
                    && !chunk_entities.contains_key(&absolute_chunk_pos)
                {
                    chunks_to_mesh.insert(absolute_chunk_pos);
                }
            }
            false
        } else {
            true
        }
    });
}

fn join_mesh_tasks(
    mut world: ResMut<World>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let World {
        chunk_mesh_tasks,
        chunk_entities,
        chunks_to_unload,
        ..
    } = &mut *world;

    chunk_mesh_tasks.retain(|&chunk_pos, task| {
        let status = block_on(poll_once(task));
        if let Some(mesh) = status {
            // verify that the chunk wasnt queued to unload when this thread got run
            // fixes stale entries when moving very fast
            if chunks_to_unload.contains(&chunk_pos) {
                return false;
            }

            let entity = commands
                .spawn((
                    Transform::from_xyz(
                        chunk_pos.0.x as f32 * CHUNK_SIZE,
                        0.0,
                        chunk_pos.0.y as f32 * CHUNK_SIZE,
                    ),
                    chunk_pos,
                    Mesh3d(meshes.add(mesh)),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color: GREEN.into(),
                        ..default()
                    })),
                ))
                .id();
            chunk_entities.insert(chunk_pos, entity);
            false
        } else {
            true
        }
    });
}

fn generate_chunk_at(chunk_pos: ChunkPos) -> Chunk {
    let mut chunk = Chunk::default();
    chunk.generate();
    chunk
}
