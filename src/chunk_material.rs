use bevy::asset::Asset;
use bevy::math::Vec4;
use bevy::mesh::{MeshVertexAttribute, MeshVertexBufferLayoutRef, VertexFormat};
use bevy::pbr::{Material, MaterialPipeline, MaterialPipelineKey};
use bevy::reflect::Reflect;
use bevy::render::render_resource::{AsBindGroup, RenderPipelineDescriptor, ShaderType, SpecializedMeshPipelineError};
use bevy::shader::ShaderRef;

#[derive(ShaderType, Clone, Default, Reflect, Debug)]
pub struct MaterialProperties {
    pub base_color: Vec4,
    pub emissive_color_intensity: Vec4,
    pub metallic_roughness_tbd_tbd: Vec4,
}

pub const ATTRIBUTE_VOXEL: MeshVertexAttribute = MeshVertexAttribute::new("Voxel", 988540919, VertexFormat::Uint32);

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone, Default)]
pub struct ChunkMaterial {
    #[uniform(0)]
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

    fn specialize(
        _pipeline: &MaterialPipeline,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayoutRef,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        let vertex_layout = layout.0.get_layout(&[ATTRIBUTE_VOXEL.at_shader_location(0)])?;
        descriptor.vertex.buffers = vec![vertex_layout];
        Ok(())
    }
}
