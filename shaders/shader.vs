#version 330 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 colour;
layout (location = 2) in vec2 texture;

out vec3 my_colour;
out vec2 texture_coordinate;

void main() {
    gl_Position = vec4(position, 1.0);
    my_colour = colour;
    texture_coordinate = vec2(texture.x, texture.y);
}