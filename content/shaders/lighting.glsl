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

#define MAX_LIGHTS 384
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
    return max( dot( normal, lightDir ), 0.0 );
}

float specular( vec3 normal, vec3 lightDir, vec3 viewDir, float shininess )
{
    vec3 halfwayDir = normalize( lightDir + viewDir );
    float spec = pow( max( dot( normal, halfwayDir ), 0.0 ), shininess );
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

    // We already calculated the main directional lighting in the gbuffer stage
    // so we can just use that.

    // We'll also calculate the lighting for each point light in the scene
    vec3 vViewDir = normalize(uCamPos - vWorldPos);
    
    for ( int i = 0; i < MAX_LIGHTS; i++ )
    {
        vec3 vLightDir = normalize( pointLights[i].vPos - vWorldPos );
        float lambertian = lambert( vNormal, vLightDir );

        float spec = 0;
        if ( fSpecular > 0 )
            spec = specular( vNormal, vLightDir, normalize( uCamPos - fs_in.vWorldPos ), fSpecular * 512.0 );

        vec3 lighting = ( lambertian + spec ) * pointLights[i].vColor;
        float attenuation = 1.0 / ( 32 + length( pointLights[i].vPos - vWorldPos ) );
        
        vColor += lighting * attenuation * 4;
    }
    
    FragColor = vec4( vColor, 1.0 );
    // FragColor = vec4( pointLights[i].vPos, 1.0 );
}

#endif