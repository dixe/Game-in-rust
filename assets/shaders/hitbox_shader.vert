#version 330 core

layout (location = 0) in vec3 Position;

out VS_OUTPUT {
  vec3 Color;
} OUT;




uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;


void main()
{
    OUT.Color = vec3(1.0, 0.0, 0.0);
    gl_Position =  projection * view * model * vec4(Position, 1.0);
}
