// ============================================================================
//
// main.rs
//
// Purpose: The main entry point for the application.
//
// ============================================================================

extern crate gl;
extern crate sdl2;

pub mod screen;
pub mod time;
pub mod transform;

pub mod gfx;
pub mod mesh;
pub mod shader;

pub mod camera;
pub mod model;
pub mod scene;

pub mod input;
pub mod lerp;

pub mod color;

use camera::Camera;
use gfx::*;
use glam::*;
use scene::Scene;
use screen::*;
use shader::Shader;
use time::*;

use crate::input::INPUT;

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

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
                    sdl2::event::Event::Quit { .. } => break 'main,
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
                            sdl2::mouse::MouseButton::Unknown => {}
                            sdl2::mouse::MouseButton::Left => INPUT.mouse.left = true,
                            sdl2::mouse::MouseButton::Middle => {}
                            sdl2::mouse::MouseButton::Right => INPUT.mouse.right = true,
                            sdl2::mouse::MouseButton::X1 => {}
                            sdl2::mouse::MouseButton::X2 => {}
                        }
                    },
                    sdl2::event::Event::MouseButtonUp { mouse_btn, .. } => unsafe {
                        match mouse_btn {
                            sdl2::mouse::MouseButton::Unknown => {}
                            sdl2::mouse::MouseButton::Left => INPUT.mouse.left = false,
                            sdl2::mouse::MouseButton::Middle => {}
                            sdl2::mouse::MouseButton::Right => INPUT.mouse.right = false,
                            sdl2::mouse::MouseButton::X1 => {}
                            sdl2::mouse::MouseButton::X2 => {}
                        }
                    },

                    sdl2::event::Event::MouseWheel { y, .. } => unsafe {
                        INPUT.mouse.wheel = y as f32;
                    },

                    sdl2::event::Event::Window { win_event, .. } => match win_event {
                        sdl2::event::WindowEvent::Resized(w, h) => {
                            gfx_resize(w, h);
                        }
                        _ => {}
                    },
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
