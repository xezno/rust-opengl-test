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

#define MAX_LIGHTS 256
struct POINT_LIGHT {
    vec3 vPos;
    vec3 vColor;
};

uniform STRUCT_MATERIAL materialInfo;
uniform STRUCT_LIGHTING lightingInfo;
uniform POINT_LIGHT pointLights[MAX_LIGHTS];

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
    return max( dot( normalize( normal ), normalize( lightDir ) ), 0.0 );
}

float specular( vec3 normal, vec3 lightDir, vec3 viewDir, float shininess )
{
    vec3 halfwayDir = normalize( lightDir + viewDir );
    float spec = pow( max( dot( halfwayDir, normal ), 0.0 ), shininess );
    return spec;
}

void main()
{
    vec3 vWorldPos = texture( gPosition, fs_in.vTexCoords ).xyz;
    vec3 vNormal = texture( gNormal, fs_in.vTexCoords ).xyz;
    bool bDraw = texture( gNormal, fs_in.vTexCoords ).w < 0.01;
    vec3 vColor = texture( gColorSpec, fs_in.vTexCoords ).rgb;
    float fSpecular = texture( gColorSpec, fs_in.vTexCoords ).a;
    
    if ( bDraw )
        discard;
    
    vec3 vViewDir = normalize(uCamPos - vWorldPos);

    
    // Calculate the lighting for each point light in the scene
    for ( int i = 0; i < MAX_LIGHTS; i++ )
    {
        vec3 vLightDir = normalize( pointLights[i].vPos - vWorldPos );
        float lambertian = clamp( lambert( vNormal, vLightDir ), 0.0, 1.0 );

        float spec = 0;
        if ( fSpecular > 0 )
            spec = specular( vNormal, vLightDir, vViewDir, fSpecular * 512.0 );

        // lambertian = 0;
        // spec = 0;
        vec3 lighting = ( lambertian + spec ) * pointLights[i].vColor;
        float attenuation = 1.0 / ( 32 + length( pointLights[i].vPos - vWorldPos ) );
        
        vColor += lighting * attenuation;
    }
    
    vColor = pow( vColor, vec3( 2.2 ) );
    FragColor = vec4( vColor, 1.0 );
}

#endif