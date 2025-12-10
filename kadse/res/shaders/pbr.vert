#version 460

layout (location = 0) in vec3 in_pos;
layout (location = 1) in vec3 in_normal;
layout (location = 2) in vec2 in_uv;

uniform mat4 proj_mat;
uniform mat4 view_mat;
uniform mat4 model_mat;

uniform vec4 tint;

out vec4 col;
out vec3 normal;
out vec2 uv;
out vec3 frag_pos;

void main() {
    gl_Position = proj_mat * view_mat * model_mat * vec4(in_pos.xyz, 1.0);
    col = vec4(1.0) * tint;

    mat3 normal_mat = transpose(inverse(mat3(model_mat)));
    normal = normalize(normal_mat * in_normal);

    uv = in_uv;
    frag_pos = vec3(model_mat * vec4(in_pos, 1.0));
}
