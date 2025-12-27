use bevy::math::Vec4;
use bevy::mesh::{Mesh, MeshVertexBufferLayout, MeshVertexBufferLayoutRef};
use bevy::pbr::{Material, MaterialExtension, MaterialPipeline, MaterialPipelineKey, MeshPipelineKey};
use bevy::pbr::wireframe::WireframeConfig;
use bevy::prelude::{AlphaMode, Asset, Reflect, Res, TypePath};
use bevy::render::render_resource::{AsBindGroup, PolygonMode, RenderPipelineDescriptor, SpecializedMeshPipelineError};
use bevy::shader::ShaderRef;

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone, Default)]
pub struct ChunkMaterial {
    //#[uniform(0)]
    pub color: Vec4
}

impl Material for ChunkMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/testt.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/testt.wgsl".into()
    }

    fn prepass_vertex_shader() -> ShaderRef {
        "shaders/chunk_prepass.wgsl".into()
    }

    fn prepass_fragment_shader() -> ShaderRef {
        "shaders/chunk_prepass.wgsl".into()
    }
}