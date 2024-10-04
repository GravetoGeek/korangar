struct Matrices {
    view: mat4x4<f32>,
    projection: mat4x4<f32>,
}

struct Constants {
    world: mat4x4<f32>,
    texture_position: vec2<f32>,
    texture_size: vec2<f32>,
    depth_offset: f32,
    curvature: f32,
    mirror: u32,
}

struct Vertex {
    position: vec3<f32>,
    texture_coordinates: vec2<f32>,
    depth_multiplier: f32,
    curvature_multiplier: f32,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) texture_coordinates: vec2<f32>,
    @location(1) depth_offset: f32,
    @location(2) curvature: f32,
}

@group(0) @binding(0) var<uniform> matrices: Matrices;
@group(0) @binding(1) var texture: texture_2d<f32>;
@group(0) @binding(2) var texture_sampler: sampler;

var<push_constant> constants: Constants;
override near_plane: f32;

// Small value to prevent division by zero.
const epsilon: f32 = 1e-7;

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    let vertex = vertex_data(vertex_index);
    var output: VertexOutput;
    output.position = matrices.projection * matrices.view * constants.world * vec4<f32>(vertex.position, 1.0);
    output.texture_coordinates = constants.texture_position + vertex.texture_coordinates * constants.texture_size;

    if (constants.mirror != 0u) {
        output.texture_coordinates.x = 1.0 - output.texture_coordinates.x;
    }

    output.depth_offset = vertex.depth_multiplier;
    output.curvature = vertex.curvature_multiplier;
    return output;
}

@fragment
fn fs_main(
    @builtin(position) position: vec4<f32>,
    @location(0) texture_coordinates: vec2<f32>,
    @location(1) depth_offset: f32,
    @location(2) curvature: f32,
) -> @builtin(frag_depth) f32 {
    let diffuse_color = textureSample(texture, texture_sampler, texture_coordinates);
    if (diffuse_color.a != 1.0) {
        discard;
    }

    let scaled_depth_offset = pow(depth_offset, 2.0) * constants.depth_offset;
    let scaled_curvature_offset = (0.5 - pow(curvature, 2.0)) * constants.curvature;

    let linear_z: f32 = nonLinearToLinear(position.z);
    // We add the offsets in linear view space.
    let adjusted_linear_z: f32 = 2.0 + linear_z - scaled_curvature_offset - scaled_curvature_offset;
    let non_linear_z: f32 = linearToNonLinear(adjusted_linear_z);
    let clamped_depth = clamp(non_linear_z, 0.0, 1.0);

    return position.z;
    // FIX: we don't even use any of the calculated values here! should it not be:
    // return clamped_depth;
}

// Optimized version of the following truth table:
//
// vertex_index  x  y  z  u  v  d  c
// 0            -1  2  1  0  0  1 -1
// 1            -1  0  1  0  1  0 -1
// 2             1  2  1  1  0  1  1
// 3             1  2  1  1  0  1  1
// 4            -1  0  1  0  1  0 -1
// 5             1  0  1  1  1  0  1
//
// (x,y,z) are the vertex position
// (u,v) are the UV coordinates
// (depth) is the depth multiplier
// (curve) is the cuvature multiplier
fn vertex_data(vertex_index: u32) -> Vertex {
    let index = 1u << vertex_index;

    let case0 = i32((index & 0x13u) != 0u);
    let case1 = i32((index & 0x0Du) != 0u);

    let x = f32(1 - 2 * case0);
    let y = f32(2 * case1);
    let z = 1.0;
    let u = f32(1 - case0);
    let v = f32(1 - case1);
    let depth = f32(case1);
    let curve = u * 2.0 - 1.0;

    return Vertex(vec3<f32>(x, y, z), vec2<f32>(u, v), depth, curve);
}

fn linearToNonLinear(linear_depth: f32) -> f32 {
    return near_plane / (linear_depth + epsilon);
}

fn nonLinearToLinear(non_linear_depth: f32) -> f32 {
    return near_plane / (non_linear_depth + epsilon);
}