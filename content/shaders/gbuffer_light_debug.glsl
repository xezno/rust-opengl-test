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

// 
// Basic lighting data
struct STRUCT_LIGHTING {
  vec3 vLightDir;
  vec3 vLightColor;
};

//
// Basic material info for this object
struct STRUCT_MATERIAL {
  float fSpecular;
  sampler2D tDiffuseTex;
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
layout(location = 2) in vec2 inTexCoords;

uniform mat4 uModelMat;
uniform mat4 uProjViewMat;

out FS_IN fs_in;

void main() 
{
  fs_in.vWorldPos = vec3( uModelMat * vec4( inPos, 1.0 ) );
  fs_in.vNormal = inNormal;
  fs_in.vScreenPos = uProjViewMat * uModelMat * vec4( inPos, 1.0 );
  fs_in.vTexCoords = inTexCoords;
  
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

uniform vec3 vDebugLightCol;

layout (location = 0) out vec4 gPosition;
layout (location = 1) out vec4 gNormal;
layout (location = 2) out vec4 gColorSpec;

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
    gPosition = vec4( fs_in.vWorldPos, 1.0 );
    gNormal = vec4( fs_in.vNormal, 1.0 );
    gColorSpec.rgb = vDebugLightCol;

    gColorSpec.a = 0.0;
}

#endif