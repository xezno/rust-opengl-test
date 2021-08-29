// ============================================================================
//
// Common shit
// 
struct FS_IN {
  vec3 vWorldPos;
  vec3 vNormal;
  vec4 vScreenPos;
  vec2 vTexCoords;
};

struct STRUCT_MATERIAL {
  float fSpecular;
  vec4 vDiffuseCol;
};

struct STRUCT_LIGHTING {
  vec3 vLightDir;
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
layout(location = 1) in vec3 inTexCoords;

uniform mat4 uModelMat;
uniform mat4 uProjViewMat;

out FS_IN fs_in;

void main() 
{
  fs_in.vScreenPos = vec4( inPos, 1.0 );
  fs_in.vTexCoords = inTexCoords.xy;
  
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

uniform sampler2D gPosition;
uniform sampler2D gNormal;
uniform sampler2D gColorSpec;

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
  // retrieve data from G-buffer
  vec3 FragPos = texture(gPosition, fs_in.vTexCoords).rgb;
  vec4 Normal = texture(gNormal, fs_in.vTexCoords);
  vec3 Albedo = texture(gColorSpec, fs_in.vTexCoords).rgb;
  float Specular = texture(gColorSpec, fs_in.vTexCoords).a;
  
  if ( Normal.w == 0.0 )
    discard;
  
  vec3 lightDir = normalize( lightingInfo.vLightDir );
  vec3 normal = normalize( Normal.xyz );

  vec3 lambertian = lambert( normal, lightDir ) * Albedo;
  vec3 spec = ( specular( normal, lightDir, normalize( uCamPos - FragPos ), Specular * 512.0 ) ) * lightingInfo.vLightColor;
  vec3 ambient = 0.4 * Albedo;

  if ( Specular <= 0 )
  {
    spec = vec3( 0.0 );
  }

  vec3 lighting = ( lambertian + spec ) * lightingInfo.vLightColor;
  lighting += ambient * normalize( lightingInfo.vLightColor );

  lighting = pow( lighting, vec3( 2.2 ) );
  FragColor = vec4( lighting, 1.0);
} 

#endif