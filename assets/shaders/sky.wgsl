#import bevy_pbr::{
    mesh_view_bindings::view,
    utils::coords_to_viewport_uv,
}
#import bevy_pbr::mesh_view_bindings::globals;
#import bevy_pbr::mesh_functions::{get_world_from_local, mesh_position_local_to_clip}

struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(0) dir: vec3<f32>,
};

@vertex
fn vertex(@location(0) position: vec3<f32>, @builtin(instance_index) instance: u32) -> VertexOutput {
    var out: VertexOutput;
    out.dir = normalize(position);
    out.pos = mesh_position_local_to_clip(get_world_from_local(instance), vec4<f32>(position, 1.0));
    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let dir = normalize(in.dir);
    let horizon = abs(dir.y);

    let horizonColor = vec3<f32>(0.8, 0.9, 1.0);
    let topColor = vec3<f32>(0.2, 0.4, 0.8);
    let bottomColor = vec3<f32>(0.3, 0.3, 0.4);

    var skyColor: vec3<f32>;
    if (dir.y > 0.0) {
        skyColor = mix(horizonColor, topColor, smoothstep(0.0, 1.0, dir.y));
    } else {
        skyColor = mix(horizonColor, bottomColor, smoothstep(0.0, 1.0, -dir.y));
    }

    return vec4<f32>(skyColor, 1.0);
}