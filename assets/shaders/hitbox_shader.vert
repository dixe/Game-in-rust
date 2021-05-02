#version 330 core

layout (location = 0) in vec3 Position;

out VS_OUTPUT {
  vec3 Color;
} OUT;


uniform vec3 color;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;


void main()
{
  OUT.Color = color;
  gl_Position =  projection * view * model * vec4(Position, 1.0);
}
