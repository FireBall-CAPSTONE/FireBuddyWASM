/// Simple fragment shader for unlit objects
pub const SHADER: &str = r##"#version 300 es

precision mediump float;

out vec4 outColor;

in vec4 position;
in vec3 normal;
in vec2 texture_coords;

void main() {
    outColor = vec4(0.9, 0.15, 0.05, 1.0);
    // outColor = vec4(1.0, 1.0, 1.0, 1.0);
}
"##;