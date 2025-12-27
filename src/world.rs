use crate::chunk::{CHUNK_SIZE, Chunk, ChunkPos};
use crate::chunk_mesh::ChunkSectionMesh;
use crate::greedy_chunk_render_plugin::generate_section_mesh;
use bevy::app::{App, Plugin, PostUpdate, Startup, Update};
use bevy::asset::{Assets, Handle, RenderAssetUsages};
use bevy::color::{Color, Srgba};
use bevy::mesh::{Indices, Mesh, Mesh3d, PrimitiveTopology};
use bevy::pbr::{MeshMaterial3d, StandardMaterial};
use bevy::prelude::{Commands, Entity, IntoScheduleConfigs, Res, ResMut, Resource, Transform};
use bevy::tasks::{AsyncComputeTaskPool, Task, block_on, poll_once};
use std::collections::HashMap;
use std::sync::Arc;
use crate::section_neighbors::SectionNeighbors;

#[derive(Resource, Debug, Default)]
pub struct World {
    pub(crate) loaded_chunks: HashMap<ChunkPos, Arc<Chunk>>,

    pub(crate) chunks_data_to_load: Vec<ChunkPos>,
    pub(crate) chunks_data_to_unload: Vec<ChunkPos>,

    pub(crate) chunks_mesh_to_load: Vec<ChunkPos>,
    pub(crate) chunks_mesh_to_unload: Vec<(ChunkPos, usize)>, // pos, sections_amount

    pub(crate) data_tasks: HashMap<ChunkPos, Task<Chunk>>,
    pub(crate) mesh_tasks: HashMap<(ChunkPos, i32), Task<Option<ChunkSectionMesh>>>,

    section_entities: HashMap<(ChunkPos, i32), Entity>,
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
            .add_systems(Startup, Self::setup)
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

#[derive(Resource)]
struct ChunkMaterial(Handle<StandardMaterial>);

impl WorldPlugin {
    pub fn setup(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>) {
        let material = materials.add(StandardMaterial::from_color(Color::Srgba(Srgba::rgb(
            0.3, 0.5, 0.3,
        ))));

        commands.insert_resource(ChunkMaterial(material));
    }

    pub fn unload_data(mut world: ResMut<World>) {
        let chunks_to_unload: Vec<_> = world.chunks_data_to_unload.drain(..).collect();

        for chunk_pos in chunks_to_unload {
            let chunk = world.loaded_chunks.remove(&chunk_pos);
            if let Some(chunk) = chunk {
                world
                    .chunks_mesh_to_unload
                    .push((chunk_pos, chunk.sections.len()));
            }
        }
    }

    pub fn unload_meshes(mut commands: Commands, mut world: ResMut<World>) {
        let chunks_to_unload: Vec<_> = world.chunks_mesh_to_unload.drain(..).collect();

        for (chunk_pos, sections_len) in chunks_to_unload {
            for i in 0..sections_len {
                let Some(chunk_id) = world.section_entities.remove(&(chunk_pos, i as i32)) else {
                    continue;
                };

                if let Ok(mut entity) = commands.get_entity(chunk_id) {
                    entity.despawn();
                }
            }
        }
    }

    pub fn start_data_tasks(mut world: ResMut<World>) {
        let task_pool = AsyncComputeTaskPool::get();
        let chunks_to_load: Vec<_> = world.chunks_data_to_load.drain(..).collect();
        for chunk_pos in chunks_to_load {
            if world.loaded_chunks.contains_key(&chunk_pos)
                || world.data_tasks.contains_key(&chunk_pos) {
                continue;
            }

            let task = task_pool.spawn::<Chunk>(async move {
                Self::generate_chunk_at(chunk_pos)
            });
            world.data_tasks.insert(chunk_pos, task);
        }
    }

    pub fn join_data_tasks(mut world: ResMut<World>) {
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

    pub fn start_mesh_tasks(mut world: ResMut<World>) {
        let task_pool = AsyncComputeTaskPool::get();
        let chunks_to_mesh: Vec<_> = world.chunks_mesh_to_load.drain(..).collect();
        for chunk_pos in chunks_to_mesh {
            let chunk = Arc::clone(&world.loaded_chunks[&chunk_pos]);
            for section_y in 0..chunk.sections.len() {
                let section = SectionNeighbors::new(&world.loaded_chunks, chunk_pos, section_y);

                let task = task_pool.spawn::<Option<ChunkSectionMesh>>(async move {
                    generate_section_mesh(section)
                });
                world.mesh_tasks.insert((chunk_pos, section_y as i32), task);
            }
        }
    }

    pub fn join_mesh_tanks(
        mut commands: Commands,
        mut world: ResMut<World>,
        mut meshes: ResMut<Assets<Mesh>>,
        material: Res<ChunkMaterial>,
    ) {
        let mut completed_sections = vec![];

        world.mesh_tasks.retain(|&(chunk_pos, section_y), task| {
            let status = block_on(poll_once(task));
            let retain = status.is_none();
            if let Some(section) = status {
                if section.is_none() {
                    // section is empty!
                    return false;
                }
                let section_mesh = section.unwrap();
                completed_sections.push((chunk_pos, section_y, section_mesh));
            }
            retain
        });

        for (chunk_pos, section_y, section_mesh) in completed_sections {
            let mut mesh = Mesh::new(
                PrimitiveTopology::TriangleList,
                RenderAssetUsages::RENDER_WORLD,
            );
            mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, section_mesh.vertices);
            mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, section_mesh.normals);
            mesh.insert_indices(Indices::U32(section_mesh.indices));

            if let Some(&entity) = world.section_entities.get(&(chunk_pos, section_y)) {
                commands.entity(entity).despawn();
            }

            let entity = commands
                .spawn((
                    Mesh3d(meshes.add(mesh)),
                    MeshMaterial3d(material.0.clone()),
                    Transform::from_xyz(
                        chunk_pos.0.x as f32 * CHUNK_SIZE as f32,
                        section_y as f32 * CHUNK_SIZE as f32,
                        chunk_pos.0.y as f32 * CHUNK_SIZE as f32,
                    ),
                    chunk_pos,
                ))
                .id();

            world
                .section_entities
                .insert((chunk_pos, section_y), entity);
        }
    }

    pub fn generate_chunk_at(coord: ChunkPos) -> Chunk {
        let mut chunk = Chunk::new();
        chunk.generate();
        chunk
    }
}
