#version 460

layout (location = 0) in vec3 in_pos;
layout (location = 1) in vec3 in_normal;
layout (location = 2) in vec2 in_uv;

uniform mat4 pvm;
uniform vec4 tint;

out vec4 col;
out vec3 normal;
out vec2 uv;

void main() {
    gl_Position = pvm * vec4(in_pos.xyz, 1.0);
    col = vec4(1.0) * tint;
    normal = in_normal;
    uv = in_uv;
}
