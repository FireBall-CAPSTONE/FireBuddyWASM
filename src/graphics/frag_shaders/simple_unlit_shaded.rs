pub const SHADER: &str = r##"#version 300 es

precision mediump float;

uniform sampler2D tex;

out vec4 outColor;

in vec4 position;
in vec3 normal;
in vec2 texture_coords;

vec3 lightDir = normalize(vec3(1.0, 1.0, 1.0));

void main() {
    // outColor = vec4(1.0, 1.0, 1.0, 1.0);
    // outColor = vec4((position.x + 1.) / 2., (position.y + 1.) / 2., (position.z + 1.) / 2., 1.0);
    // outColor = texture(tex, texture_coords);
    // outColor = vec4(texture_coords.x, texture_coords.y, 0.0, 1.0);
    vec4 col = texture(tex, -texture_coords);
    float fres = 1.0 - dot(normal, vec3(0.0, 0.0, 1.0)); // TODO: make relative to the inverse of the camera forward
    float atmosphere = fres * fres * fres * fres * 1.85;
    float shadow = (dot(normal, lightDir) + .25) * 0.85;
    float intensity = min(shadow + atmosphere, 1.0);

    outColor = vec4(col.xyz * intensity, 1.0);
}
"##;