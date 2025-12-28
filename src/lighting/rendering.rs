use crate::chunk_material::{ChunkMaterial, MaterialProperties};
use bevy::app::{App, Startup};
use bevy::asset::{Assets, Handle};
use bevy::math::{Vec4, vec4};
use bevy::prelude::{Commands, Plugin, ResMut, Resource};

pub struct CustomRenderPlugin;

impl Plugin for CustomRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_light_data);
        //.add_systems(Update, update_light_data);
    }
}

#[derive(Resource)]
pub struct GlobalChunkMaterial(pub(crate) Handle<ChunkMaterial>);

fn setup_light_data(mut commands: Commands, mut materials: ResMut<Assets<ChunkMaterial>>) {
    let material = materials.add(ChunkMaterial {
        material_properties: MaterialProperties {
            base_color: vec4(0.3, 0.5, 0.3, 1.0),
            emissive_color_intensity: Vec4::ZERO,
            metallic_roughness_tbd_tbd: vec4(0.2, 0.5, 0.0, 0.0),
        },
    });

    commands.insert_resource(GlobalChunkMaterial(material));
}
