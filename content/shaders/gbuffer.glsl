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

layout (location = 0) out vec4 gPosition;
layout (location = 1) out vec4 gNormal;
layout (location = 2) out vec4 gColorSpec;

float lambert( vec3 normal, vec3 lightDir ) 
{
    return max( dot( normal, lightDir ), 0.0 );
}

float specular( vec3 normal, vec3 lightDir, vec3 viewDir )
{
    vec3 reflectDir = reflect( -lightDir, normal );
    float spec = pow( max( dot( viewDir, reflectDir ), 0.0 ), 32 );
    return spec;
}

void main()
{
    //
    // Here's how we use each gbuffer:
    // - gPosition: Position in world space. RGB = XYZ position, alpha unused
    // - gNormal: Normal in world space. RGB = XYZ normal, alpha = draw skybox - 1 for don't draw, 0 for draw
    // - gColorSpec: Albedo (w/ scene directional lighting calculated) + specular. RGB = albedo, alpha = specular power (scaled by 512.0).
    //
    gPosition = vec4( fs_in.vWorldPos, 1.0 );
    gNormal = vec4( fs_in.vNormal, 1.0 );

    // Weird step: calculate sun lighting here rather than in our lighting pass
    {
        vec4 diffuseCol = texture( materialInfo.tDiffuseTex, fs_in.vTexCoords.xy );
        
        vec3 lightDir = normalize( lightingInfo.vLightDir );
        vec3 normal = normalize( fs_in.vNormal.xyz );

        vec3 lambertian = lambert( normal, lightDir ) * diffuseCol.rgb;
        vec3 spec = specular( normal, lightDir, normalize( uCamPos - fs_in.vWorldPos ) ) * lightingInfo.vLightColor * materialInfo.fSpecular;
        vec3 ambient = 0.3 * diffuseCol.rgb;

        vec3 lighting = ( lambertian + spec ) * lightingInfo.vLightColor;
        lighting += ambient * normalize( lightingInfo.vLightColor );

        diffuseCol = vec4( lighting, 1.0 );
        gColorSpec.rgb = diffuseCol.rgb;
    }

    gColorSpec.a = 1.0;
}

#endif