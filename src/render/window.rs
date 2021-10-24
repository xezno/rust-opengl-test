use glam::uvec2;
use sdl2::{
    sys::{SDL_GL_SetAttribute, SDL_GL_SetSwapInterval, SDL_Window},
    Sdl, VideoSubsystem,
};

use crate::util::screen::update_screen;

// ============================================================================
//
// window.rs
//
// Purpose:
//
// ============================================================================

pub struct Window {
    pub sdl_window: sdl2::video::Window,
    pub video_subsystem: VideoSubsystem,
    pub sdl: Sdl,
}

impl Window {
    pub fn new() -> Window {
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();

        unsafe {
            assert_eq!(
                0,
                SDL_GL_SetAttribute(sdl2::sys::SDL_GLattr::SDL_GL_MULTISAMPLEBUFFERS, 1)
            );
            assert_eq!(
                0,
                SDL_GL_SetAttribute(sdl2::sys::SDL_GLattr::SDL_GL_MULTISAMPLESAMPLES, 4)
            );
        }

        let win_size = uvec2(1600, 900);

        let mut window = video_subsystem
            .window("", win_size.x, win_size.y)
            .opengl()
            .resizable()
            .build()
            .unwrap();

        unsafe {
            SDL_GL_SetAttribute(sdl2::sys::SDL_GLattr::SDL_GL_CONTEXT_MAJOR_VERSION, 4);
            SDL_GL_SetAttribute(sdl2::sys::SDL_GLattr::SDL_GL_CONTEXT_MINOR_VERSION, 6);

            SDL_GL_SetAttribute(
                sdl2::sys::SDL_GLattr::SDL_GL_CONTEXT_PROFILE_MASK,
                sdl2::sys::SDL_GLprofile::SDL_GL_CONTEXT_PROFILE_CORE as i32,
            );
        }

        update_screen(win_size.as_i32());

        let _gl_context = window.gl_create_context().unwrap();

        unsafe {
            assert_eq!(0, SDL_GL_SetSwapInterval(0));
        }

        return Window {
            sdl_window: window,
            video_subsystem: video_subsystem,
            sdl: sdl,
        };
    }

    pub fn swap(&self) {
        self.sdl_window.gl_swap_window();
    }
}
