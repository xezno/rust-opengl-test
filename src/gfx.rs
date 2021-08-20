// ============================================================================
//
// gfx.rs
//
// Purpose: Graphics middleware / abstraction layer.
//
// ============================================================================


pub fn gfx_clear_color( red: f32, green: f32, blue: f32, alpha: f32 ) { 
  unsafe {
    gl::ClearColor( red, green, blue, alpha );
  }
}

pub fn gfx_clear() {
  unsafe {
    gl::Clear( gl::COLOR_BUFFER_BIT );
  }
}