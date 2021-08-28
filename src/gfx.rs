// ============================================================================
//
// gfx.rs
//
// Purpose: Graphics helpers
//
// ============================================================================

pub fn gfx_setup(window: &mut sdl2::video::Window) {
    unsafe {
        let mut major = -1;
        gl::GetIntegerv(gl::MAJOR_VERSION, &mut major);

        let mut minor = -1;
        gl::GetIntegerv(gl::MINOR_VERSION, &mut minor);

        window
            .set_title(format!("My Game, OpenGL {}.{}", major, minor).as_str())
            .unwrap();

        gl::ClipControl(gl::LOWER_LEFT, gl::ZERO_TO_ONE);
        gl::ClearColor(0.34, 0.12, 0.56, 1.0);

        gl::ClearDepth(0.0);
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::GREATER);
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
