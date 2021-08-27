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
pub mod shader;

use camera::Camera;
use gfx::*;
use glam::*;
use model::Model;
use shader::Shader;

fn main() {
    let _sdl = sdl2::init().unwrap();
    let _video_subsystem = _sdl.video().unwrap();

    let mut _window = _video_subsystem
        .window("My Game", 1280, 720)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = _window.gl_create_context().unwrap();

    let _gl = gl::load_with(|s| _video_subsystem.gl_get_proc_address(s) as *const _);
    let _viewport =
        gl::Viewport::load_with(|s| _video_subsystem.gl_get_proc_address(s) as *const _);

    unsafe {
        let mut major = -1;
        gl::GetIntegerv(gl::MAJOR_VERSION, &mut major);

        let mut minor = -1;
        gl::GetIntegerv(gl::MINOR_VERSION, &mut minor);

        _window
            .set_title(format!("My Game, OpenGL {}.{}", major, minor).as_str())
            .unwrap();
    }

    gfx_setup();

    //
    // Scene setup
    //

    // Create camera
    let mut camera = Camera::new();

    // Create triangle
    let mut shader = Shader::new("content/shaders/standard.glsl");
    shader.scan_uniforms();

    let model = Model::new("content/models/monkey.obj");
    let mut event_pump = _sdl.event_pump().unwrap();

    let mut time: f32 = 0.0;
    let distance = 5.0;

    'main: loop {
        if !input_event_poll(&mut event_pump) {
            break 'main;
        }

        gfx_clear();

        // sine
        time = time + 0.005;
        let sin_time = (time).sin();
        let cos_time = (time).cos();

        let x = sin_time * distance;
        let y = cos_time * distance;

        camera.set_position_calc_view_proj_mat(Vec3::new(x, y, 0.0));

        model.draw_this(&mut shader, &mut camera);

        _window.gl_swap_window();
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
