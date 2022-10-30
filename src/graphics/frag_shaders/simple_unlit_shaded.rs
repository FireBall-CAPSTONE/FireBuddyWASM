pub const SHADER: &str = r##"#version 300 es

precision mediump float;

uniform sampler2D tex;

out vec4 outColor;

in vec4 position;
in vec2 texture_coords;

void main() {
    // outColor = vec4(1.0, 1.0, 1.0, 1.0);
    // outColor = vec4((position.x + 1.) / 2., (position.y + 1.) / 2., (position.z + 1.) / 2., 1.0);
    // outColor = texture(tex, texture_coords);
    // outColor = vec4(texture_coords.x, texture_coords.y, 0.0, 1.0);
    vec4 col = texture(tex, vec2(texture_coords.x, -texture_coords.y));
    outColor = col;
}
"##;