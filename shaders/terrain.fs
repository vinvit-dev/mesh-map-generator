#version 450 core

out vec4 final_color;

in vec3 out_color;

void main() {
    final_color = vec4(out_color, 1.0);
}
