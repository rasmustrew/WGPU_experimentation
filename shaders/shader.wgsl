struct VertexInput {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] tex_coords: vec2<f32>;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] tex_coords: vec2<f32>;
};

[[block]] // 1.
struct Uniforms {
    mvp: mat4x4<f32>;
};
[[group(1), binding(0)]] // 2.
var<uniform> uniforms: Uniforms;


[[stage(vertex)]]
fn main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    out.clip_position = uniforms.mvp * vec4<f32>(model.position, 1.0);
    return out;
}


[[group(0), binding(0)]]
var t_diffuse: texture_2d<f32>;
[[group(0), binding(1)]]
var s_diffuse: sampler;

[[stage(fragment)]]
fn main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
     return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}

