// ============================================================================
//
// gfx.rs
//
// Purpose: Graphics helpers
//
// ============================================================================
use gl::types::GLuint;

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

pub fn gfx_bind_framebuffer(fbo: GLuint) {
    unsafe {
        gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);
    }
}

pub fn gfx_setup_gbuffer(
    g_position: &mut GLuint,
    g_normal: &mut GLuint,
    g_color_spec: &mut GLuint,
) -> GLuint {
    let mut g_buffer: GLuint = 0;
    unsafe {
        gl::GenFramebuffers(1, &mut g_buffer);
        gl::BindFramebuffer(gl::FRAMEBUFFER, g_buffer);
    }

    let window_size = crate::util::screen::get_screen().size;

    //
    // Position color buffer
    //
    unsafe {
        gl::GenTextures(1, g_position);
        gl::BindTexture(gl::TEXTURE_2D, *g_position);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA16F as i32,
            window_size.x,
            window_size.y,
            0,
            gl::RGBA,
            gl::FLOAT,
            std::ptr::null_mut(),
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        gl::FramebufferTexture2D(
            gl::FRAMEBUFFER,
            gl::COLOR_ATTACHMENT0,
            gl::TEXTURE_2D,
            *g_position,
            0,
        );
    }

    //
    // Normal color buffer
    //
    unsafe {
        gl::GenTextures(1, g_normal);
        gl::BindTexture(gl::TEXTURE_2D, *g_normal);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA16F as i32,
            window_size.x,
            window_size.y,
            0,
            gl::RGBA,
            gl::FLOAT,
            std::ptr::null_mut(),
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        gl::FramebufferTexture2D(
            gl::FRAMEBUFFER,
            gl::COLOR_ATTACHMENT1,
            gl::TEXTURE_2D,
            *g_normal,
            0,
        );
    }

    //
    // Color + specular color buffer
    //
    unsafe {
        gl::GenTextures(1, g_color_spec);
        gl::BindTexture(gl::TEXTURE_2D, *g_color_spec);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA16F as i32,
            window_size.x,
            window_size.y,
            0,
            gl::RGBA,
            gl::FLOAT,
            std::ptr::null_mut(),
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        gl::FramebufferTexture2D(
            gl::FRAMEBUFFER,
            gl::COLOR_ATTACHMENT2,
            gl::TEXTURE_2D,
            *g_color_spec,
            0,
        );
    }

    let mut rbo: GLuint = 0;
    unsafe {
        gl::GenRenderbuffers(1, &mut rbo);
        gl::BindRenderbuffer(gl::RENDERBUFFER, rbo);
        gl::RenderbufferStorage(
            gl::RENDERBUFFER,
            gl::DEPTH_COMPONENT24,
            window_size.x,
            window_size.y,
        );
        gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT, gl::RENDERBUFFER, rbo);
    }

    return g_buffer;
}
