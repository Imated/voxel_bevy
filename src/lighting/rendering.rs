use crate::chunk_material::{ChunkMaterial, MaterialProperties};
use crate::lighting::voxel_sunlight::{GlobalLightingUniform, VoxelSunlight};
use crate::utils::WithPadding;
use bevy::app::{App, Startup, Update};
use bevy::asset::{Assets, Handle};
use bevy::color::ColorToComponents;
use bevy::math::{vec4, Vec4};
use bevy::prelude::{Commands, GlobalTransform, Plugin, Query, ResMut, Resource, With};
use bevy::render::storage::ShaderStorageBuffer;

pub struct CustomRenderPlugin;

impl Plugin for CustomRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_light_data);
            //.add_systems(Update, update_light_data);
    }
}

#[derive(Resource)]
pub struct GlobalChunkMaterial(pub(crate) Handle<ChunkMaterial>);

fn setup_light_data(mut commands: Commands, mut buffers: ResMut<Assets<ShaderStorageBuffer>>, mut materials: ResMut<Assets<ChunkMaterial>>) {
    let global_light_data = buffers.add(ShaderStorageBuffer::from(GlobalLightingUniform::default()));
    let material = materials.add(ChunkMaterial {
        global_light_data,
        material_properties: MaterialProperties {
            base_color: vec4(0.3, 0.5, 0.3, 1.0),
            emissive_color_intensity: Vec4::ZERO,
            metallic_roughness_tbd_tbd: vec4(0.2, 0.5, 0.0, 0.0),
        },
    });

    commands.insert_resource(GlobalChunkMaterial(material));
}

fn update_light_data(sun_query: Query<(&GlobalTransform, &VoxelSunlight), With<VoxelSunlight>>,
                     mut buffers: ResMut<Assets<ShaderStorageBuffer>>,
                     chunk_material: ResMut<GlobalChunkMaterial>,
                     mut materials: ResMut<Assets<ChunkMaterial>>) {
    let material = materials.get_mut(&chunk_material.0).unwrap();
    let buffer = buffers.get_mut(&material.global_light_data).unwrap();
    let (sun_transform, sun) = sun_query.iter().next().unwrap();
    let mut sun_color = sun.color.to_srgba().to_vec4();
    sun_color.w = sun.illuminance;

    buffer.set_data(GlobalLightingUniform {
        color_intensity: sun_color,
        ambient_color_intensity: vec4(0.05, 0.05, 0.05, 1.0),
        sun_direction: sun_transform.down().to_vec4(),
    })
}
