use bevy::asset::Handle;
use bevy::math::Vec4;
use bevy::mesh::{Mesh, MeshVertexBufferLayout, MeshVertexBufferLayoutRef};
use bevy::pbr::wireframe::WireframeConfig;
use bevy::pbr::{
    Material, MaterialExtension, MaterialPipeline, MaterialPipelineKey, MeshPipelineKey,
};
use bevy::prelude::{AlphaMode, Asset, Reflect, Res, TypePath};
use bevy::render::render_resource::{AsBindGroup, PolygonMode, RenderPipelineDescriptor, ShaderType, SpecializedMeshPipelineError};
use bevy::render::storage::ShaderStorageBuffer;
use bevy::shader::ShaderRef;

#[derive(ShaderType, Clone, Default, Reflect, Debug)]
pub struct MaterialProperties {
    pub base_color: Vec4,
    pub emissive_color_intensity: Vec4,
    pub metallic_roughness_tbd_tbd: Vec4,
}

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone, Default)]
pub struct ChunkMaterial {
    #[storage(0, read_only)]
    pub global_light_data: Handle<ShaderStorageBuffer>,

    #[uniform(1)]
    pub material_properties: MaterialProperties,
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
