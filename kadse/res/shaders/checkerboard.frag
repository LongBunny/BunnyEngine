#version 460

in vec4 col;
in vec3 normal;
in vec2 uv;

out vec4 frag_col;

void main() {

    vec2 check = fract(uv * 30.0);
    float c = (check.x + check.y) / 2.0;
    c = c > 0.5 ? 1 : 0;
    frag_col = vec4(c, c, c, 1.0) * 0.5;
}
