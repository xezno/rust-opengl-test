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

// For poisson disk sampling
vec2 poissonDisk[16] = vec2[] (
    vec2(-0.94201624,    -0.39906216), 
    vec2(0.94558609,     -0.76890725), 
    vec2(-0.094184101,    -0.92938870), 
    vec2(0.34495938,     0.29387760), 
    vec2(-0.91588581,    0.45771432), 
    vec2(-0.81544232,    -0.87912464), 
    vec2(-0.38277543,    0.27676845), 
    vec2(0.97484398,     0.75648379), 
    vec2(0.44323325,     -0.97511554), 
    vec2(0.53742981,     -0.47373420), 
    vec2(-0.26496911,    -0.41893023), 
    vec2(0.79197514,     0.19090188), 
    vec2(-0.24188840,    0.99706507), 
    vec2(-0.81409955,    0.91437590), 
    vec2(0.19984126,     0.78641367), 
    vec2(0.14383161,     -0.14100790)
);


float GetRand(vec4 seed)
{
    float dot_product = dot(seed, vec4(12.9898,78.233,45.164,94.673));
    return fract(sin(dot_product) * 43758.5453);
}

float ShadowCalculation(vec4 fragPosLightSpace, vec3 normal)
{
    float bias = 0.000005;
    vec3 projCoords = fragPosLightSpace.xyz / fragPosLightSpace.w;
    if (projCoords.z > 1.0)
        return 0.0;
    
    projCoords = projCoords * 0.5 + 0.5;
    
    float currentDepth = projCoords.z;

    float shadow = 0.0;
    int samplesX = 6;
    int samplesY = 6;

    vec2 texelSize = 1.0 / textureSize(sShadowMap, 0);
    for(int x = -samplesX/2; x <= samplesX/2; ++x)
    {
        for(int y = -samplesY/2; y <= samplesY/2; ++y)
        {
            float pcfDepth = texture(sShadowMap, projCoords.xy + vec2(x, y) * texelSize).r; 
            shadow += currentDepth - bias > pcfDepth ? 1.0 : 0.0;        
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
    }

    vColor = pow( vColor, vec3( 2.2 ) );

    vec4 vLightSpace = uLightSpaceMat * vec4( vWorldPos, 1.0 );
    vColor *= mix( 0.1, 1.0, ShadowCalculation( vLightSpace, vNormal ));

    FragColor = vec4( vColor, 1.0 );
    // FragColor = vec4( vec3( ShadowCalculation( vLightSpace ) ), 1.0 );
    // FragColor = vLightSpace;
}

#endif