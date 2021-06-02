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

  //TODO set this in uniform
  vec3 color = vec3(0.0);


  float st = fract(IN.FragPos.x*2);

  color = vec3(st,st,st);

  gl_FragColor = vec4(texture(Texture, IN.TexCord).rgb, 1.0f);

  // dispplay the bones influences
  //Color =  vec4( IN.Color.xyz, 1.0f);

 }
