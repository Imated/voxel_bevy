use crate::chunk::{Chunk, CHUNK_SIZE};
use crate::chunk_mesh::ChunkMesh;
use bevy::app::{App, Plugin, Update};
use bevy::math::IVec3;
use bevy::prelude::{Commands, Component, Entity, GlobalTransform, Query, ResMut, Resource, Transform, With};
use bevy::tasks::Task;
use std::collections::HashMap;
use std::sync::Arc;
use bevy::asset::{Assets, RenderAssetUsages};
use bevy::color::{Color, Srgba};
use bevy::log::tracing::span::Attributes;
use bevy::mesh::{Indices, Mesh, Mesh3d, MeshVertexAttribute, PrimitiveTopology};
use bevy::pbr::{MeshMaterial3d, StandardMaterial};
use crate::greedy_chunk_render_plugin::generate_chunk_mesh;

#[derive(Resource, Debug, Default)]
pub struct World {
    pub(crate) loaded_chunks: HashMap<IVec3, Arc<Chunk>>,

    pub(crate) chunks_data_to_load: Vec<IVec3>,
    pub(crate) chunks_data_to_unload: Vec<IVec3>,

    pub(crate) chunks_mesh_to_load: Vec<IVec3>,
    pub(crate) chunks_mesh_to_unload: Vec<IVec3>,

    pub(crate) data_tasks: HashMap<IVec3, Task<Chunk>>,
    pub(crate) mesh_tasks: HashMap<IVec3, Task<ChunkMesh>>,

    chunk_entities: HashMap<IVec3, Entity>,
}

impl World {
    pub fn load_chunk(&mut self, position: IVec3) {
        if self.loaded_chunks.contains_key(&position) || self.chunks_data_to_load.contains(&position) {
            return;
        }

        self.chunks_data_to_load.push(position);
    }

    pub fn unload_chunk(&mut self, position: IVec3) {
        if !self.loaded_chunks.contains_key(&position) && !self.chunks_data_to_load.contains(&position) {
            return;
        }
        self.chunks_data_to_unload.push(position);
    }
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(World::default()).add_systems(Update, Self::update_chunks);
    }
}

#[derive(Component)]
struct ChunkPos(IVec3);

impl WorldPlugin {
    pub fn update_chunks(mut commands: Commands, mut world: ResMut<World>, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
        let chunks_to_load: Vec<_> = world.chunks_data_to_load.drain(..).collect();

        for chunk_pos in chunks_to_load {
            if world.loaded_chunks.contains_key(&chunk_pos) {
                continue;
            }

            let mut chunk = Chunk::new();
            chunk.generate();
            let chunk_data = Arc::new(chunk);
            let chunk_mesh = generate_chunk_mesh(Arc::clone(&chunk_data));
            println!("Chunk {} mesh: {} vertices, {} indices",
                     chunk_pos, chunk_mesh.vertices.len(), chunk_mesh.indices.len());

            let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD);
            mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, chunk_mesh.vertices);
            mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, chunk_mesh.normals);
            mesh.insert_indices(Indices::U32(chunk_mesh.indices));

            let entity = commands.spawn((
                Mesh3d(meshes.add(mesh)),
                MeshMaterial3d(materials.add(StandardMaterial::from_color(
                    Color::Srgba(Srgba::rgb(0.3, 0.5, 0.3))
                ))),
                Transform::from_xyz(
                    chunk_pos.x as f32 * CHUNK_SIZE as f32,
                    chunk_pos.y as f32 * CHUNK_SIZE as f32,
                    chunk_pos.z as f32 * CHUNK_SIZE as f32
                ),
                ChunkPos(chunk_pos),
            )).id();

            world.loaded_chunks.insert(chunk_pos, Arc::clone(&chunk_data));
            world.chunk_entities.insert(chunk_pos, entity);
        }

        let chunks_to_unload: Vec<_> = world.chunks_data_to_unload.drain(..).collect();

        // Unload chunks
        for chunk_pos in chunks_to_unload {
            if let Some(entity) = world.chunk_entities.remove(&chunk_pos) {
                commands.entity(entity).despawn();
            }

            world.loaded_chunks.remove(&chunk_pos);
        }
    }
}