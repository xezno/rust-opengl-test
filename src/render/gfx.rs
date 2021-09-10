// ============================================================================
//
// gfx.rs
//
// Purpose: Graphics helpers
//
// ============================================================================

use build_timestamp::build_time;
use gl::types::*;
use glam::IVec2;
use std::{ffi::c_void, ptr};

pub fn gfx_setup(window: &mut sdl2::video::Window) {
    unsafe {
        let mut major = -1;
        gl::GetIntegerv(gl::MAJOR_VERSION, &mut major);

        let mut minor = -1;
        gl::GetIntegerv(gl::MINOR_VERSION, &mut minor);

        let version_triple = version_check::triple().unwrap();
        let version = version_triple.0;
        let channel = version_triple.1;

        build_time!("%Y-%m-%d %H:%M:%S");

        window
            .set_title(
                format!(
                    "{} {} | OpenGL {}.{} | Rust {} {}",
                    env!("CARGO_PKG_NAME"),
                    BUILD_TIME,
                    // |
                    major,
                    minor,
                    // |
                    version,
                    channel
                )
                .as_str(),
            )
            .unwrap();

        let window_icon = sdl2::surface::Surface::load_bmp("content/suzanne.bmp").unwrap();
        window.set_icon(&window_icon);

        gl::Enable(gl::MULTISAMPLE);
        gl::Enable(gl::CULL_FACE);
    }
}

pub fn gfx_resize(w: i32, h: i32) {
    unsafe {
        gl::Viewport(0, 0, w, h);
    }
}

pub fn gfx_clear() {
    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);
    }
}

pub fn gfx_check_generic_errors() {
    unsafe {
        // Check for errors
        let error = gl::GetError();
        if error != gl::NO_ERROR {
            panic!("GL error {}", error); // TODO (04/09/2021): Handle this better
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
    g_orm: &mut GLuint,
) -> GLuint {
    let mut g_buffer: GLuint = 0;
    unsafe {
        gl::GenFramebuffers(1, &mut g_buffer);
        gl::BindFramebuffer(gl::FRAMEBUFFER, g_buffer);
    }

    let window_size = crate::util::screen::get_screen().size;

    gfx_create_single_g_buffer(&mut *g_position, window_size, gl::COLOR_ATTACHMENT0, false);
    gfx_create_single_g_buffer(&mut *g_normal, window_size, gl::COLOR_ATTACHMENT1, true);
    gfx_create_single_g_buffer(
        &mut *g_color_spec,
        window_size,
        gl::COLOR_ATTACHMENT2,
        false,
    );
    gfx_create_single_g_buffer(&mut *g_orm, window_size, gl::COLOR_ATTACHMENT3, false);

    let mut rbo: GLuint = 0;
    unsafe {
        gl::GenRenderbuffers(1, &mut rbo);
        gl::BindRenderbuffer(gl::RENDERBUFFER, rbo);
        gl::RenderbufferStorage(
            gl::RENDERBUFFER,
            gl::DEPTH24_STENCIL8,
            window_size.x,
            window_size.y,
        );
        gl::FramebufferRenderbuffer(
            gl::FRAMEBUFFER,
            gl::DEPTH_STENCIL_ATTACHMENT,
            gl::RENDERBUFFER,
            rbo,
        );
    }

    return g_buffer;
}

fn gfx_create_single_g_buffer(
    g_buffer_tex: &mut GLuint,
    window_size: IVec2,
    attachment: GLuint,
    with_alpha: bool,
) {
    unsafe {
        gl::GenTextures(1, g_buffer_tex);
        gl::BindTexture(gl::TEXTURE_2D, *g_buffer_tex);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            if with_alpha { gl::RGBA16F } else { gl::RGB16F } as i32,
            window_size.x,
            window_size.y,
            0,
            if with_alpha { gl::RGBA } else { gl::RGB },
            gl::FLOAT,
            std::ptr::null_mut(),
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
        gl::FramebufferTexture2D(
            gl::FRAMEBUFFER,
            attachment,
            gl::TEXTURE_2D,
            *g_buffer_tex,
            0,
        );
    }
}

pub fn gfx_quad_setup() -> GLuint {
    let mut vao: GLuint = 0;
    let mut vbo: GLuint = 0;

    #[rustfmt::skip]
    let quad_verts: [f32; 20] = [
        // Positions         // Texture Coords
        -1.0,  1.0, 0.0,    0.0, 1.0,
        -1.0, -1.0, 0.0,    0.0, 0.0,
        1.0,  1.0, 0.0,     1.0, 1.0,
        1.0, -1.0, 0.0,     1.0, 0.0,
    ];

    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (quad_verts.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
            &quad_verts[0] as *const GLfloat as *const c_void,
            gl::STATIC_DRAW,
        );
        let stride = (5 * std::mem::size_of::<GLfloat>()) as GLsizei;
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            stride,
            (3 * std::mem::size_of::<GLfloat>()) as *const c_void,
        );
    }

    return vao;
}

pub fn gfx_quad_render(vao: GLuint) {
    unsafe {
        gl::BindVertexArray(vao);
        gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
        gl::BindVertexArray(0);
    }
}

pub fn gfx_prepare_geometry_pass(fbo: GLuint) {
    unsafe {
        gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);

        gl::Enable(gl::DEPTH_TEST);
        gl::ClipControl(gl::LOWER_LEFT, gl::ZERO_TO_ONE);
        gl::DepthFunc(gl::GREATER);
        gl::CullFace(gl::BACK);

        gl::Viewport(
            0,
            0,
            crate::util::screen::get_screen().size.x,
            crate::util::screen::get_screen().size.y,
        );

        let attachments = [
            gl::COLOR_ATTACHMENT0,
            gl::COLOR_ATTACHMENT1,
            gl::COLOR_ATTACHMENT2,
            gl::COLOR_ATTACHMENT3,
        ];
        gl::ClearDepth(0.0);
        gl::ClearColor(0.0, 0.0, 0.0, 0.0);
        gl::NamedFramebufferDrawBuffers(fbo, 4, &attachments[0]);
    }
}

const SHADOW_MAP_SIZE: GLint = 2048;

pub fn gfx_prepare_shadow_pass(fbo: GLuint) {
    unsafe {
        gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);

        gl::Enable(gl::DEPTH_TEST);
        gl::ClipControl(gl::LOWER_LEFT, gl::NEGATIVE_ONE_TO_ONE);
        gl::DepthFunc(gl::GREATER);
        gl::CullFace(gl::FRONT);

        gl::Viewport(0, 0, SHADOW_MAP_SIZE, SHADOW_MAP_SIZE);
        let attachments = [gl::DEPTH_ATTACHMENT];

        gl::ClearDepth(0.0);
        gl::ClearColor(0.0, 0.0, 0.0, 0.0);
        gl::NamedFramebufferDrawBuffers(fbo, 1, &attachments[0]);
    }
}

pub fn gfx_prepare_lighting_pass(sky_color: &(f32, f32, f32)) {
    unsafe {
        gl::DepthFunc(gl::LESS);
        gl::Disable(gl::DEPTH_TEST);
        gl::ClearColor(sky_color.0, sky_color.1, sky_color.2, 1.0);
        gl::Enable(gl::FRAMEBUFFER_SRGB);
    }
}

pub fn gfx_prepare_imgui_pass() {
    unsafe {
        gl::Disable(gl::FRAMEBUFFER_SRGB);
    }
}

pub fn gfx_setup_shadow_buffer() -> (GLuint, GLuint) {
    let mut shadow_buffer: GLuint = 0;
    let mut shadow_buffer_tex: GLuint = 0;
    unsafe {
        gl::GenFramebuffers(1, &mut shadow_buffer);
        gl::BindFramebuffer(gl::FRAMEBUFFER, shadow_buffer);
    }

    unsafe {
        gl::GenTextures(1, &mut shadow_buffer_tex);
        gl::BindTexture(gl::TEXTURE_2D, shadow_buffer_tex);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::DEPTH_COMPONENT as i32,
            SHADOW_MAP_SIZE,
            SHADOW_MAP_SIZE,
            0,
            gl::DEPTH_COMPONENT,
            gl::FLOAT,
            std::ptr::null_mut(),
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_WRAP_S,
            gl::CLAMP_TO_BORDER as i32,
        );
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_WRAP_T,
            gl::CLAMP_TO_BORDER as i32,
        );
        let border_color = [1.0, 1.0, 1.0, 1.0];
        gl::TexParameterfv(gl::TEXTURE_2D, gl::TEXTURE_BORDER_COLOR, &border_color[0]);
        gl::FramebufferTexture2D(
            gl::FRAMEBUFFER,
            gl::DEPTH_ATTACHMENT,
            gl::TEXTURE_2D,
            shadow_buffer_tex,
            0,
        );
    }

    return (shadow_buffer, shadow_buffer_tex);
}
