/// Simple fragment shader for unlit objects
pub const SHADER: &str = r##"#version 300 es

precision mediump float;

out vec4 outColor;

in vec4 v_pos;

void main() {
    outColor = vec4((v_pos.x + 1.) / 2., (v_pos.y + 1.) / 2., 0.0, 1.0);
}
"##;