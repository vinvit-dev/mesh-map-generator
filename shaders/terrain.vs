#version 450 core

layout (location = 0) in vec3 pos;
layout (location = 1) in vec3 color;

uniform mat4 view;
uniform mat4 model;
uniform mat4 projection;

out vec3 out_color;

void main() {
    gl_Position = projection * model * view * vec4(pos.x / 2, pos.y * 5, pos.z / 2, 1.0);
    out_color = color;
}
