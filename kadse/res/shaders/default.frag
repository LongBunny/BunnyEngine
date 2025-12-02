#version 460

float ambient_light = 0.1;
vec3 light_dir = vec3(-0.2, 1.0, 0.0);
vec3 light_color = vec3(1.0, 1.0, 1.0);

in vec4 col;
in vec3 normal;
in vec2 uv;

uniform sampler2D u_texture;

out vec4 frag_col;

void main() {
    vec3 color = (texture(u_texture, uv) * col).xyz;
    vec3 norm = normalize(normal);
    float diff = max(dot(normal, light_dir), 0.0);
    vec3 diffuse = diff * light_color;
    vec3 result = (ambient_light + diffuse) * color;
    frag_col = vec4(result, 1.0);
}
