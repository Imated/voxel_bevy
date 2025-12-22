use std::mem::transmute;
use std::time::Instant;
use crate::block::{Block, MAX_BLOCK_ID};
use crate::chunk::{Chunk, CHUNK_HEIGHT, CHUNK_SIZE};
use crate::quad::{Direction, GreedyQuad};
use bevy::app::App;
use bevy::asset::RenderAssetUsages;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;
use bevy::reflect::Array;

#[derive(Default)]
pub struct GreedyChunkRenderPlugin;

impl Plugin for GreedyChunkRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, Self::render_chunks)
            .add_systems(Startup, Self::test_gen_chunks);
    }
}

impl GreedyChunkRenderPlugin {
    // https://github.com/TanTanDev/binary_greedy_mesher_demo/blob/main/src/greedy_mesher.rs#L251
    fn greedy_mesh_binary_plane(mut data: [u16; 16]) -> Vec<GreedyQuad> {
        let mut greedy_quads = vec![];
        for row in 0..data.len() {
            let mut y: u32 = 0;
            while y < CHUNK_SIZE as u32 {
                y += (data[row] >> y).trailing_zeros(); // air blocks offset
                if y >= CHUNK_SIZE as u32 {
                    continue;
                }
                let height = (data[row] >> y).trailing_ones();
                // 1 = 0b1, 2 = 0b11, etc
                let height_mask = u16::checked_shl(1, height).map_or(!0, |v| v - 1);
                let mask = height_mask << y;

                data[row] &= !mask;

                let mut w = 1;
                while row + w < CHUNK_SIZE as usize {
                    let next_row_height = (data[row + w] >> y) & height_mask;
                    if next_row_height != height_mask {
                        break; // cant expand
                    }

                    //remove bits we expanded into so we dont reuse them for later quads
                    data[row + w] = data[row + w] & !mask;
                    w += 1;
                }

                greedy_quads.push(GreedyQuad {
                    x: row as u32,
                    y,
                    w: w as u32,
                    h: height,
                });
                y += height;
            }
        }

        greedy_quads
    }

    fn vertices_from_face(direction: Direction, chunk: &Chunk) -> (Vec<[f32; 3]>, Vec<[f32; 3]>) {
        let mut vertices = vec![];
        let mut normals = vec![];
        for axis in 0..CHUNK_SIZE {
            for block_type in 0..MAX_BLOCK_ID {
                if !Block(block_type as u16).is_solid() {
                    continue;
                }
                let mut x_data = [0u16; 16];
                for i in 0..CHUNK_SIZE * CHUNK_SIZE {
                    let row = i % CHUNK_SIZE;
                    let column = i / CHUNK_SIZE;
                    let pos = direction.world_to_sample(axis, row, column);
                    let (current, in_front) = chunk.get_2(pos, direction.direction_in_front());
                    // dont merge if dif block types
                    if current != Block(block_type as u16) {
                        continue;
                    }
                    let is_solid = current.is_solid() && !in_front.is_solid();
                    x_data[row as usize] |= (is_solid as u16) << column;
                }
                let quads_from_axis = Self::greedy_mesh_binary_plane(x_data);
                quads_from_axis.into_iter().for_each(|quad| quad.append_vertices(&mut vertices, &mut normals, direction, axis as u32))
            }
        }

        (vertices, normals)
    }

    //https://github.com/TanTanDev/binary_greedy_mesher_demo/blob/main/src/utils.rs#L95
    fn generate_indices(vertex_count: usize) -> Vec<u16> {
        let quad_count = vertex_count / 4;
        let mut indices = Vec::<u16>::with_capacity(quad_count * 6);
        (0..quad_count).into_iter().for_each(|vert_index| {
            let base = vert_index as u16 * 4u16;
            indices.push(base);
            indices.push(base + 1);
            indices.push(base + 2);
            indices.push(base);
            indices.push(base + 2);
            indices.push(base + 3);
        });
        indices
    }

    fn generate_chunk_mesh(chunk: &Chunk) -> Mesh {
        let start = Instant::now();
        let mut vertices = vec![];
        let mut normals = vec![];

        for direction in [Direction::Up, Direction::Down, Direction::Left, Direction::Right, Direction::Forward, Direction::Back] {
            let (verts, norms) = Self::vertices_from_face(direction, &chunk);
            vertices.extend(verts);
            normals.extend(norms);
        }

        //info!("{:?}", Instant::now() - start);

        let mut mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::RENDER_WORLD,
        );
        mesh.insert_indices(Indices::U16(Self::generate_indices(vertices.len())));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);

        mesh
    }

    pub fn test_gen_chunks(mut commands: Commands) {
        let mut chunk = Chunk::new();
        for i in 0..CHUNK_SIZE * CHUNK_SIZE * CHUNK_HEIGHT {
            let coords = Chunk::coords_by_index(i);
            let dx = coords.x as f32;
            let dy = coords.y as f32;
            let dz = coords.z as f32;

            let voxel = if dx*dx + dy*dy + dz*dz < 64.0 {
                Block(1)
            } else {
                Block(0)
            };

            chunk.set_by_index(i, voxel);
        }
        commands.spawn((Transform::from_xyz(0., 0., 0.), chunk));
        commands.spawn((Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y), DirectionalLight {
            illuminance: 2_500.0,
            shadows_enabled: false,
            ..default()
        }));
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
            let chunk_material = materials.add(StandardMaterial {
                base_color: Color::Hsva(Hsva::hsv(97.8, 0.6, 0.97)),
                perceptual_roughness: 0.9,
                metallic: 0.0,
                double_sided: true,
                ..default()
            });
            chunk_entity.insert((Mesh3d(chunk_mesh), MeshMaterial3d(chunk_material)));
        }
    }
}
