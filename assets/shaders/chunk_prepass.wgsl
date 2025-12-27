#import bevy_pbr::{
    prepass_io::{FragmentOutput},
}
#import bevy_pbr::mesh_functions::{mesh_normal_local_to_world, get_world_from_local, mesh_position_local_to_clip}
#import bevy_render::instance_index::{get_instance_index}

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
}

@vertex
fn prepass_vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    let local_position = vec4<f32>(vertex.position, 1.0);
    var model = get_world_from_local(vertex.instance_index);
    let world_position = model * local_position;

    out.clip_position = mesh_position_local_to_clip(model, local_position);
    out.world_normal = mesh_normal_local_to_world(vertex.normal, vertex.instance_index);

    return out;
}

/// -----------------FRAGMENT------------------

@fragment
fn fragment(in: VertexOutput) -> FragmentOutput {
    var out: FragmentOutput;

    out.frag_depth = in.clip_position.z;
    out.normal = vec4(in.world_normal * 0.5 + vec3(0.5), 1.0);

    return out;
}