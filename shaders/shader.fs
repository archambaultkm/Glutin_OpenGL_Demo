#version 330 core

out vec4 FragColour;

in vec3 my_colour;
in vec2 texture_coordinate;

//texture sampler
uniform sampler2D texture1;
uniform sampler2D texture2;

void main() {
    FragColour = mix(texture(texture1, texture_coordinate), texture(texture2, texture_coordinate), 0.2); // 0.2 returns 80% first input colour, 20% the second.
}