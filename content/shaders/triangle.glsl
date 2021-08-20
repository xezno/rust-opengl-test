// ============================================================================
//
// Common shit
// 
struct FS_IN {
  vec3 pos;
};

// ============================================================================
//
// Vertex shader
// 
#ifdef VERTEX

layout(location = 0) in vec3 inPos;

out FS_IN fs_in;

void main() {
  fs_in.pos = inPos;

  gl_Position = vec4( inPos, 1.0 );
}

#endif

// ============================================================================
//
// Fragment shader
//
#ifdef FRAGMENT

in FS_IN fs_in;

out vec4 FragColor;

void main()
{
  FragColor = vec4( fs_in.pos, 1.0 );
} 

#endif