use crate::chunk::Chunk;
use bevy::app::App;
use bevy::asset::RenderAssetUsages;
use bevy::camera::Camera3dDepthLoadOp;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;

#[derive(Default)]
pub struct ChunkRenderPlugin;

impl Plugin for ChunkRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, Self::render_chunks).add_systems(Startup, Self::test_gen_chunks);
    }
}

impl ChunkRenderPlugin {
    fn generate_chunk_mesh(chunk: &Chunk) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD);

        let positions = vec![
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
        ];

        let indices = vec![0, 1, 2];

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_indices(Indices::U16(indices));
        mesh
    }

    pub fn test_gen_chunks(mut commands: Commands) {
        commands.spawn((Transform::from_xyz(0., 0., 0.), Chunk::default()));
        commands.spawn((Camera3d::default(), Transform::from_xyz(0.0, 7.0, 14.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y)));
    }

    pub fn render_chunks(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>, query: Query<(Entity, &Chunk)>) {
        for (entity, chunk) in query {
            let mut chunk_entity = commands.entity(entity);
            let chunk_mesh = meshes.add(Self::generate_chunk_mesh(chunk));
            let chunk_material = materials.add(StandardMaterial::default());
            chunk_entity.insert((Mesh3d(chunk_mesh), MeshMaterial3d(chunk_material)));
        }
    }
}
