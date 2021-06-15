#version 330 core

in VS_OUTPUT {
    vec3 Normal;
    vec3 FragPos;
    vec3 Color;
} IN;


uniform vec3 lightColor;
uniform vec3 lightPos;
uniform vec3 viewPos;
out vec4 Color;



void main()
{

  // ABIENT
  float ambientStrength = 0.5;
  vec3 ambient = ambientStrength * lightColor;



  //DIFFUSE
  vec3 norm = normalize(IN.Normal);
  vec3 lightDir = normalize(lightPos - IN.FragPos);
  float diff = max(dot(norm, lightDir), 0.0);

  /*
  float mul = 0.0;
  if (diff > 0.)
  {
    mul = 1.0;
  }

  diff = diff * mul;
  */

  vec3 diffuse = (diff * lightColor) * 0.70;


  // SPECULAR
  float specularStrength = 0.1;
  vec3 viewDir = normalize(viewPos - IN.FragPos);
  vec3 reflectionDir = reflect(-lightDir, IN.Normal);

  float spec = pow(max(dot(viewDir, reflectionDir), 0.0), 5);
  vec3 specular = specularStrength * spec * lightColor;


  Color = vec4( (ambient + diffuse + specular) * IN.Color, 1.0f);

  //Color = vec4( (ambient + diffuse + specular) * color, 1.0f);
  //Color = vec4( lightDir, 1.0f);

  // dispplay the bones influences
  //Color =  vec4( IN.Color.rgb, 1.0f);





 }
