// ============================================================================
//
// gfx.rs
//
// Purpose: Graphics helpers
//
// ============================================================================

pub fn gfx_setup() {
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }

    gfx_clear_color(0.34, 0.12, 0.56, 1.0);
}

pub fn gfx_clear_color(red: f32, green: f32, blue: f32, alpha: f32) {
    unsafe {
        gl::ClearColor(red, green, blue, alpha);
    }
}

pub fn gfx_clear() {
    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }
}

pub fn gfx_check_generic_errors() {
    unsafe {
        // Check for errors
        let error = gl::GetError();
        if error != gl::NO_ERROR {
            panic!("Error compiling shader {}", error);
        }
    }
}
