#version 460

in vec4 col;
in vec3 normal;
in vec2 uv;
in vec3 frag_pos;

uniform sampler2D u_texture;
uniform vec3 camera_pos;

out vec4 frag_col;

void main() {
    vec3 directional_light = vec3(1.0, -1.0, 1.0);
    vec3 light_dir = normalize(-directional_light);
    vec3 light_color = vec3(1.0, 1.0, 1.0);
    vec3 ambient_color = vec3(1.0, 1.0, 1.0);
    float ambient_strength = 0.025;
    vec3 specular_color = vec3(1.0, 1.0, 1.0);
    vec3 surface_normal = normalize(normal);
    float specular_shinyness = 32f;

    vec3 color = (texture(u_texture, uv) * col).xyz;

    // ambient
    vec3 ambient = color * ambient_strength * ambient_color;

    // diffuse
    float diffuse_factor = max(dot(light_dir, surface_normal), 0.0);
    vec3 diffuse = color * diffuse_factor * light_color;

    // specular
    vec3 view_dir = normalize(camera_pos - frag_pos);
    vec3 halfway_dir = normalize(light_dir + view_dir);
    float specular_strength = pow(
        max(dot(surface_normal, halfway_dir), 0.0),
        specular_shinyness
    );
    vec3 specular = specular_color * specular_strength;

    // combine
    vec3 result = ambient + diffuse + specular;
    frag_col = vec4(result, 1.0);
}
