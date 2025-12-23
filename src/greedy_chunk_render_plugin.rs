use crate::block::Block;
use crate::chunk::{Chunk, CHUNK_SIZE, CHUNK_SIZE3, PADDED_CHUNK_SIZE, PADDED_CHUNK_SIZE2, PADDED_CHUNK_SIZE2_USIZE, PADDED_CHUNK_SIZE3, PADDED_CHUNK_SIZE3_USIZE, PADDED_CHUNK_SIZE_USIZE};
use crate::quad::{Direction, GreedyQuad};
use bevy::app::App;
use bevy::asset::RenderAssetUsages;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;
use bevy::reflect::Array;
use std::collections::HashMap;
use std::time::Instant;

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

    fn vertices_from_face(chunk: &Chunk) -> (Vec<[f32; 3]>, Vec<[f32; 3]>) {
        let mut vertices = vec![];
        let mut normals = vec![];

        // solid voxels as binary per axis x, y, z
        let mut solid_voxels_per_axis = vec![0u64; (3 * PADDED_CHUNK_SIZE3_USIZE)];
        // cull mask for greedy slicing based on solids on previous axis column
        let mut voxels_face_mask = [[[0u64; PADDED_CHUNK_SIZE_USIZE]; PADDED_CHUNK_SIZE_USIZE]; 6];

        for y in 0..PADDED_CHUNK_SIZE_USIZE {
            for z in 0..PADDED_CHUNK_SIZE_USIZE {
                for x in 0..PADDED_CHUNK_SIZE_USIZE {
                    let block = chunk.get_by_xyz(x as i32 - 1, y as i32 - 1, z as i32 - 1);
                    if block.is_solid() {
                        solid_voxels_per_axis[x + z * PADDED_CHUNK_SIZE_USIZE] |= 1u64 << y;
                        solid_voxels_per_axis[z + y * PADDED_CHUNK_SIZE_USIZE + PADDED_CHUNK_SIZE2_USIZE] |= 1u64 << x;
                        solid_voxels_per_axis[x + y * PADDED_CHUNK_SIZE_USIZE + PADDED_CHUNK_SIZE2_USIZE * 2] |= 1u64 << z;
                    }
                }
            }
        }

        //face cull
        for axis in 0..3usize {
            for z in 0..PADDED_CHUNK_SIZE_USIZE {
                for x in 0..PADDED_CHUNK_SIZE_USIZE {
                    let i = z * PADDED_CHUNK_SIZE_USIZE + x;
                    let col = solid_voxels_per_axis[(PADDED_CHUNK_SIZE2_USIZE * axis) + i];
                    // sample ascending/descending axes and set true if air meets solid aka need to draw face.
                    voxels_face_mask[2 * axis + 1][z][x] = col & !(col >> 1);
                    voxels_face_mask[2 * axis + 0][z][x] = col & !(col << 1);
                }
            }
        }

        // (axis, block, y) -> binary plane
        let mut data: HashMap<(u8, Block, u16), [u16; 16]> = Default::default();

        for axis in 0..6 {
            for z in 0..CHUNK_SIZE as usize {
                for x in 0..CHUNK_SIZE as usize {
                    // skip padded by adding 1(for x padding) and (z+1) for (z padding)
                    let mut col = voxels_face_mask[axis][z + 1][x + 1];

                    // remove right most out of chunk bounds value
                    col >>= 1;
                    // remove left most out of chunk bounds value
                    col &= !(1 << CHUNK_SIZE);

                    while col != 0 {
                        let y = col.trailing_zeros();
                        // clear last set bit
                        col &= col - 1;

                        let voxel_pos = match axis {
                            0 | 1 => ivec3(x as i32, y as i32, z as i32), // down | up
                            2 | 3 => ivec3(y as i32, z as i32, x as i32), // left | right
                            _ => ivec3(x as i32, z as i32, y as i32), // forward | back
                        };

                        let block = chunk.get(voxel_pos);
                        //let key = (axis, block, y);
                        let data = data.entry((axis as u8, block, y as u16)).or_default();
                        data[x] |= 1u16 << z as u16;
                    }
                }
            }
        }

        for (&(axis, block, axis_pos), &plane) in data.iter() {
            let face_dir = match axis {
                0 => Direction::Down,
                1 => Direction::Up,
                2 => Direction::Left,
                3 => Direction::Right,
                4 => Direction::Forward,
                _ => Direction::Back,
            };
            let quads_from_axis = Self::greedy_mesh_binary_plane(plane);

            quads_from_axis.into_iter().for_each(|q| {
                q.append_vertices(&mut vertices, &mut normals, face_dir, axis_pos as i32);
            });
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

        let (verts, norms) = Self::vertices_from_face(&chunk);
        vertices.extend(verts);
        normals.extend(norms);

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
        for i in 0..CHUNK_SIZE3 {
            let coords = Chunk::coords_by_index(i);
            let dx = coords.x as f32 - 8.0;
            let dy = coords.y as f32 - 8.0;
            let dz = coords.z as f32 - 8.0;

            let voxel = if dx*dx + dy*dy + dz*dz < 64.0 {
                Block(1)
            } else {
                Block(0)
            };

            chunk.set_by_index(i, voxel);
        }
        //let mut chunk_2 = Chunk::new();
        //for i in 0..CHUNK_SIZE3 {
        //    let coords = Chunk::coords_by_index(i);
        //    let dx = coords.x as f32 - 8.0;
        //    let dy = coords.y as f32 - 8.0;
        //    let dz = coords.z as f32 - 8.0;
        //
        //    let voxel = if dx*dx + dy*dy < 64.0 {
        //        Block(1)
        //    } else {
        //        Block(0)
        //    };
        //
        //    chunk_2.set_by_index(i, voxel);
        //}

        commands.spawn((Transform::from_xyz(0., 0., 0.), chunk.clone()));
        //commands.spawn((Transform::from_xyz(16., 0., 0.), chunk));
        //commands.spawn((Transform::from_xyz(8., 0., 16.), chunk_2));
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
