// ============================================================================
//
// Common shit
// 
struct FS_IN {
  vec3 vWorldPos;
  vec3 vNormal;
  vec4 vScreenPos;
};

struct STRUCT_MATERIAL {
  float fSpecular;
  vec4 vDiffuseCol;
};

struct STRUCT_LIGHTING {
  vec3 vLightPos;
  vec3 vLightColor;
};

uniform STRUCT_MATERIAL materialInfo;
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

  vec3 lambertian = lambert( normal, lightDir ) * materialInfo.vDiffuseCol.xyz;
  vec3 spec = ( specular( normal, lightDir, normalize( uCamPos - fs_in.vWorldPos ), materialInfo.fSpecular ) * 2 ) * lightingInfo.vLightColor;
  vec3 ambient = 0.4 * materialInfo.vDiffuseCol.xyz;

  if ( materialInfo.fSpecular <= 0 )
  {
    spec = vec3( 0.0 );
  }

  vec3 lighting = ( lambertian + spec ) * lightingInfo.vLightColor;// * normalize( lightingInfo.vLightColor );

  lighting += ambient* normalize( lightingInfo.vLightColor );

  FragColor = vec4( pow( lighting, vec3( 2.2 ) ), 1.0 );
} 

#endif