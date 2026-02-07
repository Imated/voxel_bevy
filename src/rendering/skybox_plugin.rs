use crate::rendering::skybox_material::SkyboxMaterial;
use bevy::app::{App, Startup, Update};
use bevy::asset::Assets;
use bevy::camera::Camera3d;
use bevy::mesh::{Mesh, Mesh3d, Meshable};
use bevy::pbr::MeshMaterial3d;
use bevy::prelude::{Commands, Component, Cuboid, Name, Plugin, ResMut};
use bevy::prelude::{MeshBuilder, Query, Transform, With, Without};

#[derive(Component, Copy, Clone, Debug, Default)]
pub struct Skybox;

pub struct SkyboxPlugin;

impl Plugin for SkyboxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::create_skybox)
            .add_systems(Update, Self::update_skybox).init_resource::<Assets<SkyboxMaterial>>();
    }
}

impl SkyboxPlugin {
    pub fn create_skybox(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<SkyboxMaterial>>,
    ) {
        let mesh = Cuboid::new(1000.0, 1000.0, 1000.0).mesh().build();

        commands.spawn((
            Name::new("Skybox"),
            Transform::from_xyz(0.0, 0.0, 0.0),
            Mesh3d(meshes.add(mesh)),
            MeshMaterial3d(materials.add(SkyboxMaterial::default())),
        ));
    }

    pub fn update_skybox(
        camera_query: Query<&Transform, (With<Skybox>, With<Camera3d>)>,
        mut sky_query: Query<&mut Transform, (Without<Camera3d>, With<MeshMaterial3d<SkyboxMaterial>>)>,
    ) {
        for cam in camera_query.iter() {
            for mut skybox in sky_query.iter_mut() {
                skybox.translation = cam.translation;
            }
        }
    }
}
