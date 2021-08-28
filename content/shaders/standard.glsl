// ============================================================================
//
// Common shit
// 
struct FS_IN {
  vec3 vWorldPos;
  vec3 vNormal;
  vec4 vScreenPos;
};

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
  fs_in.vWorldPos = inPos;
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
  const vec3 color = vec3( 0.32, 0.37, 0.87 );
  vec3 lightDir = normalize( vec3( 0.0, 1.0, 1.0 ) );
  vec3 normal = normalize( fs_in.vNormal );

  float lambertian = lambert( normal, lightDir );
  float spec = specular( normal, lightDir, normalize( uCamPos - fs_in.vWorldPos ), 64.0 ) * 2;
  float ambient = 0.3;

  vec3 lighting = color * ( lambertian + spec + ambient );

  FragColor = vec4( lighting, 1.0 );
} 

#endif