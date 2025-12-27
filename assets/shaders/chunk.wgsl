#import bevy_pbr::{
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::alpha_discard,
}

#ifdef PREPASS_PIPELINE
#import bevy_pbr::{
    prepass_io::{FragmentOutput},
    pbr_deferred_functions::deferred_output,
}
#else
#import bevy_pbr::{
    forward_io::{FragmentOutput},
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing},
}
#endif

#import bevy_pbr::mesh_functions::{get_world_from_local, mesh_position_local_to_clip, mesh_normal_local_to_world}
#import bevy_pbr::pbr_functions::{calculate_view, prepare_world_normal}
#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings
#import bevy_pbr::mesh_bindings::mesh
#import bevy_pbr::pbr_types::pbr_input_new
#import bevy_pbr::prepass_utils
#import bevy_pbr::forward_io::VertexOutput

struct ChunkMaterial {
    color: vec4<f32>,
};
@group(2) @binding(0) var<uniform> chunk_material: ChunkMaterial;

/// -----------------VERTEX------------------

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    //@location(0) vertex_data: u32,
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_normal: vec3<f32>,
    @location(1) world_position: vec4<f32>,
    @location(2) blend_color: vec3<f32>,
    @location(3) instance_index: u32,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;

    let local_position = vec4<f32>(vertex.position, 1.0);
    let world_position = get_world_from_local(vertex.instance_index) * local_position;
    out.clip_position = mesh_position_local_to_clip(
        get_world_from_local(vertex.instance_index),
        vec4<f32>(vertex.position, 1.0),
    );

    out.world_position = world_position;

    out.world_normal = mesh_normal_local_to_world(vertex.normal, vertex.instance_index);

    out.blend_color = chunk_material.color.xyz;
    out.instance_index = vertex.instance_index;

    return out;
}

/// -----------------FRAGMENT------------------

struct FragmentInput {
    @location(0) blend_color: vec4<f32>,
};

@fragment
fn fragment(input: VertexOutput) -> FragmentOutput {
    var pbr_input = pbr_input_new();

    pbr_input.flags = mesh[input.instance_index].flags;

    pbr_input.V = calculate_view(input.world_position, false);
    pbr_input.frag_coord = input.clip_position;
    pbr_input.world_position = input.world_position;

    pbr_input.world_normal = prepare_world_normal(
        input.world_normal,
        false,
        false,
    );
#ifdef LOAD_PREPASS_NORMALS
    pbr_input.N = prepass_utils::prepass_normal(input.clip_position, 0u);
#else
    pbr_input.N = normalize(pbr_input.world_normal);
#endif

    pbr_input.material.base_color = vec4<f32>(input.blend_color, 1.0);
    pbr_input.material.reflectance = 1.0;
    pbr_input.material.perceptual_roughness = 0.5;
    pbr_input.material.metallic = 0.0;

#ifdef PREPASS_PIPELINE
    // in deferred mode we can't modify anything after that, as lighting is run in a separate fullscreen shader.
    let out = deferred_output(input, pbr_input);
#else
    var out: FragmentOutput;
    // apply lighting
    out.color = apply_pbr_lighting(pbr_input);
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);
#endif

    out.color = vec4<f32>(1.0, 0.0,0.0,1.0);

    return out;
   // return vec4<f32>(input.blend_color, 1.0);
    // return vec4<f32>(input.blend_color * input.ambient, 1.0);
    // return vec4<f32>(1.0, 0.0,0.0,1.0);
}