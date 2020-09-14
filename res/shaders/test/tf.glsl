#version 330 core

in vec2 color_pos;
out vec4 Color;

void main()
{
    Color = vec4(color_pos.x + 0.5 * 0.5, color_pos.y + 0.5 / 2, 1.0, 1.0);
}