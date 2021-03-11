#version 330 core

in VS_OUTPUT {
    vec3 Color;
} IN;


uniform vec3 lightColor;
out vec4 Color;

void main()
{
    float ambientStrength = 0.1;
    vec3 ambient = ambientStrength * lightColor;
    Color =  vec4( ambient * IN.Color, 1.0f);
 }