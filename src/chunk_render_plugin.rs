use crate::block::Block;
use crate::chunk::{CHUNK_SIZE3, Chunk, CHUNK_SIZE};
use bevy::app::App;
use bevy::asset::RenderAssetUsages;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;
use std::hint::unreachable_unchecked;
use std::time::Instant;

#[derive(Default)]
pub struct ChunkRenderPlugin;

impl Plugin for ChunkRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::test_gen_chunks);
    }
}

impl ChunkRenderPlugin {
    fn add_face(
        coords: Vec3,
        face: i32,
        vertices: &mut Vec<[f32; 3]>,
        normals: &mut Vec<[f32; 3]>,
        indices: &mut Vec<u16>,
    ) {
        let base = vertices.len() as u16;
        match face {
            0 => {
                // Top
                vertices.push([coords.x, coords.y + 1., coords.z]);
                vertices.push([coords.x, coords.y + 1., coords.z + 1.]);
                vertices.push([coords.x + 1., coords.y + 1., coords.z + 1.]);
                vertices.push([coords.x + 1., coords.y + 1., coords.z]);
                normals.push([0.0, 1.0, 0.0]);
                normals.push([0.0, 1.0, 0.0]);
                normals.push([0.0, 1.0, 0.0]);
                normals.push([0.0, 1.0, 0.0]);
            }
            1 => {
                // Bottom
                vertices.push([coords.x, coords.y, coords.z]);
                vertices.push([coords.x + 1., coords.y, coords.z]);
                vertices.push([coords.x + 1., coords.y, coords.z + 1.]);
                vertices.push([coords.x, coords.y, coords.z + 1.]);
                normals.push([0.0, -1.0, 0.0]);
                normals.push([0.0, -1.0, 0.0]);
                normals.push([0.0, -1.0, 0.0]);
                normals.push([0.0, -1.0, 0.0]);
            }
            2 => {
                // Left
                vertices.push([coords.x, coords.y, coords.z]);
                vertices.push([coords.x, coords.y, coords.z + 1.]);
                vertices.push([coords.x, coords.y + 1., coords.z + 1.]);
                vertices.push([coords.x, coords.y + 1., coords.z]);
                normals.push([-1.0, 0.0, 0.0]);
                normals.push([-1.0, 0.0, 0.0]);
                normals.push([-1.0, 0.0, 0.0]);
                normals.push([-1.0, 0.0, 0.0]);
            }
            3 => {
                // Right
                vertices.push([coords.x + 1., coords.y, coords.z + 1.]);
                vertices.push([coords.x + 1., coords.y, coords.z]);
                vertices.push([coords.x + 1., coords.y + 1., coords.z]);
                vertices.push([coords.x + 1., coords.y + 1., coords.z + 1.]);
                normals.push([1.0, 0.0, 0.0]);
                normals.push([1.0, 0.0, 0.0]);
                normals.push([1.0, 0.0, 0.0]);
                normals.push([1.0, 0.0, 0.0]);
            }
            4 => {
                // Front
                vertices.push([coords.x, coords.y, coords.z + 1.]);
                vertices.push([coords.x + 1., coords.y, coords.z + 1.]);
                vertices.push([coords.x + 1., coords.y + 1., coords.z + 1.]);
                vertices.push([coords.x, coords.y + 1., coords.z + 1.]);
                normals.push([0.0, 0.0, 1.0]);
                normals.push([0.0, 0.0, 1.0]);
                normals.push([0.0, 0.0, 1.0]);
                normals.push([0.0, 0.0, 1.0]);
            }
            5 => {
                // Back
                vertices.push([coords.x + 1., coords.y, coords.z]);
                vertices.push([coords.x, coords.y, coords.z]);
                vertices.push([coords.x, coords.y + 1., coords.z]);
                vertices.push([coords.x + 1., coords.y + 1., coords.z]);
                normals.push([0.0, 0.0, -1.0]);
                normals.push([0.0, 0.0, -1.0]);
                normals.push([0.0, 0.0, -1.0]);
                normals.push([0.0, 0.0, -1.0]);
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
        let start = Instant::now();
        let mut vertices = Vec::new();
        let mut normals = Vec::new();
        let mut indices = Vec::new();
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let block = chunk.get_by_xyz(x, y, z);
                    if block == Block(0) {
                        // 0 = air
                        continue;
                    }

                    let neighbors = [
                        chunk.get_by_xyz(x, y + 1, z),
                        chunk.get_by_xyz(x, y - 1, z),
                        chunk.get_by_xyz(x - 1, y, z),
                        chunk.get_by_xyz(x + 1, y, z),
                        chunk.get_by_xyz(x, y, z + 1),
                        chunk.get_by_xyz(x, y, z - 1),
                    ];

                    for face in 0..6 {
                        if neighbors[face] == Block(0) {
                            Self::add_face(
                                vec3(x as f32, y as f32, z as f32),
                                face as i32,
                                &mut vertices,
                                &mut normals,
                                &mut indices,
                            );
                        }
                    }
                }
            }
        }

        // println!("{:?}", Instant::now() - start);

        let mut mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::RENDER_WORLD,
        );
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_indices(Indices::U16(indices));

        mesh
    }

    pub fn test_gen_chunks(mut commands: Commands) {
        let mut chunk = Chunk::new();
        commands.spawn((Transform::from_xyz(0., 0., 0.)));
        commands.spawn((
            Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            DirectionalLight {
                illuminance: 2_500.0,
                shadows_enabled: false,
                ..default()
            },
        ));
    }
}
