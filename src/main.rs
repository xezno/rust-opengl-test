// ============================================================================
//
// main.rs
//
// Purpose: The main entry point for the application.
//
// ============================================================================

extern crate gl;
extern crate sdl2;

use glam::*;
use render::{gfx::*, shader::Shader};
use scene::{camera::Camera, scene::Scene};
use sdl2::sys::SDL_GL_SetAttribute;
use util::{input::INPUT, screen::update_screen, time::update_time};

pub mod render;
pub mod scene;
pub mod util;

fn main() {
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

    let mut window = video_subsystem
        .window("", 1280, 720)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();

    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);
    let _viewport = gl::Viewport::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);

    let mut imgui = imgui::Context::create();
    let imgui_renderer = imgui_opengl_renderer::Renderer::new(&mut imgui, |s| {
        video_subsystem.gl_get_proc_address(s) as _
    });
    let mut imgui_sdl2 = imgui_sdl2::ImguiSdl2::new(&mut imgui, &window);

    //
    // Imgui setup
    //
    let mut io = imgui.io_mut();
    io.display_size = [1280.0, 720.0];

    update_screen(IVec2::new(1280, 720));

    gfx_setup(&mut window);

    let mut camera = Camera::new();
    let mut shader = Shader::new("content/shaders/standard.glsl");
    shader.scan_uniforms();

    //
    // Scene setup
    //
    let scene = Scene::new("content/scene.json");
    let loaded_scene = scene.load();

    let mut event_pump = sdl.event_pump().unwrap();
    let mut last_time = std::time::Instant::now();

    'main: loop {
        //
        // Input
        //
        {
            // Reset input
            unsafe {
                INPUT.mouse.delta = vec2(0.0, 0.0);
                INPUT.mouse.wheel = 0.0;
            }

            for event in event_pump.poll_iter() {
                imgui_sdl2.handle_event(&mut imgui, &event);
                if imgui_sdl2.ignore_event(&event) {
                    continue;
                }

                match event {
                    //
                    // Mouse
                    //
                    sdl2::event::Event::MouseMotion { x, y, .. } => unsafe {
                        let delta = vec2(
                            (x - INPUT.mouse.position.x) as f32,
                            (y - INPUT.mouse.position.y) as f32,
                        );

                        INPUT.mouse.delta = delta;
                        INPUT.mouse.position = IVec2::new(x, y);
                    },
                    sdl2::event::Event::MouseButtonDown { mouse_btn, .. } => unsafe {
                        match mouse_btn {
                            sdl2::mouse::MouseButton::Left => INPUT.mouse.left = true,
                            sdl2::mouse::MouseButton::Right => INPUT.mouse.right = true,
                            _ => {}
                        }
                    },
                    sdl2::event::Event::MouseButtonUp { mouse_btn, .. } => unsafe {
                        match mouse_btn {
                            sdl2::mouse::MouseButton::Left => INPUT.mouse.left = false,
                            sdl2::mouse::MouseButton::Right => INPUT.mouse.right = false,
                            _ => {}
                        }
                    },

                    sdl2::event::Event::MouseWheel { y, .. } => unsafe {
                        INPUT.mouse.wheel = y as f32;
                    },

                    //
                    // Window
                    //
                    sdl2::event::Event::Quit { .. } => break 'main,
                    sdl2::event::Event::Window { win_event, .. } => match win_event {
                        sdl2::event::WindowEvent::Resized(w, h) => {
                            gfx_resize(w, h);
                            update_screen(IVec2::new(w, h));
                        }
                        _ => {}
                    },

                    //
                    //
                    //
                    _ => {}
                }
            }
        }

        imgui_sdl2.prepare_frame(imgui.io_mut(), &window, &event_pump.mouse_state());
        let ui = imgui.frame();

        //
        // Update
        //
        {
            camera.update(&ui);
        }

        //
        // Render
        //
        {
            gfx_clear();
            loaded_scene.draw_this(&mut shader, &mut camera);

            imgui_sdl2.prepare_render(&ui, &window);
            imgui_renderer.render(ui);

            window.gl_swap_window();
        }

        //
        // Timings
        //
        let mut delta = std::time::Instant::now()
            .duration_since(last_time)
            .as_millis() as f32;
        delta /= 1000.0;
        update_time(delta);

        last_time = std::time::Instant::now();
    }
}
