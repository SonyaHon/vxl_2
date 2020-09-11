#version 330 core

layout (location = 0) in vec3 Position;

out vec2 color_pos;

void main()
{
    gl_Position = vec4(Position, 1.0);
    color_pos = Position.xy;
}