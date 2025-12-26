#version 460 core
out vec4 out_color;

in vec2 frag_uv;

layout(binding = 0) uniform sampler2D screen_texture;

void main()
{
    out_color = texture(screen_texture, frag_uv);
}