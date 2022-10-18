/// Standard 3D vertex shader used by all objects
pub const SHADER: &str = r##"#version 300 es

layout(location = 0) in vec3 vertex_position;
layout(location = 1) in vec3 vertex_normal;
layout(location = 2) in vec2 vertex_uv_coords;

layout(location = 3) uniform mat4 projection_matrix; // projection * view * transform
//uniform mat4 view_matrix; // inverse camera transform * object transform
//uniform mat4 transform_matrix; // object transform
//uniform mat4 normal_matrix; // transpose(inverse(view))

void main() {
    gl_Position = projection_matrix * vec4(vertex_position, 1.0);
    position = vec3(view_matrix * vec4(vertex_position, 1.0));
    // normal = mat3(normal_matrix) * vertex_normal;
    // texture_coords = vertex_uv_coords;
}
"##;