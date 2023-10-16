#version 330 core

out vec4 FragColour;

in vec3 my_colour;

void main() {
    FragColour = vec4(my_colour, 1.0);
}