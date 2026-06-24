use bevy::color::palettes::css::{BLACK, WHITE};
use bevy::mesh::MeshVertexBufferLayoutRef;
use bevy::pbr::{MaterialExtension, MaterialExtensionKey, MaterialExtensionPipeline};
use bevy::prelude::{App, Asset, Component, Gradient, LinearGradient, Plugin, Reflect, Resource};
use bevy::render::render_resource::{AsBindGroup, CompareFunction, RenderPipelineDescriptor, SpecializedMeshPipelineError};

#[derive(Copy, Clone, Default, Component)]
pub struct Skybox;

#[derive(Clone, Resource)]
pub struct SkyboxSettings {
    pub gradient: Gradient,
}

#[derive(Clone, AsBindGroup, Asset, Reflect)]
pub struct SkyboxShader {}

impl MaterialExtension for SkyboxShader {
    fn specialize(
        _pipeline: &MaterialExtensionPipeline,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayoutRef,
        _key: MaterialExtensionKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        if let Some(depth_stencil) = &mut descriptor.depth_stencil {
            depth_stencil.depth_write_enabled = Some(false);
            depth_stencil.depth_compare = Some(CompareFunction::Always);
        }

        Ok(())
    }
}

pub struct SkyboxPlugin;

impl Plugin for SkyboxPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SkyboxSettings {
            gradient: Gradient::Linear(LinearGradient::to_right(vec![WHITE.into(), BLACK.into()])),
        });
    }
}
