#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec3 Normal;
layout (location = 2) in vec2 BoneWeights;
layout (location = 3) in vec2 BoneIndices;
layout (location = 4) in vec2 TexCord;

out VS_OUTPUT {
   vec3 Normal;
   vec3 FragPos;
   vec2 TexCord;
} OUT;


uniform mat4 uBones[32];

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;



mat4 boneTransform() {

  mat4 ret;

  // Weight1 * Bone1 + Weight2 * Bone2
  ret = BoneWeights.x * uBones[int(BoneIndices.x)]
       + BoneWeights.y * uBones[int(BoneIndices.y)];

  return ret;

}


void main()
{
    mat4 bt = boneTransform();

    OUT.FragPos = vec3(model * bt * vec4(Position, 1.0));

    // This is called normal matrix, maybe do on cpu( the transpose and invere part)
    // and send it in as a uniform
    OUT.Normal = mat3(transpose(inverse(model))) * (model * bt * vec4(Normal, 1.0)).xyz;

    OUT.TexCord = TexCord;

    gl_Position =  projection * view * model * bt  * vec4(Position, 1.0);


}
