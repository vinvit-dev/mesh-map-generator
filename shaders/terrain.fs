#version 450 core

out vec4 final_color;

in float out_color;

void main() {
    final_color = vec4(out_color, out_color + 0.3, out_color, 1.0);
}
