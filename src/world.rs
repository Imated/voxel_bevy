use crate::chunk::{Chunk, ChunkPos, CHUNK_SIZE};
use crate::chunk_mesh::ChunkSectionMesh;
use crate::greedy_chunk_render_plugin::generate_section_mesh;
use bevy::app::{App, Plugin, Startup, Update};
use bevy::asset::{Assets, Handle, RenderAssetUsages};
use bevy::color::{Color, Srgba};
use bevy::mesh::{Indices, Mesh, Mesh3d, PrimitiveTopology};
use bevy::pbr::{MeshMaterial3d, StandardMaterial};
use bevy::prelude::{Commands, Entity, Res, ResMut, Resource, Transform};
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
    pub(crate) mesh_tasks: HashMap<ChunkPos, Task<ChunkSectionMesh>>,

    chunk_entities: HashMap<ChunkPos, Vec<Entity>>,
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
        app.insert_resource(World::default()).add_systems(Update, Self::update_chunks).add_systems(Startup, Self::setup);
    }
}

#[derive(Resource)]
struct ChunkMaterial(Handle<StandardMaterial>);

impl WorldPlugin {
    pub fn setup(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>) {
        let material = materials.add(StandardMaterial::from_color(
            Color::Srgba(Srgba::rgb(0.3, 0.5, 0.3))
        ));

        commands.insert_resource(ChunkMaterial(material));
    }

    pub fn update_chunks(mut commands: Commands, mut world: ResMut<World>, mut meshes: ResMut<Assets<Mesh>>, mut material: Res<ChunkMaterial>) {
        let chunks_to_load: Vec<_> = world.chunks_data_to_load.drain(..).collect();

        for chunk_pos in chunks_to_load {
            if world.loaded_chunks.contains_key(&chunk_pos) {
                continue;
            }

            let mut chunk = Chunk::new();
            chunk.generate();
            let mut section_entities = vec![];
            for (section_y, section) in chunk.sections.iter().enumerate() {
                let section_data = section.read().unwrap();
                if section_data.is_empty() {
                    continue;
                }
                let section_mesh = generate_section_mesh(&section_data);

                let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD);
                mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, section_mesh.vertices);
                mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, section_mesh.normals);
                mesh.insert_indices(Indices::U32(section_mesh.indices));

                let entity = commands.spawn((
                    Mesh3d(meshes.add(mesh)),
                    MeshMaterial3d(material.0.clone()),
                    Transform::from_xyz(
                        chunk_pos.0.x as f32 * CHUNK_SIZE as f32,
                        CHUNK_SIZE as f32 * section_y as f32,
                        chunk_pos.0.y as f32 * CHUNK_SIZE as f32
                    ),
                    chunk_pos,
                )).id();

                section_entities.push(entity);
            }

            let chunk_data = Arc::new(chunk);
            world.chunk_entities.insert(chunk_pos, section_entities);
            world.loaded_chunks.insert(chunk_pos, chunk_data.clone());
        }

        let chunks_to_unload: Vec<_> = world.chunks_data_to_unload.drain(..).collect();

        // Unload chunks
        for chunk_pos in chunks_to_unload {
            if let Some(entities) = world.chunk_entities.remove(&chunk_pos) {
                for entity in entities {
                    commands.entity(entity).despawn();
                }
            }

            world.loaded_chunks.remove(&chunk_pos);
        }
    }
}