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
  vec3 vFogColor;
};

#define MAX_LIGHTS 256
struct POINT_LIGHT {
    vec3 vPos;
    vec3 vColor;
};

uniform STRUCT_MATERIAL materialInfo;
uniform STRUCT_LIGHTING lightingInfo;
uniform POINT_LIGHT pointLights[MAX_LIGHTS];
uniform int iNumLights;

uniform mat4 uLightSpaceMat;

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

uniform sampler2D sShadowMap;

out vec4 FragColor;

float ShadowCalculation(vec3 worldCoords, vec4 fragPosLightSpace, vec3 normal)
{
    float bias = 0.00001;
    vec3 projCoords = fragPosLightSpace.xyz / fragPosLightSpace.w;
    if (projCoords.z > 1.0)
        return 0.0;
    
    projCoords = projCoords * 0.5 + 0.5;
    
    float currentDepth = projCoords.z;

    float shadow = 0.0;
    int samplesX = 3;
    int samplesY = 3;

    vec2 texelSize = 1.0 / textureSize(sShadowMap, 0);
    for(int x = -samplesX/2; x <= samplesX/2; ++x)
    {
        for(int y = -samplesY/2; y <= samplesY/2; ++y)
        {
            if ( projCoords.x < 0 || projCoords.x > 1 || projCoords.y < 0 || projCoords.y > 1 )
            {
                return 1.0;
            }
            else 
            {
                vec3 worldCoords_ = worldCoords + vec3(x, y, 0) * texelSize.x;
                worldCoords_.y = worldCoords_.y*2;

                float pcfDepth = texture(sShadowMap, projCoords.xy + vec2(x, y) * texelSize).r;                
                shadow += currentDepth - bias > pcfDepth ? 1.0 : 0.0;
            }
        }    
    }
    shadow /= (samplesX * samplesY);
    return shadow;
}  

float lambert( vec3 normal, vec3 lightDir ) 
{
    return max( dot( normalize( normal ), lightDir ), 0.0 );
}

float specular( vec3 normal, vec3 lightDir, vec3 viewDir )
{
    vec3 halfwayDir = normalize( lightDir + viewDir );
    float spec = pow( max( dot( halfwayDir, normal ), 0.0 ), 32 );
    return spec;
}

void main()
{
    bool bDraw = texture( gNormal, fs_in.vTexCoords ).w < 0.01;
    if ( bDraw )
        discard;
    
    vec3 vWorldPos = texture( gPosition, fs_in.vTexCoords ).xyz;
    vec3 vNormal = texture( gNormal, fs_in.vTexCoords ).xyz;
    vec3 vColor = texture( gColorSpec, fs_in.vTexCoords ).rgb;
    float fSpecular = 0.0; //texture( gColorSpec, fs_in.vTexCoords ).a;
    
    vec3 vViewDir = normalize(uCamPos - vWorldPos);

    // Calculate the lighting for each point light in the scene
    for ( int i = 0; i < MAX_LIGHTS; i++ )
    {
        vec3 vLightDir = normalize( pointLights[i].vPos - vWorldPos );
        float lambertian = clamp( lambert( vNormal, vLightDir ), 0.0, 1.0 );

        float spec = 0;
        if ( fSpecular > 0 )
            spec = specular( vNormal, vLightDir, vViewDir ) * fSpecular;

        vec3 lighting = ( lambertian + spec ) * pointLights[i].vColor;
        float attenuation = 1.0 / ( 32 + length( pointLights[i].vPos - vWorldPos ) );
        
        vColor += lighting * attenuation;

        if ( i > iNumLights )
            break;
    }

    vec4 vLightSpace = uLightSpaceMat * vec4( vWorldPos, 1.0 );
    vColor *= mix( 0.4, 1.25, ShadowCalculation( vWorldPos, vLightSpace, vNormal ));

    vColor = pow( vColor, vec3( 2.2 ) );
    FragColor = vec4( vColor, 1.0 );
}

#endif