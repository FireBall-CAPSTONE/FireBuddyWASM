/// Simple shader to test outcolor
pub const SHADER: &str = r##"#version 300 es

precision mediump float;

out vec4 outColor;

in vec4 position;

void main() {
    // outColor = vec4(1.0, 1.0, 1.0, 1.0);
    outColor = vec4((v_pos.x + 1.) / 2., (v_pos.y + 1.) / 2., (v_pos.z + 1.) / 2., 1.0);
}
"##;