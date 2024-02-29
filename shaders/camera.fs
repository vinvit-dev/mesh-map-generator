#version 450 core

uniform sampler2D the_texture;

out vec4 final_color;

in vec4 frag_color;
in vec2 frag_tex;

void main() {
    final_color = texture(the_texture, frag_tex) * frag_color;
}
