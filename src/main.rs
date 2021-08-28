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
use imgui::{im_str, Condition, Window};
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
        // Update
        //
        {
            for event in event_pump.poll_iter() {
                imgui_sdl2.handle_event(&mut imgui, &event);
                if imgui_sdl2.ignore_event(&event) {
                    continue;
                }

                match event {
                    sdl2::event::Event::Quit { .. } => break 'main,
                    _ => {}
                }
            }

            camera.update();
        }

        //
        // Render
        //
        {
            gfx_clear();
            loaded_scene.draw_this(&mut shader, &mut camera);

            imgui_sdl2.prepare_frame(imgui.io_mut(), &window, &event_pump.mouse_state());

            let ui = imgui.frame();

            Window::new(im_str!("Hello world"))
                .size([300.0, 110.0], Condition::FirstUseEver)
                .build(&ui, || {
                    ui.text(im_str!("Hello world!"));
                    ui.text(im_str!(":mitarejoice:"));
                    ui.separator();
                    let mouse_pos = ui.io().mouse_pos;
                    ui.text(format!(
                        "Mouse Position: ({:.1},{:.1})",
                        mouse_pos[0], mouse_pos[1]
                    ));
                });

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
