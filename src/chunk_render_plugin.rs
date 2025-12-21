use crate::chunk::{Block, CHUNK_HEIGHT, CHUNK_SIZE, Chunk};
use bevy::app::App;
use bevy::asset::RenderAssetUsages;
use bevy::camera::Camera3dDepthLoadOp;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;
use std::hint::unreachable_unchecked;

#[derive(Default)]
pub struct ChunkRenderPlugin;

impl Plugin for ChunkRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, Self::render_chunks)
            .add_systems(Startup, Self::test_gen_chunks);
    }
}

impl ChunkRenderPlugin {
    fn add_face(coords: Vec3, face: i32, vertices: &mut Vec<[f32; 3]>, indices: &mut Vec<u16>) {
        let base = vertices.len() as u16;
        match face {
            0 => {
                // Top
                vertices.push([coords.x, coords.y + 1., coords.z]);
                vertices.push([coords.x, coords.y + 1., coords.z + 1.]);
                vertices.push([coords.x + 1., coords.y + 1., coords.z + 1.]);
                vertices.push([coords.x + 1., coords.y + 1., coords.z]);
            }
            1 => {
                // Bottom
                vertices.push([coords.x, coords.y, coords.z]);
                vertices.push([coords.x + 1., coords.y, coords.z]);
                vertices.push([coords.x + 1., coords.y, coords.z + 1.]);
                vertices.push([coords.x, coords.y, coords.z + 1.]);
            }
            2 => {
                // Left
                vertices.push([coords.x, coords.y, coords.z]);
                vertices.push([coords.x, coords.y, coords.z + 1.]);
                vertices.push([coords.x, coords.y + 1., coords.z + 1.]);
                vertices.push([coords.x, coords.y + 1., coords.z]);
            }
            3 => {
                // Right
                vertices.push([coords.x + 1., coords.y, coords.z + 1.]);
                vertices.push([coords.x + 1., coords.y, coords.z]);
                vertices.push([coords.x + 1., coords.y + 1., coords.z]);
                vertices.push([coords.x + 1., coords.y + 1., coords.z + 1.]);
            }
            4 => {
                // Front
                vertices.push([coords.x, coords.y, coords.z + 1.]);
                vertices.push([coords.x + 1., coords.y, coords.z + 1.]);
                vertices.push([coords.x + 1., coords.y + 1., coords.z + 1.]);
                vertices.push([coords.x, coords.y + 1., coords.z + 1.]);
            }
            5 => {
                // Back
                vertices.push([coords.x + 1., coords.y, coords.z]);
                vertices.push([coords.x, coords.y, coords.z]);
                vertices.push([coords.x, coords.y + 1., coords.z]);
                vertices.push([coords.x + 1., coords.y + 1., coords.z]);
            }
            _ => unsafe { unreachable_unchecked() },
        }
        indices.push(base);
        indices.push(base + 1);
        indices.push(base + 2);
        indices.push(base);
        indices.push(base + 2);
        indices.push(base + 3);
    }

    fn generate_chunk_mesh(chunk: &Chunk) -> Mesh {
        let mut vertices = Vec::new();
        //let mut normals = Vec::new();
        let mut indices = Vec::new();
        for i in 0..CHUNK_SIZE * CHUNK_SIZE * CHUNK_HEIGHT {
            let coords = Chunk::coords_by_index(i);
            let block = chunk.get_by_index(i);
            if block == Block(0) {
                // 0 = air
                continue;
            }

            let neighbors = [
                chunk.get_by_xyz(coords.x, coords.y + 1, coords.z),
                chunk.get_by_xyz(coords.x, coords.y - 1, coords.z),
                chunk.get_by_xyz(coords.x - 1, coords.y, coords.z),
                chunk.get_by_xyz(coords.x + 1, coords.y, coords.z),
                chunk.get_by_xyz(coords.x, coords.y, coords.z + 1),
                chunk.get_by_xyz(coords.x, coords.y, coords.z - 1),
            ];

            for face in 0..6 {
                if neighbors[face] == Block(0) {
                    Self::add_face(coords.as_vec3(), face as i32, &mut vertices, &mut indices);
                }
            }
        }

        let mut mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::RENDER_WORLD,
        );
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        //mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_indices(Indices::U16(indices));

        mesh
    }

    pub fn test_gen_chunks(mut commands: Commands) {
        let mut chunk = Chunk::new();
        for i in 0..CHUNK_SIZE * CHUNK_SIZE * CHUNK_HEIGHT {
            let coords = Chunk::coords_by_index(i);

            let voxel = if coords.x < 4 && coords.y < 4 && coords.z < 4 {
                Block(1)
            } else {
                Block(0)
            };

            chunk.set_by_index(i, voxel);
        }
        commands.spawn((Transform::from_xyz(0., 0., 0.), chunk));
    }

    pub fn render_chunks(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        query: Query<(Entity, &Chunk)>,
    ) {
        for (entity, chunk) in query {
            let mut chunk_entity = commands.entity(entity);
            let chunk_mesh = meshes.add(Self::generate_chunk_mesh(chunk));
            let chunk_material = materials.add(StandardMaterial::from_color(Color::Hsva(
                Hsva::hsv(97.8, 60.3, 96.9),
            )));
            chunk_entity.insert((Mesh3d(chunk_mesh), MeshMaterial3d(chunk_material)));
        }
    }
}
