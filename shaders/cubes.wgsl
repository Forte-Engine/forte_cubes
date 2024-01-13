#import ./light.wgsl as Lights

struct CameraUniform {
    view_pos: vec4<f32>,
    view_proj: mat4x4<f32>,
};
@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) normal: vec3<f32>
}

struct InstanceInput {
    @location(5) model_matrix_0: vec4<f32>,
    @location(6) model_matrix_1: vec4<f32>,
    @location(7) model_matrix_2: vec4<f32>,
    @location(8) model_matrix_3: vec4<f32>,
    @location(9) normal_matrix_0: vec3<f32>,
    @location(10) normal_matrix_1: vec3<f32>,
    @location(11) normal_matrix_2: vec3<f32>,
    @location(12) texture_splits_1: vec4<f32>,  // x-start, x1, x-mid, x2
    @location(13) texture_splits_2: vec4<f32>  // x-end, y-start, y-mid, y-end
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) world_position: vec3<f32>,
}

@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput
) -> VertexOutput {
    // create model matrix
    let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );

    // used to rotate the normals
    let normal_matrix = mat3x3<f32>(
        instance.normal_matrix_0,
        instance.normal_matrix_1,
        instance.normal_matrix_2,
    );

    // create texture coords
    var tex_coords = vec2<f32>(0.0, 0.0);

    if (model.tex_coords.x == 0.0) { tex_coords.x = instance.texture_splits_1.x; }
    else if (model.tex_coords.x == 0.25) { tex_coords.x = instance.texture_splits_1.y; }
    else if (model.tex_coords.x == 0.50) { tex_coords.x = instance.texture_splits_1.z; }
    else if (model.tex_coords.x == 0.75) { tex_coords.x = instance.texture_splits_1.w; } 
    else if (model.tex_coords.x == 1.00) { tex_coords.x = instance.texture_splits_2.x; }

    if (model.tex_coords.y == 0.0) { tex_coords.y = instance.texture_splits_2.y; }
    else if (model.tex_coords.y == 0.5) { tex_coords.y = instance.texture_splits_2.z; }
    else if (model.tex_coords.y == 1.0) { tex_coords.y = instance.texture_splits_2.w; }

    // finalize output
    var out: VertexOutput;
    out.tex_coords = tex_coords;
    out.world_normal = normal_matrix * model.normal;
    var world_position = model_matrix * vec4<f32>(model.position, 1.0);
    out.world_position = world_position.xyz;
    out.clip_position = camera.view_proj * world_position;
    return out;
}

// Fragment shader

@group(1) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(1) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let diffuse = textureSample(t_diffuse, s_diffuse, in.tex_coords);
    let color = diffuse.xyz * Lights::calculate_lights(camera.view_pos.xyz, in.world_position, in.world_normal);
    return vec4<f32>(color, diffuse.a);
}
