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
  sampler2D tDiffuseTex;
};

uniform STRUCT_MATERIAL materialInfo;

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

void main()
{
    //
    // Here's how we use each gbuffer:
    // - gPosition: Position in world space. RGB = XYZ position, alpha unused
    // - gNormal: Normal in world space. RGB = XYZ normal, alpha = draw skybox - 1 for don't draw, 0 for draw
    // - gColorSpec: Albedo + specular. RGB = albedo, alpha = specular power (scaled by 512.0).
    //
    gPosition = vec4( fs_in.vWorldPos, 1.0 );
    gNormal = vec4( fs_in.vNormal, 1.0 );

    vec4 diffuseCol = texture( materialInfo.tDiffuseTex, fs_in.vTexCoords.xy );
    // vec4 diffuseCol = vec4( fs_in.vTexCoords.xy, 1, 1 );

    gColorSpec.rgb = diffuseCol.rgb;
    gColorSpec.a = materialInfo.fSpecular / 512.0;
}

#endif