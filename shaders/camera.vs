#version 450 core

layout (location = 0) in vec3 pos;
layout (location = 1) in vec3 color;
layout (location = 2) in vec2 tex;

uniform mat4 model;
uniform mat4 projection;
uniform mat4 view;

out vec4 frag_color;
out vec2 frag_tex;

void main() {
    gl_Position = projection * view * model * vec4(pos, 1.0);
    frag_color = vec4(color, 1.0);
    frag_tex = tex; 
}