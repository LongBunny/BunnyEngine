#version 460

layout (location = 0) in vec3 in_pos;
layout (location = 1) in vec3 in_normal;
layout (location = 2) in vec2 in_uv;
layout (location = 3) in vec3 in_tangent;

uniform mat4 proj_mat;
uniform mat4 view_mat;
uniform mat4 model_mat;

out vec4 frag_col;
out vec2 frag_uv;
out vec3 frag_pos;
out mat3 frag_tbn;

void main() {
    gl_Position = proj_mat * view_mat * model_mat * vec4(in_pos.xyz, 1.0);
    frag_col = vec4(1.0);

    mat3 normal_mat = transpose(inverse(mat3(model_mat)));
    vec3 N = normalize(normal_mat * in_normal);
    vec3 T = normalize(normal_mat * in_tangent);
    // Re-orthogonalize tangent
    T = normalize(T - dot(T, N) * N);
    vec3 B = cross(N, T);
    frag_tbn = mat3(T, B, N);

    frag_uv = in_uv;
    frag_pos = vec3(model_mat * vec4(in_pos, 1.0));
}
