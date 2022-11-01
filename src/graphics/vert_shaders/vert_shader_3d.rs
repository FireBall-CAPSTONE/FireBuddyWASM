/// Standard 3D vertex shader used by all objects
pub const SHADER: &str = r##"#version 300 es

in vec3 vertex_position;
in vec3 vertex_normal;
in vec2 vertex_uv_coords;

uniform mat4 projection_matrix; // projection * view * transform
uniform mat4 view_matrix; // inverse camera transform * object transform
uniform mat4 transform_matrix; // object transform
//uniform mat4 normal_matrix; // transpose(inverse(view))

out vec3 position;
out vec2 texture_coords;
out vec3 normal;

void main() {
    mat4 normal_matrix = transpose(inverse(view_matrix * transform_matrix));
    gl_Position = projection_matrix * view_matrix * transform_matrix * vec4(vertex_position, 1.0);
    // position = vec3(view_matrix * vec4(vertex_position, 1.0));
    normal = mat3(normal_matrix) * vertex_normal;
    texture_coords = vertex_uv_coords;
}
"##;