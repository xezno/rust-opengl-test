// ============================================================================
//
// Common shit
// 
struct FS_IN {
  vec3 vWorldPos;
  vec3 vNormal;
  vec4 vScreenPos;
};

struct STRUCT_LIGHTING {
  vec3 vLightPos;
  vec3 vLightColor;
};

uniform STRUCT_LIGHTING lightingInfo;

// ============================================================================
//
// Vertex shader
// 
#ifdef VERTEX

layout(location = 0) in vec3 inPos;
layout(location = 1) in vec3 inNormal;

uniform mat4 uModelMat;
uniform mat4 uProjViewMat;

out FS_IN fs_in;

void main() 
{
  fs_in.vWorldPos = vec3( uModelMat * vec4( inPos, 1.0 ) );
  fs_in.vNormal = inNormal;
  fs_in.vScreenPos = uProjViewMat * uModelMat * vec4( inPos, 1.0 );
  
  gl_Position = fs_in.vScreenPos;
}

#endif

// ============================================================================
//
// Fragment shader
//
#ifdef FRAGMENT

in FS_IN fs_in;

uniform vec3 uCamPos;
uniform vec4 uModelCol;

out vec4 FragColor;

float lambert( vec3 normal, vec3 lightDir ) 
{
  return max( dot( normal, lightDir ), 0.0 );
}

float specular( vec3 normal, vec3 lightDir, vec3 viewDir, float shininess )
{
  vec3 reflectDir = reflect( -lightDir, normal );
  float spec = pow( max( dot( viewDir, reflectDir ), 0.0 ), shininess );
  return spec;
}

void main()
{
  vec3 lightDir = normalize( lightingInfo.vLightPos - fs_in.vWorldPos);
  vec3 normal = normalize( fs_in.vNormal );

  vec3 lambertian = lambert( normal, lightDir ) * uModelCol.xyz;
  vec3 spec = ( specular( normal, lightDir, normalize( uCamPos - fs_in.vWorldPos ), 64.0 ) * 2 ) * lightingInfo.vLightColor;
  vec3 ambient = 0.4 * uModelCol.xyz;

  vec3 lighting = lambertian + spec + ambient;

  FragColor = vec4( lighting, 1.0 );
} 

#endif