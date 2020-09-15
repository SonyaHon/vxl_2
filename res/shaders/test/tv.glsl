#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec2 UV;

out vec2 color_pos;

uniform mat4 transform_mat;
uniform mat4 projection_mat;
uniform mat4 view_mat;

void main()
{
    gl_Position = projection_mat * view_mat * transform_mat * vec4(Position, 1.0) ;
    color_pos = UV;
}