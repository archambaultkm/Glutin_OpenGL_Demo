#version 330 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 colour;

out vec3 my_colour;

uniform float x_offset;

void main() {
    gl_Position = vec4(position.x + x_offset, position.y, position.z, 1.0);
    my_colour = colour;
}