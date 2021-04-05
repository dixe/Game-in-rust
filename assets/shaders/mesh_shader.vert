#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec3 Normal;

out VS_OUTPUT {
   vec3 Color;
   vec3 Normal;
   vec3 FragPos;
} OUT;


uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;


void main()
{
    OUT.FragPos = vec3(model * vec4(Position, 1.0));
    // This is called normal matrix, maybe do on cpu( the transpose and invere part)
    // and send it in as a uniform
    OUT.Normal = mat3(transpose(inverse(model))) * Normal;
    OUT.Color = vec3(1.0,1.0,0.0);

    gl_Position = projection * view * model * vec4(Position, 1.0);

}