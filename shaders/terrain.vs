#version 450 core

layout (location = 0) in vec3 pos;
layout (location = 1) in vec3 color;

uniform mat4 view;
uniform mat4 model;
uniform mat4 projection;

out vec3 out_color;

void main() {
    float scale = 0.2;
    gl_Position = projection * model * view * vec4(pos.x * scale, pos.y * 7 - 5, pos.z * scale, 1.0);
    out_color = color;
}
