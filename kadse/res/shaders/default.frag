#version 460

in vec4 frag_col;
in vec3 frag_normal;
in vec2 frag_uv;
in vec3 frag_pos;

uniform sampler2D u_texture;
uniform vec3 camera_pos;
uniform float texture_scale = 1.0;
uniform float specular_intensity = 1.0;

out vec4 out_col;

void main() {
    vec2 uv = fract(frag_uv * texture_scale);
    vec3 directional_light = vec3(1.0, -1.0, 1.0);
    vec3 light_dir = normalize(-directional_light);
    vec3 light_color = vec3(1.0, 1.0, 1.0);
    vec3 ambient_color = vec3(1.0, 1.0, 1.0);
    float ambient_strength = 0.025;
    vec3 specular_color = vec3(1.0, 1.0, 1.0);
    vec3 surface_normal = normalize(frag_normal);
    float specular_shinyness = 256f;

    vec3 color = (texture(u_texture, uv) * frag_col).xyz;

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
    vec3 specular = specular_color * specular_strength * specular_intensity;

    // combine
    vec3 result = ambient + diffuse + specular;
    out_col = vec4(result, 1.0);
}
