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

use camera::Camera;
use gfx::*;
use glam::*;
use scene::Scene;
use screen::*;
use shader::Shader;
use time::*;

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
        // Update
        //
        {
            if !input_event_poll(&mut event_pump) {
                break 'main;
            }

            camera.update();
        }

        //
        // Render
        //
        {
            gfx_clear();
            loaded_scene.draw_this(&mut shader, &mut camera);
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

fn input_event_poll(event_pump: &mut sdl2::EventPump) -> bool {
    for _event in event_pump.poll_iter() {
        match _event {
            sdl2::event::Event::Quit { .. } => return false,
            _ => {}
        }
    }

    return true;
}
