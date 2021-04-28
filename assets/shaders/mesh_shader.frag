#version 330 core

in VS_OUTPUT {
    vec3 Normal;
    vec3 FragPos;
    vec2 TexCord;
    vec3 Color;
} IN;


uniform vec3 lightColor;
uniform vec3 lightPos;
uniform sampler2D Texture;

out vec4 Color;



void main()
{
  float ambientStrength = 0.5;
  vec3 ambient = ambientStrength * lightColor;

  vec3 norm = normalize(IN.Normal);
  vec3 lightDir = normalize(lightPos - IN.FragPos);
  float diff = max(dot(norm, lightDir), 0.0);
  vec3 diffuse = (diff * lightColor) * 0.5;


  vec3 color = texture(Texture, IN.TexCord).rgb;
  Color =  vec4( (ambient + diffuse) * color, 1.0f);


  // dispplay the bones influences
  //Color =  vec4( IN.Color.xyz, 1.0f);

 }
