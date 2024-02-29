#version 450 core

layout (location = 0) in vec3 pos;

uniform mat4 view;
uniform mat4 model;
uniform mat4 projection;

out float out_color;

void main() {
    gl_Position = projection * model * view * vec4(pos.x, pos.y, pos.z, 1.0);
    out_color = pos.y - 0.5;
}
