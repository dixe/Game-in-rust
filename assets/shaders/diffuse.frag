#version 330 core

in VS_OUTPUT {
    vec3 Color;
    vec3 Normal;
    vec3 FragPos;
} IN;


uniform vec3 lightColor;
uniform vec3 lightPos;
out vec4 Color;

void main()
{
    float ambientStrength = 0.1;
    vec3 ambient = ambientStrength * lightColor;

    vec3 norm = normalize(IN.Normal);
    vec3 lightDir = normalize(lightPos - IN.FragPos);
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = diff * lightColor;


    Color =  vec4( (ambient + diffuse) * IN.Color, 1.0f);
 }