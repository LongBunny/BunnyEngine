#version 460

#define PI 3.1415926535

in vec4 frag_col;
in vec2 frag_uv;
in vec3 frag_pos;
in mat3 frag_tbn;

layout(binding = 0) uniform sampler2D tex_albedo;
layout(binding = 1) uniform sampler2D tex_normal;
layout(binding = 2) uniform sampler2D tex_roughness;
layout(binding = 3) uniform sampler2D tex_metallic;

uniform vec3 camera_pos;
uniform vec2 texture_scale = vec2(1.0);

layout(std140, binding = 2) uniform MaterialUBO {
    vec4 albedo_color;
    vec4 emissive_color;

    float metallic_value;
    float roughness_value;
    float normal_scale_value;

    int albedo_has_texture;
    int normal_has_texture;
    int emissive_has_texture;
    int metallic_has_texture;
    int roughness_has_texture;
};

out vec4 out_col;


// midday sun
//vec3 directional_light = normalize(vec3(0.3, -1.0, 0.2));
//vec3 light_color = vec3(20.0);

// late afternoon sun
//vec3 directional_light = normalize(vec3(-0.8, -0.6, 0.2));
//vec3 light_color = vec3(15.0);

// rim light
vec3 directional_light = normalize(vec3(-1.0, -0.4, 0.0));
vec3 light_color = vec3(10.0);


vec3 specular_color = vec3(1.0);

vec3 fresnel_schlick(float cos_theta, vec3 F0) {
    return F0 + (1.0 - F0) * pow(1.0 - cos_theta, 5.0);
}

float distribution_ggx(vec3 N, vec3 H, float roughness) {
    float a = roughness * roughness;
    float a2 = a * a;
    float N_dot_H = max(dot(N, H), 0.0);
    float N_dot_H2 = N_dot_H * N_dot_H;

    float denom = (N_dot_H2 * (a2 - 1.0) + 1.0);
    return a2 / (PI * denom * denom);
}

float geometry_schlick_ggx(float N_dot_V, float roughness) {
    float r = roughness + 1.0;
    float k = (r * r) / 8.0;
    return N_dot_V / (N_dot_V * (1.0 - k) + k);
}

float geometry_smith(vec3 N, vec3 V, vec3 L, float roughness) {
    float N_dot_V = max(dot(N, V), 0.0);
    float N_dot_L = max(dot(N, L), 0.0);
    float ggx_V = geometry_schlick_ggx(N_dot_V, roughness);
    float ggx_L = geometry_schlick_ggx(N_dot_L, roughness);
    return ggx_V * ggx_L;
}

void main() {
    vec2 uv = fract(frag_uv * texture_scale);

    // base values (either from texture or uniform)
    vec3 albedo = albedo_has_texture == 1
    ? (texture(tex_albedo, uv) * frag_col).xyz
    : (albedo_color * frag_col).xyz;
    albedo = clamp(albedo, 0.0, 0.8);

    vec3 N = normalize(frag_tbn[2]);
    if (normal_has_texture == 1) {
        vec3 n = texture(tex_normal, uv).xyz;
        n = n * 2.0 - 1.0;
        n.xy *= normal_scale_value;
        N = normalize(frag_tbn * n);
    }

    float roughness = roughness_has_texture == 1
    ? (texture(tex_roughness, uv)).x
    : roughness_value;
    roughness = clamp(roughness, 0.04, 1.0);

    float metallic = metallic_has_texture == 1
    ? (texture(tex_metallic, uv)).x
    : metallic_value;
    metallic  = clamp(metallic, 0.0, 1.0);


    vec3 V = normalize(camera_pos - frag_pos); // view direction
    vec3 L = normalize(-directional_light); // light direction
    vec3 H = normalize(V + L); // half way vector

    vec3 F0 = mix(vec3(0.04), albedo, metallic);

    vec3 F = fresnel_schlick(max(dot(H, V), 0.0), F0);
    float D = distribution_ggx(N, H, roughness);
    float G = geometry_smith(N, V, L, roughness);

    float N_dot_L = max(dot(N, L), 0.0);
    float N_dot_V = max(dot(N, V), 0.0);

    vec3 specular = (D * G * F) / (4.0 * N_dot_L * N_dot_V + 0.0001);

    vec3 kD = (1.0 - F) * (1.0 - metallic);
    vec3 diffuse = kD * albedo / PI;

    vec3 radiance = light_color;
    vec3 Lo = (diffuse + specular) * radiance * N_dot_L;

    // fake IBL
    // diffuse ambient
    vec3 sky_color    = vec3(0.25, 0.3, 0.4);
    vec3 ground_color = vec3(0.05, 0.04, 0.03);
    float hemi = N.y * 0.5 + 0.5;
    vec3 ambient_diffuse =
    mix(ground_color, sky_color, hemi) * albedo * (1.0 - metallic);

    // specular ambient
    vec3 F_env = fresnel_schlick(max(dot(N, V), 0.0), F0);
    vec3 ambient_specular =
    vec3(0.5) * F_env * (1.0 - roughness);

    // final
    out_col = vec4(Lo + ambient_diffuse + ambient_specular, 1.0);
}
