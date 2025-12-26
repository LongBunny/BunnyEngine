#version 460 core
out vec4 out_color;

in vec2 frag_uv;

layout(binding = 0) uniform sampler2D screen_texture;

float exposure = 0.8;

// Narkowicz 2015, "ACES Filmic Tone Mapping Curve"
vec3 aces(vec3 x) {
    const float a = 2.51;
    const float b = 0.03;
    const float c = 2.43;
    const float d = 0.59;
    const float e = 0.14;
    return clamp((x * (a * x + b)) / (x * (c * x + d) + e), 0.0, 1.0);
}

float aces(float x) {
    const float a = 2.51;
    const float b = 0.03;
    const float c = 2.43;
    const float d = 0.59;
    const float e = 0.14;
    return clamp((x * (a * x + b)) / (x * (c * x + d) + e), 0.0, 1.0);
}



void main()
{
    vec3 hdr = texture(screen_texture, frag_uv).rgb;

    // Exposure (linear)
    hdr *= exposure;

    // ACES tone mapping
    vec3 mapped = aces(hdr);

    // Gamma correction (linear → sRGB)
    mapped = pow(mapped, vec3(1.0 / 2.2));

    out_color = vec4(mapped, 1.0);
}