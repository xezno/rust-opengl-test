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

        // Cornflower blue as hex
        let col = crate::render::color::from_hex("#6495ED");

        gl::ClearColor(col.0, col.1, col.2, 1.0);

        gl::ClearDepth(0.0);
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::GREATER);

        gl::Enable(gl::MULTISAMPLE);
    }
}

pub fn gfx_resize(w: i32, h: i32) {
    unsafe {
        gl::Viewport(0, 0, w, h);
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
