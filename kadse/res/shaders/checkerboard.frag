#version 460

in vec4 col;
in vec3 normal;
in vec2 uv;
in vec3 frag_pos;

uniform vec3 camera_pos;

out vec4 frag_col;

vec3 color_a = vec3(181.0 / 255.0, 136.0 / 255.0, 99.0 / 255.0); // dark
vec3 color_b = vec3(240.0 / 255.0, 217.0 / 255.0, 181.0 / 255.0); // light
float tiles = 32.0;                 // number of tiles across one axis

void main() {
    // Scale UVs by tile count
    vec2 s_uv = uv * tiles;

    // Get integer tile coordinates
    float cx = floor(s_uv.x);
    float cy = floor(s_uv.y);

    // Sum and mod 2 to alternate
    float checker = mod(cx + cy, 2.0);

    // If checker == 0 → colorA, else colorB
    vec3 color = mix(color_a, color_b, checker);

    frag_col = vec4(color, 1.0);
}
