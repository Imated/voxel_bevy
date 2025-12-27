use crate::block::Block;
use crate::chunk::{
    CHUNK_SIZE, ChunkSection, PADDED_CHUNK_SIZE_USIZE, PADDED_CHUNK_SIZE2_USIZE,
    PADDED_CHUNK_SIZE3_USIZE,
};
use crate::chunk_mesh::ChunkSectionMesh;
use crate::quad::{Direction, GreedyQuad};
use crate::section_neighbors::SectionNeighbors;
use bevy::app::App;
use bevy::prelude::*;
use bevy::reflect::Array;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Default)]
pub struct GreedyChunkRenderPlugin;

impl Plugin for GreedyChunkRenderPlugin {
    fn build(&self, app: &mut App) {}
}

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

#[inline]
fn get_block_at_section(section: &Option<Arc<RwLock<ChunkSection>>>, x: i32, y: i32, z: i32) -> Block {
    if section.is_none() {
        return Block(0)
    }

    let up = section.as_ref().unwrap();
    let up_section = up.read().unwrap();
    up_section.get_by_xyz(x, y, z).unwrap()
}

pub fn generate_section_mesh(sections: SectionNeighbors) -> Option<ChunkSectionMesh> {
    let section_data = sections.center.read().unwrap();
    if section_data.is_empty() {
        return None;
    }

    let mut vertices = vec![];
    let mut normals = vec![];

    // solid voxels as binary per axis x, y, z
    let mut solid_voxels_per_axis = vec![0u64; (3 * PADDED_CHUNK_SIZE3_USIZE)];
    // cull mask for greedy slicing based on solids on previous axis column
    let mut voxels_face_mask = [[[0u64; PADDED_CHUNK_SIZE_USIZE]; PADDED_CHUNK_SIZE_USIZE]; 6];

    for y in 0..PADDED_CHUNK_SIZE_USIZE {
        for z in 0..PADDED_CHUNK_SIZE_USIZE {
            for x in 0..PADDED_CHUNK_SIZE_USIZE {
                let block_x = x as i32 - 1i32;
                let block_y = y as i32 - 1i32;
                let block_z = z as i32 - 1i32;

                let section_x = block_x.rem_euclid(CHUNK_SIZE);
                let section_y = block_y.rem_euclid(CHUNK_SIZE);
                let section_z = block_z.rem_euclid(CHUNK_SIZE);

                assert!(section_x >= 0 && section_x < CHUNK_SIZE);
                assert!(section_y >= 0 && section_y < CHUNK_SIZE);
                assert!(section_z >= 0 && section_z < CHUNK_SIZE);

                let block = section_data.get_by_xyz(block_x, block_y, block_z).unwrap_or_else(|| {
                    if block_y < 0 {
                        return get_block_at_section(&sections.down, section_x, section_y, section_z);
                    } else if block_y > CHUNK_SIZE {
                        return get_block_at_section(&sections.up, section_x, section_y, section_z);
                    }

                    if block_x < 0 {
                        return get_block_at_section(&sections.west, section_x, section_y, section_z);
                    }
                    else if block_x > CHUNK_SIZE {
                        return get_block_at_section(&sections.east, section_x, section_y, section_z);
                    }

                    if block_z < 0 {
                        return get_block_at_section(&sections.south, section_x, section_y, section_z);
                    }
                    else if block_z > CHUNK_SIZE {
                        return get_block_at_section(&sections.north, section_x, section_y, section_z);
                    }

                    Block(0)
                });

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
                        _ => ivec3(x as i32, z as i32, y as i32),     // forward | back
                    };

                    let block = section_data.get_by_xyz(voxel_pos.x, voxel_pos.y, voxel_pos.z).unwrap();
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
        let quads_from_axis = greedy_mesh_binary_plane(plane);

        quads_from_axis.into_iter().for_each(|q| {
            q.append_vertices(&mut vertices, &mut normals, face_dir, axis_pos as i32);
        });
    }

    let indices = generate_indices(vertices.len());
    Some(ChunkSectionMesh::new(vertices, normals, indices))
}

//https://github.com/TanTanDev/binary_greedy_mesher_demo/blob/main/src/utils.rs#L95
fn generate_indices(vertex_count: usize) -> Vec<u32> {
    let quad_count = vertex_count / 4;
    let mut indices = Vec::<u32>::with_capacity(quad_count * 6);
    (0..quad_count).into_iter().for_each(|vert_index| {
        let base = vert_index as u32 * 4u32;
        indices.push(base);
        indices.push(base + 1);
        indices.push(base + 2);
        indices.push(base);
        indices.push(base + 2);
        indices.push(base + 3);
    });
    indices
}
