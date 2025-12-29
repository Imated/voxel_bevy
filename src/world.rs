use crate::chunk::{CHUNK_SIZE, Chunk, ChunkPos};
use crate::chunk_mesh::ChunkSectionMesh;
use crate::greedy_chunk_render_plugin::generate_section_mesh;
use crate::lighting::rendering::GlobalChunkMaterial;
use crate::section_neighbors::SectionNeighbors;
use bevy::app::{App, Plugin, PostUpdate, Update};
use bevy::asset::{Assets, RenderAssetUsages};
use bevy::mesh::{Indices, Mesh, Mesh3d, PrimitiveTopology};
use bevy::pbr::MeshMaterial3d;
use bevy::prelude::{Commands, Entity, IntoScheduleConfigs, Query, Res, ResMut, Resource, Transform, ViewVisibility, With};
use bevy::tasks::{AsyncComputeTaskPool, Task, block_on, poll_once};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use bevy::camera::Camera;
use bevy::camera::primitives::Aabb;
use bevy::math::Vec3;
use bevy::time::common_conditions::on_timer;
use crate::chunk_material::ATTRIBUTE_VOXEL;

#[derive(Resource, Debug, Default)]
pub struct World {
    pub(crate) loaded_chunks: HashMap<ChunkPos, Arc<Chunk>>,

    pub(crate) chunks_data_to_load: Vec<ChunkPos>,
    pub(crate) chunks_data_to_unload: Vec<ChunkPos>,

    pub(crate) chunks_mesh_to_load: Vec<ChunkPos>,
    pub(crate) chunks_mesh_to_unload: Vec<ChunkPos>,

    pub(crate) data_tasks: HashMap<ChunkPos, Task<Chunk>>,
    pub(crate) mesh_tasks: HashMap<(ChunkPos, i32), Task<Option<ChunkSectionMesh>>>,

    chunk_entities: HashMap<ChunkPos, Entity>,
    chunk_sections: HashMap<ChunkPos, HashMap<i32, Option<ChunkSectionMesh>>>,
}

impl World {
    pub fn load_chunk(&mut self, position: ChunkPos) {
        if self.loaded_chunks.contains_key(&position)
            || self.chunks_data_to_load.contains(&position)
        {
            return;
        }

        self.chunks_data_to_load.push(position);
    }

    pub fn unload_chunk(&mut self, position: ChunkPos) {
        if !self.loaded_chunks.contains_key(&position)
            && !self.chunks_data_to_load.contains(&position)
        {
            return;
        }
        self.chunks_data_to_unload.push(position);
    }
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(World::default())
            .add_systems(PostUpdate, (Self::start_data_tasks, Self::start_mesh_tasks))
            .add_systems(
                Update,
                (
                    (Self::join_data_tasks, Self::join_mesh_tanks),
                    Self::unload_meshes,
                    Self::unload_data,
                )
                    .chain(),
            );
    }
}

impl WorldPlugin {
    pub fn unload_data(mut world: ResMut<World>) {
        let chunks_to_unload: Vec<_> = world.chunks_data_to_unload.drain(..).collect();

        for chunk_pos in chunks_to_unload {
            let chunk = world.loaded_chunks.remove(&chunk_pos);
            if let Some(_chunk) = chunk {
                world.chunks_mesh_to_unload.push(chunk_pos);
            }
        }
    }

    pub fn unload_meshes(mut commands: Commands, mut world: ResMut<World>) {
        let chunks_to_unload: Vec<_> = world.chunks_mesh_to_unload.drain(..).collect();

        for (chunk_pos) in chunks_to_unload {
            let Some(chunk_id) = world.chunk_entities.remove(&chunk_pos) else {
                continue;
            };

            if let Ok(mut entity) = commands.get_entity(chunk_id) {
                entity.despawn();
            }
        }
    }

    fn start_data_tasks(mut world: ResMut<World>) {
        let task_pool = AsyncComputeTaskPool::get();
        let chunks_to_load: Vec<_> = world.chunks_data_to_load.drain(..).collect();
        for chunk_pos in chunks_to_load {
            if world.loaded_chunks.contains_key(&chunk_pos)
                || world.data_tasks.contains_key(&chunk_pos)
            {
                continue;
            }

            let task = task_pool.spawn::<Chunk>(async move { Self::generate_chunk_at(chunk_pos) });
            world.data_tasks.insert(chunk_pos, task);
        }
    }

    fn join_data_tasks(mut world: ResMut<World>) {
        let mut completed_chunks = vec![];

        world.data_tasks.retain(|&chunk_pos, task| {
            let status = block_on(poll_once(task));
            let retain = status.is_none();
            if let Some(chunk) = status {
                completed_chunks.push((chunk_pos, chunk));
            }
            retain
        });

        for (chunk_pos, chunk) in completed_chunks {
            world.loaded_chunks.insert(chunk_pos, Arc::new(chunk));
            world.chunks_mesh_to_load.push(chunk_pos);
        }
    }

    fn start_mesh_tasks(mut world: ResMut<World>) {
        let task_pool = AsyncComputeTaskPool::get();
        let chunks_to_mesh: Vec<_> = world.chunks_mesh_to_load.drain(..).collect();
        for chunk_pos in chunks_to_mesh {
            let chunk = Arc::clone(&world.loaded_chunks[&chunk_pos]);
            for section_y in 0..chunk.sections.len() {
                let section = SectionNeighbors::new(&world.loaded_chunks, chunk_pos, section_y);

                let task = task_pool.spawn::<Option<ChunkSectionMesh>>(async move {
                    generate_section_mesh(section, section_y as i32)
                });
                world.mesh_tasks.insert((chunk_pos, section_y as i32), task);
            }
        }
    }

    fn join_mesh_tanks(
        mut commands: Commands,
        mut world: ResMut<World>,
        mut meshes: ResMut<Assets<Mesh>>,
        material: Res<GlobalChunkMaterial>,
    ) {
        let mut completed = vec![];

        world.mesh_tasks.retain(|&(chunk_pos, section_y), task| {
            if let Some(section) = block_on(poll_once(task)) {
                completed.push((chunk_pos, section_y, section));
                false
            } else {
                true
            }
        });

        for (chunk_pos, section_y, section) in completed {
            world.chunk_sections.entry(chunk_pos).or_default().insert(section_y, section);
        }

        let chunks_to_spawn: Vec<_> = world.chunk_sections
            .iter()
            .filter(|(chunk_pos, sections)| {
                let expected = world.loaded_chunks[chunk_pos].sections.len();
                sections.len() == expected
            })
            .map(|(pos, _)| *pos)
            .collect();

        for chunk_pos in chunks_to_spawn {
            let sections = world.chunk_sections.remove(&chunk_pos).unwrap();

            let mut all_vertices = Vec::new();
            let mut all_indices = Vec::new();
            let mut vertex_offset = 0u32;

            for (_section_y, section) in sections.iter() {
                let Some(section) = section.as_ref() else {
                    continue;
                };

                all_vertices.extend_from_slice(&section.vertices);
                all_indices.extend(section.indices.iter().map(|&i| i + vertex_offset));
                vertex_offset += section.vertices.len() as u32;
            }

            if all_vertices.is_empty() {
                continue;
            }

            let mut mesh = Mesh::new(
                PrimitiveTopology::TriangleList,
                RenderAssetUsages::RENDER_WORLD,
            );

            mesh.insert_attribute(ATTRIBUTE_VOXEL, all_vertices);
            mesh.insert_indices(Indices::U32(all_indices));

            if let Some(old_entity) = world.chunk_entities.remove(&chunk_pos) {
                commands.entity(old_entity).despawn();
            }

            let entity = commands
                .spawn((
                    Mesh3d(meshes.add(mesh)),
                    MeshMaterial3d(material.0.clone()),
                    Transform::from_xyz(
                        chunk_pos.0.x as f32 * CHUNK_SIZE as f32,
                        0.0,
                        chunk_pos.0.y as f32 * CHUNK_SIZE as f32,
                    ),
                    chunk_pos,
                    Aabb::from_min_max(Vec3::ZERO, Vec3::new(CHUNK_SIZE as f32, CHUNK_SIZE as f32 * sections.len() as f32, CHUNK_SIZE as f32))
                ))
                .id();

            world.chunk_entities.insert(chunk_pos, entity);
        }
    }

    pub fn generate_chunk_at(_coord: ChunkPos) -> Chunk {
        let mut chunk = Chunk::new();
        chunk.generate();
        chunk
    }
}
