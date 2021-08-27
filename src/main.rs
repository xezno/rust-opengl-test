// ============================================================================
//
// main.rs
//
// Purpose: The main entry point for the application.
//
// ============================================================================

extern crate gl;
extern crate sdl2;

pub mod camera;
pub mod gfx;
pub mod mesh;
pub mod model;
pub mod scene;
pub mod shader;
pub mod time;
pub mod transform;

use camera::Camera;
use gfx::*;
use glam::*;
use scene::Scene;
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

    unsafe {
        let mut major = -1;
        gl::GetIntegerv(gl::MAJOR_VERSION, &mut major);

        let mut minor = -1;
        gl::GetIntegerv(gl::MINOR_VERSION, &mut minor);

        window
            .set_title(format!("My Game, OpenGL {}.{}", major, minor).as_str())
            .unwrap();
    }

    gfx_setup();

    //
    // Scene setup
    //
    let scene = Scene::new("content/scene.json");
    let loaded_scene = scene.load();

    // Create camera
    let mut camera = Camera::new();

    // Create triangle
    let mut shader = Shader::new("content/shaders/standard.glsl");
    shader.scan_uniforms();

    // let model = Model::new("content/models/monkey.obj");
    let mut event_pump = sdl.event_pump().unwrap();

    'main: loop {
        //
        // Update
        //
        {
            if !input_event_poll(&mut event_pump) {
                break 'main;
            }

            // TODO: Proper time delta
            update_time(0.005);
            camera.update();
        }

        //
        // Render
        //
        {
            gfx_clear();
            // model.draw_this(&mut shader, &mut camera);
            loaded_scene.draw_this(&mut shader, &mut camera);
            window.gl_swap_window();
        }
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
