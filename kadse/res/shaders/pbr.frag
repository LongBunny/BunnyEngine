#version 460

in vec4 frag_col;
in vec2 frag_uv;
in vec3 frag_pos;
in mat3 frag_tbn;

layout(binding = 0) uniform sampler2D tex_albedo;
layout(binding = 1) uniform sampler2D tex_normal;
uniform vec3 camera_pos;
uniform float texture_scale = 1.0;
uniform float specular_intensity = 1.0;

layout(std140, binding = 2) uniform MaterialUBO {
    vec4 albedo_color;
    vec4 emissive_color;

    float metallic;
    float roughness;
    float normal_scale;

    int albedo_has_texture;
    int normal_has_texture;
    int emissive_has_texture;
    int metallic_has_texture;
    int roughness_has_texture;
};

out vec4 out_col;

void main() {
    vec2 uv = fract(frag_uv * texture_scale);
    vec3 directional_light = vec3(1.0, -1.0, 1.0);
    vec3 light_dir = normalize(-directional_light);
    vec3 light_color = vec3(1.0, 1.0, 1.0);
    vec3 ambient_color = vec3(1.0, 1.0, 1.0);
    float ambient_strength = 0.025;
    vec3 specular_color = vec3(1.0, 1.0, 1.0);

    vec3 surface_normal = normalize(frag_tbn[2]);
    if (normal_has_texture == 1) {
        vec3 n = texture(tex_normal, uv).xyz;
        n = n * 2.0 - 1.0;
        n.xy *= normal_scale;
        surface_normal = normalize(frag_tbn * n);
    }

    float specular_shinyness = 48f;

    vec3 color = albedo_has_texture == 1
        ? (texture(tex_albedo, uv) * frag_col).xyz
        : (albedo_color * frag_col).xyz;

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
//    out_col = vec4(normalize(surface_normal) * 0.5 + 0.5, 1.0);
}
