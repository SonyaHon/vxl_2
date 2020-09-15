#version 330 core

in vec2 color_pos;
out vec4 Color;

uniform sampler2D texSampler;

void main()
{
    Color = texture(texSampler, color_pos);
}