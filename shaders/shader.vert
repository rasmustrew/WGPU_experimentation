// shader.vert
#version 450

layout(location=0) in vec3 a_position;
// Changed
layout(location=1) in vec2 a_tex_coords;

// Changed
layout(location=0) out vec2 v_tex_coords;

// NEW!
layout(set=1, binding=0) // 1.
uniform Uniforms {
    mat4 u_model_view_proj; // 2.
};

void main() {
    // Changed
    v_tex_coords = a_tex_coords;
    gl_Position = u_model_view_proj * vec4(a_position, 1.0);
}
