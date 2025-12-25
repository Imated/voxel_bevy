use crate::chunk::{Chunk, ChunkPos, CHUNK_SIZE};
use crate::chunk_mesh::ChunkMesh;
use crate::greedy_chunk_render_plugin::generate_chunk_mesh;
use bevy::app::{App, Plugin, Update};
use bevy::asset::{Assets, RenderAssetUsages};
use bevy::color::{Color, Srgba};
use bevy::mesh::{Indices, Mesh, Mesh3d, PrimitiveTopology};
use bevy::pbr::{MeshMaterial3d, StandardMaterial};
use bevy::prelude::{Commands, Entity, ResMut, Resource, Transform};
use bevy::tasks::Task;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Resource, Debug, Default)]
pub struct World {
    pub(crate) loaded_chunks: HashMap<ChunkPos, Arc<Chunk>>,

    pub(crate) chunks_data_to_load: Vec<ChunkPos>,
    pub(crate) chunks_data_to_unload: Vec<ChunkPos>,

    pub(crate) chunks_mesh_to_load: Vec<ChunkPos>,
    pub(crate) chunks_mesh_to_unload: Vec<ChunkPos>,

    pub(crate) data_tasks: HashMap<ChunkPos, Task<Chunk>>,
    pub(crate) mesh_tasks: HashMap<ChunkPos, Task<ChunkMesh>>,

    chunk_entities: HashMap<ChunkPos, Entity>,
}

impl World {
    pub fn load_chunk(&mut self, position: ChunkPos) {
        if self.loaded_chunks.contains_key(&position) || self.chunks_data_to_load.contains(&position) {
            return;
        }

        self.chunks_data_to_load.push(position);
    }

    pub fn unload_chunk(&mut self, position: ChunkPos) {
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
            println!("Chunk {:?} mesh: {} vertices, {} indices",
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
                    chunk_pos.0.x as f32 * CHUNK_SIZE as f32,
                    0.0,
                    chunk_pos.0.y as f32 * CHUNK_SIZE as f32
                ),
                chunk_pos,
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